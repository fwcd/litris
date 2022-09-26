use lighthouse_client::LIGHTHOUSE_SIZE;

use super::Board;

/// The state of a game.
pub struct State {
    /// The game board.
    board: Board<LIGHTHOUSE_SIZE>,
}
