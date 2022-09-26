use lighthouse_client::{Pos, Color};

/// A game piece composed of four pixels connected orthogonally.
pub struct Tetromino {
    /// The relative positions of the pixels.
    pixels: [Pos; 4],
    /// The color of the piece.
    color: Color,
}
