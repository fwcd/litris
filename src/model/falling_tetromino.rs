use lighthouse_client::protocol::{Pos, Rotation, Delta, Color};

use super::Tetromino;

/// A positioned, rotated tetromino.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallingTetromino {
    /// The shape of the tetromino.
    tetromino: Tetromino,
    /// The position on the board.
    pos: Pos<i32>,
    /// The rotation.
    rotation: Rotation<i32>,
    /// The color of the tetromino.
    color: Color,
}

impl FallingTetromino {
    /// Creates a new falling tetromino with the given configuration.
    pub const fn new(tetromino: Tetromino, pos: Pos<i32>, rotation: Rotation<i32>, color: Color) -> Self {
        Self { tetromino, pos, rotation, color }
    }

    /// The tetromino after falling one pixel down.
    pub fn fallen(mut self) -> Self {
        self.pos += Delta::DOWN;
        self
    }

    /// The tetromino moved by the given delta.
    pub fn moved_by(mut self, delta: Delta<i32>) -> Self {
        self.pos += delta;
        self
    }

    /// Rotates by the given delta.
    pub fn rotated_by(mut self, rotation: Rotation<i32>) -> Self {
        self.rotation = rotation * self.rotation;
        self
    }

    /// The color of the tetromino.
    pub fn color(&self) -> Color {
        self.color
    }

    /// The positions occupied by this falling tetromino.
    pub fn pixels(&self) -> [Pos<i32>; 4] {
        self.tetromino.pixels.map(|d| self.pos + self.rotation * d)
    }

    /// Whether this falling tetromino contains the given position.
    pub fn contains(&self, pos: Pos<i32>) -> bool {
        self.pixels().contains(&pos)
    }
}
