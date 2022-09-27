use lighthouse_client::{Pos, Rotation, Delta, Color};

use super::Tetromino;

/// A positioned, rotated tetromino.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallingTetromino {
    /// The shape of the tetromino.
    tetromino: Tetromino,
    /// The position on the board.
    pos: Pos,
    /// The rotation.
    rotation: Rotation,
    /// The color of the tetromino.
    color: Color,
}

impl FallingTetromino {
    /// Creates a new falling tetromino with the given configuration.
    pub const fn new(tetromino: Tetromino, pos: Pos, rotation: Rotation, color: Color) -> Self {
        Self { tetromino, pos, rotation, color }
    }

    /// The tetromino after falling one pixel down.
    pub fn fallen(mut self) -> Self {
        self.pos += Delta::DOWN;
        self
    }

    /// The tetromino moved by the given delta.
    pub fn moved_by(mut self, delta: Delta) -> Self {
        self.pos += delta;
        self
    }

    /// Rotates by the given delta.
    pub fn rotated_by(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation * self.rotation;
        self
    }

    /// The color of the tetromino.
    pub fn color(&self) -> Color {
        self.color
    }

    /// The positions occupied by this falling tetromino.
    pub fn pixels(&self) -> [Pos; 4] {
        self.tetromino.pixels.map(|d| self.pos + self.rotation * d)
    }

    /// Whether this falling tetromino contains the given position.
    pub fn contains(&self, pos: Pos) -> bool {
        self.pixels().contains(&pos)
    }
}
