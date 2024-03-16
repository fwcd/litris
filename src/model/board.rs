use std::{collections::HashSet, iter::repeat};

use arrayvec::ArrayVec;
use itertools::Itertools;
use lighthouse_client::protocol::{Color, Pos, Rotation, Rect, Delta};
use rand::{thread_rng, seq::SliceRandom};
use tracing::info;

use super::{FallingTetromino, Tetromino};

/// A game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<const W: usize, const H: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [[Option<Color>; W]; H],
    /// The in-flight tetromino.
    falling: FallingTetromino,
    /// The current score.
    score: usize,
    /// Whether the game is over.
    game_over: bool,
}

impl<const W: usize, const H: usize> Board<W, H> {
    /// Creates an empty board with a random falling tetromino.
    pub fn new() -> Self {
        Board {
            fields: [[None; W]; H],
            falling: Self::new_falling_tetromino(),
            score: 0,
            game_over: false,
        }
    }

    /// Creates a new falling tetromino.
    fn new_falling_tetromino() -> FallingTetromino {
        let mut rng = thread_rng();
        let tetromino = Tetromino::random_with(&mut rng);
        let pos = Pos::new(W as i32 / 2, 1);
        let rotation = Rotation::IDENTITY;
        let color = *[
            Color::MAGENTA,
            Color::YELLOW,
            Color::CYAN,
            Color::GREEN,
            Color::RED,
            Color::WHITE,
        ].choose(&mut rng).unwrap();
        FallingTetromino::new(tetromino, pos, rotation, color)
    }

    /// Moves the falling tetromino.
    pub fn move_falling(&mut self, delta: Delta) {
        let next = self.falling.moved_by(delta);
        if !self.collides_with(next) {
            self.falling = next;
        }
    }

    /// Rotates the falling tetromino.
    pub fn rotate_falling(&mut self, rotation: Rotation) {
        let next = self.falling.rotated_by(rotation);
        if !self.collides_with(next) {
            self.falling = next;
        }
    }

    /// Checks whether the given falling tetromino collides with the walls or placed fields.
    fn collides_with(&self, falling: FallingTetromino) -> bool {
        let bounds = self.bounds();
        let occupied = self.occupied_pixels();
        falling.pixels().into_iter().any(|p| !bounds.contains(p) || occupied.contains(&p))
    }

    /// The bounding rectangle.
    pub fn bounds(&self) -> Rect {
        Rect::new(Pos::ZERO, Delta::new(W as i32, H as i32))
    }

    /// The occupied pixels.
    pub fn occupied_pixels(&self) -> HashSet<Pos> {
        (0..H as i32)
            .flat_map(|y| (0..W as i32).map(move |x| Pos::new(x, y)))
            .filter(|&p| self.get_field(p).is_some())
            .collect()
    }

    fn clear_full_rows(&mut self) -> usize {
        let cleared = self.fields.into_iter()
            .filter(|row| row.iter().any(|c| c.is_none()))
            .collect::<ArrayVec<_, H>>();
        let cleared_count = H - cleared.len();
        self.fields.iter_mut().set_from(
            repeat([None; W])
                .take(cleared_count)
                .chain(cleared.into_iter())
        );
        cleared_count
    }

    fn place_falling(&mut self) {
        for pos in self.falling.pixels() {
            self.fields[pos.y as usize][pos.x as usize] = Some(self.falling.color());
        }

        self.falling = Self::new_falling_tetromino();
        let cleared_count = self.clear_full_rows();
        self.score += 40 * cleared_count;

        if self.collides_with(self.falling) {
            self.game_over = true;
            info!("Game over with score {}", self.score);
        }

        if cleared_count > 0 {
            info!("Cleared {} row(s), score: {}", cleared_count, self.score)
        }
    }

    /// Whether the game is over.
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Lets the tetromino fall.
    pub fn fall(&mut self) {
        let next = self.falling.fallen();
        if self.collides_with(next) {
            self.place_falling();
        } else {
            self.falling = next;
        }
    }

    /// Fetches the field at the given position.
    fn get_field(&self, pos: Pos) -> Option<Color> {
        self.fields[pos.y as usize][pos.x as usize]
    }

    /// Fetches the color at the given position.
    pub fn get(&self, pos: Pos) -> Option<Color> {
        if self.falling.contains(pos) {
            Some(self.falling.color())
        } else {
            self.get_field(pos)
        }
    }
}
