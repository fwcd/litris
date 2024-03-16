use std::collections::HashSet;

use lighthouse_client::protocol::{Frame, Pos, Color, Delta, Rotation};

use super::{Board, Key};

/// The state of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State<const W: usize, const H: usize> {
    /// The game board.
    board: Board<W, H>,
    /// Pressed keys.
    keys: HashSet<Key>, // TODO: Use a bit set?
}

impl<const W: usize, const H: usize> State<W, H> {
    /// Creates an empty game.
    pub fn new() -> Self {
        State {
            board: Board::new(),
            keys: HashSet::new(),
        }
    }

    /// Resets the game.
    pub fn reset(&mut self) {
        self.board = Board::new();
    }

    /// Renders the state to the given frame.
    pub fn render_to(&self, frame: &mut Frame) {
        for y in 0..H {
            for x in 0..W {
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

    /// Presses a key.
    pub fn press(&mut self, key: Key) {
        self.keys.insert(key);
    }

    /// Releases a key.
    pub fn release(&mut self, key: Key) {
        self.keys.remove(&key);
    }

    fn handle(key: Key, board: &mut Board<W, H>) {
        if !board.game_over() {
            match key {
                Key::Up => board.rotate_falling(Rotation::CW_90),
                Key::Left => board.move_falling(Delta::LEFT),
                Key::Right => board.move_falling(Delta::RIGHT),
                Key::Down => board.fall(),
            }
        }
    }

    /// Clicks a key.
    pub fn click(&mut self, key: Key) {
        Self::handle(key, &mut self.board);
    }

    /// Performs an input tick, i.e. updates held keys.
    pub fn input_tick(&mut self) {
        for key in &self.keys {
            Self::handle(*key, &mut self.board);
        }
    }

    /// Performs a game tick.
    pub fn tick(&mut self) {
        if self.board.game_over() {
            self.reset();
        } else {
            self.board.fall();
        }
    }
}
