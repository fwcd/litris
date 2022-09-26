use lighthouse_client::LIGHTHOUSE_SIZE;

use super::Board;

/// The state of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    /// The game board.
    board: Board<LIGHTHOUSE_SIZE>,
}

impl State {
    /// Creates an empty game.
    pub fn new() -> Self {
        State {
            board: Board::new(),
        }
    }
}
