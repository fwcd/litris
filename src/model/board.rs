use lighthouse_client::Color;

use super::FallingTetromino;

/// A game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<const SIZE: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [Option<Color>; SIZE],
    /// The in-flight tetromino.
    falling: FallingTetromino,
}

impl<const SIZE: usize> Board<SIZE> {
    /// Creates an empty board with a random falling tetromino.
    pub fn new() -> Self {
        Board {
            fields: [None; SIZE],
            falling: FallingTetromino::random(),
        }
    }
}
