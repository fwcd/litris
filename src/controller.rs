use std::sync::Arc;

use anyhow::Result;
use futures::{Stream, lock::Mutex, StreamExt};
use lighthouse_client::protocol::{Direction, GamepadButtonEvent, GamepadControlEvent, GamepadEvent, InputEvent, KeyEvent, ServerMessage};

use crate::model::{State, Key};

pub async fn run<const W: usize, const H: usize>(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin,
    shared_state: Arc<Mutex<State<W, H>>>
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;

        let opt_key = match input_event.direction() {
            Some(Direction::Left) => Some(Key::Left),
            Some(Direction::Up) => Some(Key::Up),
            Some(Direction::Right) => Some(Key::Right),
            Some(Direction::Down) => Some(Key::Down),
            _ => None,
        };
        let opt_down = match input_event {
            InputEvent::Key(KeyEvent { down, .. }) => Some(down),
            InputEvent::Gamepad(GamepadEvent { control: GamepadControlEvent::Button(GamepadButtonEvent { down, .. }), .. }) => Some(down),
            _ => None,
        };

        if let Some(key) = opt_key {
            if let Some(down) = opt_down {
                let mut state = shared_state.lock().await;
                if down {
                    if key == Key::Up {
                        state.click(key);
                    } else {
                        state.press(key);
                    }
                } else {
                    state.release(key);
                }
            }
        }
    }
    Ok(())
}
