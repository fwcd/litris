use lighthouse_client::{Frame, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Pos, Color, Delta};

use super::Board;

/// The state of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    /// The game board.
    board: Board<LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS>,
}

impl State {
    /// Creates an empty game.
    pub fn new() -> Self {
        State {
            board: Board::new(),
        }
    }

    /// Renders the state to the given frame.
    pub fn render_to(&self, frame: &mut Frame) {
        for y in 0..LIGHTHOUSE_ROWS {
            for x in 0..LIGHTHOUSE_COLS {
                let pos = Pos::new(x as i32, y as i32);
                frame[pos] = self.board.get(pos).unwrap_or(Color::BLACK);
            }
        }
    }

    /// Renders the state to a frame.
    pub fn render(&self) -> Frame {
        let mut frame = Frame::empty();
        self.render_to(&mut frame);
        frame
    }

    /// Moves the falling tetromino.
    pub fn move_falling(&mut self, delta: Delta) {
        self.board.move_falling(delta)
    }

    /// Performs a game tick.
    pub fn tick(&mut self) {
        self.board.tick();
    }
}
