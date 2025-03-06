use std::sync::Arc;

use anyhow::Result;
use futures::{Stream, lock::Mutex, StreamExt};
use lighthouse_client::protocol::{Model, ServerMessage};

use crate::model::{State, Key};

pub async fn run<const W: usize, const H: usize>(
    mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<Model>>> + Unpin,
    shared_state: Arc<Mutex<State<W, H>>>
) -> Result<()> {
    while let Some(msg) = stream.next().await {
        if let Model::InputEvent(event) = msg?.payload {
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
    Ok(())
}
