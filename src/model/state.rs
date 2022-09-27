use std::collections::HashSet;

use lighthouse_client::{Frame, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Pos, Color, Delta, Rotation};

use super::{Board, Key};

/// The state of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board<LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS>,
    /// Pressed keys.
    keys: HashSet<Key>, // TODO: Use a bit set?
}

impl State {
    /// Creates an empty game.
    pub fn new() -> Self {
        State {
            board: Board::new(),
            keys: HashSet::new(),
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

    /// Rotates the falling tetromino.
    pub fn rotate_falling(&mut self, rotation: Rotation) {
        self.board.rotate_falling(rotation)
    }

    /// Moves the falling tetromino.
    pub fn move_falling(&mut self, delta: Delta) {
        self.board.move_falling(delta)
    }

    /// Presses a key.
    pub fn press(&mut self, key: Key) {
        self.keys.insert(key);
    }

    /// Releases a key.
    pub fn release(&mut self, key: Key) {
        self.keys.remove(&key);
    }

    /// Performs an input tick, i.e. updates held keys.
    pub fn input_tick(&mut self) {
        for key in self.keys.clone() {
            match key {
                Key::Up => self.rotate_falling(Rotation::CW_90),
                Key::Left => self.move_falling(Delta::LEFT),
                Key::Right => self.move_falling(Delta::RIGHT),
                Key::Down => self.move_falling(Delta::DOWN),
            }
        }
    }

    /// Performs a game tick.
    pub fn tick(&mut self) {
        self.board.tick();
    }
}
