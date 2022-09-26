use lighthouse_client::{Pos, Rotation};

use super::Tetromino;

/// A positioned, rotated tetromino.
pub struct FallingTetromino {
    /// The shape and color of the tetromino.
    tetromino: Tetromino,
    /// The position on the board.
    pos: Pos,
    /// The rotation.
    rotation: Rotation,
}
