use lighthouse_client::protocol::Delta;
use rand::{seq::IndexedRandom, Rng};

/// A game piece composed of four pixels connected orthogonally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tetromino {
    /// The relative positions of the pixels.
    pub pixels: [Delta<i32>; 4],
}

impl Tetromino {
    /// The straight (horizontal) tetromino. Horizontally and vertically symmetric.
    pub const STRAIGHT: Self = Self::new([
        Delta::new(-1, 0),
        Delta::new(0, 0),
        Delta::new(1, 0),
        Delta::new(2, 0),
    ]);
    /// The 4x4 square tetromino. Horizontally and vertically symmetric.
    pub const SQUARE: Self = Self::new([
        Delta::new(0, 0),
        Delta::new(1, 0),
        Delta::new(0, 1),
        Delta::new(1, 1),
    ]);
    /// The T-tetromino. Symmetric along the vertical axis.
    pub const T: Self = Self::new([
        Delta::new(-1, 0),
        Delta::new(0, 0),
        Delta::new(0, 1),
        Delta::new(1, 0),
    ]);
    /// The left L-tetromino.
    pub const LEFT_L: Self = Self::new([
        Delta::new(0, -1),
        Delta::new(0, 0),
        Delta::new(0, 1),
        Delta::new(1, 1),
    ]);
    /// The right L-tetromino.
    pub const RIGHT_L: Self = Self::new([
        Delta::new(0, -1),
        Delta::new(0, 0),
        Delta::new(0, 1),
        Delta::new(-1, 1),
    ]);
    /// The bottom skew tetromino.
    pub const BOTTOM_SKEW: Self = Self::new([
        Delta::new(-1, 1),
        Delta::new(0, 1),
        Delta::new(0, 0),
        Delta::new(1, 0),
    ]);
    /// The top skew tetromino.
    pub const TOP_SKEW: Self = Self::new([
        Delta::new(-1, 0),
        Delta::new(0, 0),
        Delta::new(0, 1),
        Delta::new(1, 1),
    ]);
    /// All seven standard tetrominos.
    pub const ALL: [Self; 7] = [
        Self::STRAIGHT, Self::SQUARE,
        Self::LEFT_L, Self::RIGHT_L,
        Self::T,
        Self::TOP_SKEW, Self::BOTTOM_SKEW,
    ];

    /// Creates a new tetromino from the given configuration.
    pub const fn new(pixels: [Delta<i32>; 4]) -> Self {
        Self { pixels }
    }

    /// Chooses a random tetromino with the given rng.
    pub fn random_with(rng: &mut impl Rng) -> Self {
        *Tetromino::ALL.choose(rng).unwrap()
    }
}
