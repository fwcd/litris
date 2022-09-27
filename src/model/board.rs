use std::collections::HashSet;

use lighthouse_client::{Color, Pos, Rotation, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Rect, Delta};
use rand::{thread_rng, seq::SliceRandom};

use super::{FallingTetromino, Tetromino};

/// A game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [[Option<Color>; WIDTH]; HEIGHT],
    /// The in-flight tetromino.
    falling: FallingTetromino,
    /// Whether the game is over.
    game_over: bool,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    /// Creates an empty board with a random falling tetromino.
    pub fn new() -> Self {
        Board {
            fields: [[None; WIDTH]; HEIGHT],
            falling: Self::new_falling_tetromino(),
            game_over: false,
        }
    }

    /// Creates a new falling tetromino.
    fn new_falling_tetromino() -> FallingTetromino {
        let mut rng = thread_rng();
        let tetromino = Tetromino::random_with(&mut rng);
        let pos = Pos::new(WIDTH as i32 / 2, 1);
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
        self.with_place_check(|board| {
            board.falling.move_by(delta);
        });
    }

    /// Rotates the falling tetromino.
    pub fn rotate_falling(&mut self, rotation: Rotation) {
        self.with_place_check(|board| {
            board.falling.rotate_by(rotation);
        });
    }

    /// The bounding rectangle.
    pub fn bounds(&self) -> Rect {
        Rect::new(Pos::ZERO, Delta::new(WIDTH as i32, HEIGHT as i32))
    }

    /// The occupied pixels.
    pub fn occupied_pixels(&self) -> HashSet<Pos> {
        (0..LIGHTHOUSE_ROWS as i32)
            .flat_map(|y| (0..LIGHTHOUSE_COLS as i32).map(move |x| Pos::new(x, y)))
            .filter(|&p| self.get_field(p).is_some())
            .collect()
    }

    fn place_falling(&mut self) {
        for pos in self.falling.pixels() {
            self.fields[pos.y as usize][pos.x as usize] = Some(self.falling.color());
        }
    }

    fn falls_freely(&self) -> bool {
        let bounds = self.bounds();
        let falling_pixels = self.falling.pixels();

        falling_pixels.into_iter().all(|p| bounds.contains(p))
        && self.occupied_pixels().is_disjoint(&falling_pixels.into_iter().collect())
    }

    fn with_place_check(&mut self, action: impl FnOnce(&mut Self)) {
        let mut next = *self;
        action(&mut next);
        if next.falls_freely() {
            *self = next;
        } else {
            self.place_falling();
            self.falling = Self::new_falling_tetromino();
            if !self.falls_freely() {
                self.game_over = true;
            }
        }
    }

    /// Whether the game is over.
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Lets the tetromino fall.
    pub fn fall(&mut self) {
        self.with_place_check(|board| {
            board.falling.fall();
        });
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
