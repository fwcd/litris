use std::collections::HashSet;

use lighthouse_client::{Color, Pos, Rotation, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Rect, Delta};

use super::{FallingTetromino, Tetromino};

/// A game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [[Option<Color>; WIDTH]; HEIGHT],
    /// The in-flight tetromino.
    falling: FallingTetromino,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    /// Creates an empty board with a random falling tetromino.
    pub fn new() -> Self {
        Board {
            fields: [[None; WIDTH]; HEIGHT],
            falling: Self::new_falling_tetromino(),
        }
    }

    /// Creates a new falling tetromino.
    fn new_falling_tetromino() -> FallingTetromino {
        let tetromino = Tetromino::random();
        let pos = Pos::new(WIDTH as i32 / 2, 1);
        let rotation = Rotation::IDENTITY;
        FallingTetromino::new(tetromino, pos, rotation)
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
            self.fields[pos.y as usize][pos.x as usize] = Some(self.falling.tetromino().color);
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
        }
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
            Some(self.falling.tetromino().color)
        } else {
            self.get_field(pos)
        }
    }
}
