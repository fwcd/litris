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

    /// Creates a random tetromino with the given rng.
    pub fn random_with(rng: &mut impl Rng) -> Self {
        let tetromino = *Tetromino::ALL.choose(rng).unwrap();
        let pos = rng.gen();
        let rotation = Rotation::random_cardinal_with(rng);
        Self::new(tetromino, pos, rotation)
    }

    /// Creates a random tetromino with the default thread-local rng.
    pub fn random() -> Self {
        Self::random_with(&mut thread_rng())
    }
}
