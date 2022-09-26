use lighthouse_client::{Pos, Rotation};
use rand::{seq::SliceRandom, thread_rng, Rng};

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

    // TODO: Add random_with for customizing the rng

    /// Creates a random tetromino.
    pub fn random() -> Self {
        let mut rng = &mut thread_rng();
        let tetromino = *Tetromino::ALL.choose(&mut rng).unwrap();
        let pos = rng.gen();
        let rotation = Rotation::random_cardinal();
        Self::new(tetromino, pos, rotation)
    }
}
