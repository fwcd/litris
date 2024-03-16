use std::sync::Arc;

use futures::{Stream, lock::Mutex, StreamExt};
use lighthouse_client::protocol::{ServerMessage, Payload};

use crate::model::{State, Key};

pub async fn run<const W: usize, const H: usize>(
    mut stream: impl Stream<Item = ServerMessage> + Unpin,
    shared_state: Arc<Mutex<State<W, H>>>
) {
    while let Some(msg) = stream.next().await {
        if let Payload::InputEvent(event) = msg.payload {
            let opt_key = match event.key {
                Some(37) => Some(Key::Left),
                Some(38) => Some(Key::Up),
                Some(39) => Some(Key::Right),
                Some(40) => Some(Key::Down),
                _ => None,
            };

            if let Some(key) = opt_key {
                let mut state = shared_state.lock().await;
                if event.is_down {
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
}
