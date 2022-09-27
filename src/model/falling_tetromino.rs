use lighthouse_client::{Pos, Rotation, Delta};

use super::Tetromino;

/// A positioned, rotated tetromino.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallingTetromino {
    /// The shape and color of the tetromino.
    tetromino: Tetromino,
    /// The position on the board.
    pos: Pos,
    /// The rotation.
    rotation: Rotation,
}

impl FallingTetromino {
    /// Creates a new falling tetromino with the given configuration.
    pub const fn new(tetromino: Tetromino, pos: Pos, rotation: Rotation) -> Self {
        Self { tetromino, pos, rotation }
    }

    /// Falls by one pixel.
    pub fn fall(&mut self) {
        self.pos += Delta::DOWN;
    }

    /// The tetromino after falling by one pixel.
    pub fn next(self) -> Self {
        let mut f = self;
        f.fall();
        f
    }

    /// Moves by the given delta.
    pub fn move_by(&mut self, delta: Delta) {
        self.pos += delta;
    }

    /// Rotates by the given delta.
    pub fn rotate_by(&mut self, rotation: Rotation) {
        self.rotation = rotation * self.rotation;
    }

    /// The underlying tetromino.
    pub fn tetromino(&self) -> Tetromino {
        self.tetromino
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
