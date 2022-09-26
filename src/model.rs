use lighthouse_client::{Pos, Color, LIGHTHOUSE_SIZE, Rotation};

/// A game piece composed of four pixels connected orthogonally.
pub struct Tetromino {
    /// The relative positions of the pixels.
    pixels: [Pos; 4],
    /// The color of the piece.
    color: Color,
}

/// A positioned, rotated tetromino.
pub struct FallingTetromino {
    /// The shape and color of the tetromino.
    tetromino: Tetromino,
    /// The position on the board.
    pos: Pos,
    /// The rotation.
    rotation: Rotation,
}

/// A game board.
pub struct Board<const SIZE: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [Option<Color>; SIZE],
    /// The in-flight tetromino.
    tetromino: Tetromino,
}

/// The state of a game.
pub struct State {
    /// The game board.
    board: Board<LIGHTHOUSE_SIZE>,
}
