use lighthouse_client::Color;

use super::FallingTetromino;

/// A game board.
pub struct Board<const SIZE: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [Option<Color>; SIZE],
    /// The in-flight tetromino.
    falling: FallingTetromino,
}
