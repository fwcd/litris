use std::ops::Mul;

/// An 2D rotation into one of the four cardinal directions.
pub struct Rotation {
    /// The integer matrix representing the corresponding linear transformation.
    matrix: [i32; 4],
}

impl Rotation {
    /// The identity rotation.
    pub const IDENTITY: Self = Self::new([
         1,  0,
         0,  1,
    ]);
    /// The rotation by 90° clockwise.
    pub const CW_90: Self = Self::new([
         0, -1,
         1,  0,
    ]);
    /// The rotation by 180° clockwise or counter-clockwise.
    pub const CW_180: Self = Self::new([
        -1,  0,
         0, -1,
    ]);
    /// The rotation by 270° clockwise (or 90° counter-clockwise).
    pub const CW_270: Self = Self::new([
         0,  1,
        -1,  0,
    ]);

    /// Creates a new rotation from the given matrix.
    pub const fn new(matrix: [i32; 4]) -> Self {
        Self { matrix }
    }
}
