use std::ops::{Index, IndexMut};

use lighthouse_client::{Color, Pos};

use super::FallingTetromino;

/// A game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    /// The board's fields, containing placed tetrominos.
    fields: [[Option<Color>; WIDTH]; HEIGHT],
    /// The in-flight tetromino.
    falling: FallingTetromino,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    /// Creates an empty board with a random falling tetromino.
    pub fn new() -> Self {
        Board {
            fields: [[None; WIDTH]; HEIGHT],
            falling: FallingTetromino::random(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<Pos> for Board<WIDTH, HEIGHT> {
    type Output = Option<Color>;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.fields[pos.y as usize][pos.x as usize]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<Pos> for Board<WIDTH, HEIGHT> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.fields[pos.y as usize][pos.x as usize]
    }
}
