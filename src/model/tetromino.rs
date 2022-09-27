use lighthouse_client::{Delta, Color};
use rand::{Rng, thread_rng, seq::SliceRandom};

/// A game piece composed of four pixels connected orthogonally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tetromino {
    /// The relative positions of the pixels.
    pub pixels: [Delta; 4],
    /// The color of the piece.
    pub color: Color,
}

impl Tetromino {
    /// The straight (horizontal) tetromino. Horizontally and vertically symmetric.
    pub const STRAIGHT: Self = Self::new([
        Delta::new(-1, 0),
        Delta::new(0, 0),
        Delta::new(1, 0),
        Delta::new(2, 0),
    ], Color::CYAN);
    /// The 4x4 square tetromino. Horizontally and vertically symmetric.
    pub const SQUARE: Self = Self::new([
        Delta::new(0, 0),
        Delta::new(1, 0),
        Delta::new(0, 1),
        Delta::new(1, 1),
    ], Color::YELLOW);
    /// The T-tetromino. Symmetric along the vertical axis.
    pub const T: Self = Self::new([
        Delta::new(-1, 0),
        Delta::new(0, 0),
        Delta::new(1, 0),
        Delta::new(1, 1),
    ], Color::MAGENTA);
    /// The L-tetromino.
    pub const L: Self = Self::new([
        Delta::new(0, -1),
        Delta::new(0, 0),
        Delta::new(0, 1),
        Delta::new(1, 1),
    ], Color::MAGENTA);
    /// The skew tetromino.
    pub const SKEW: Self = Self::new([
        Delta::new(-1, 1),
        Delta::new(0, 1),
        Delta::new(0, 0),
        Delta::new(1, 0),
    ], Color::MAGENTA);
    /// All free tetrominos.
    pub const ALL: [Self; 5] = [Self::STRAIGHT, Self::SQUARE, Self::T, Self::L, Self::SKEW];

    /// Creates a new tetromino from the given configuration.
    pub const fn new(pixels: [Delta; 4], color: Color) -> Self {
        Self { pixels, color }
    }

    /// Chooses a random tetromino with the given rng.
    pub fn random_with(rng: &mut impl Rng) -> Self {
        *Tetromino::ALL.choose(rng).unwrap()
    }

    /// Chooses a random tetromino with the default thread-local rng.
    pub fn random() -> Self {
        Self::random_with(&mut thread_rng())
    }
}
