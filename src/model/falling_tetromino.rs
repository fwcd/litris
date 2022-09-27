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

    /// Falls by one pixel.
    pub fn fall(&mut self) {
        self.pos += Delta::DOWN;
    }

    /// Moves by the given delta.
    pub fn move_by(&mut self, delta: Delta) {
        self.pos += delta;
    }

    /// Rotates by the given delta.
    pub fn rotate_by(&mut self, rotation: Rotation) {
        self.rotation = rotation * self.rotation;
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
