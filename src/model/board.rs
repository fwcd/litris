use lighthouse_client::{Color, Pos, Rotation, Delta};

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
        let pos = Pos::new(WIDTH as i32 / 2, 0);
        let rotation = Rotation::IDENTITY;
        FallingTetromino::new(tetromino, pos, rotation)
    }

    /// Performs a game tick.
    pub fn tick(&mut self) {
        self.falling.fall();
    }

    /// Moves the falling tetromino.
    pub fn move_falling(&mut self, delta: Delta) {
        self.falling.move_by(delta);
    }

    /// Rotates the falling tetromino.
    pub fn rotate_falling(&mut self, rotation: Rotation) {
        self.falling.rotate_by(rotation);
    }

    /// Fetches the color at the given position.
    pub fn get(&self, pos: Pos) -> Option<Color> {
        if self.falling.contains(pos) {
            Some(self.falling.tetromino().color)
        } else {
            self.fields[pos.y as usize][pos.x as usize]
        }
    }
}
