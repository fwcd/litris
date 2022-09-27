use lighthouse_client::{Color, Pos, Rotation};

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

    /// Fetches the falling tetromino mutably.
    pub fn falling_mut(&mut self) -> &mut FallingTetromino {
        &mut self.falling
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
