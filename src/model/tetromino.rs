use lighthouse_client::{Pos, Color};

/// A game piece composed of four pixels connected orthogonally.
pub struct Tetromino {
    /// The relative positions of the pixels.
    pixels: [Pos; 4],
    /// The color of the piece.
    color: Color,
}

impl Tetromino {
    /// The straight (horizontal) tetromino. Horizontally and vertically symmetric.
    pub const STRAIGHT: Self = Self::new([
        Pos::new(-1, 0),
        Pos::new(0, 0),
        Pos::new(1, 0),
        Pos::new(2, 0),
    ], Color::CYAN);
    /// The 4x4 square tetromino. Horizontally and vertically symmetric.
    pub const SQUARE: Self = Self::new([
        Pos::new(0, 0),
        Pos::new(1, 0),
        Pos::new(0, 1),
        Pos::new(1, 1),
    ], Color::YELLOW);
    /// The T-tetromino. Symmetric along the vertical axis.
    pub const T: Self = Self::new([
        Pos::new(-1, 0),
        Pos::new(0, 0),
        Pos::new(1, 0),
        Pos::new(1, 1),
    ], Color::MAGENTA);
    /// The L-tetromino.
    pub const L: Self = Self::new([
        Pos::new(0, -1),
        Pos::new(0, 0),
        Pos::new(0, 1),
        Pos::new(1, 1),
    ], Color::MAGENTA);
    /// The skew tetromino.
    pub const SKEW: Self = Self::new([
        Pos::new(-1, 1),
        Pos::new(0, 1),
        Pos::new(0, 0),
        Pos::new(1, 0),
    ], Color::MAGENTA);
    /// All free tetrominos.
    pub const ALL: [Self; 5] = [Self::STRAIGHT, Self::SQUARE, Self::T, Self::L, Self::SKEW];

    /// Creates a new tetromino from the given configuration.
    pub const fn new(pixels: [Pos; 4], color: Color) -> Self {
        Self { pixels, color }
    }
}
