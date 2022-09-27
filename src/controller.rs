use std::sync::Arc;

use futures::{Stream, lock::Mutex, StreamExt};
use lighthouse_client::{ServerMessage, Payload, Delta};

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = ServerMessage> + Unpin, shared_state: Arc<Mutex<State>>) {
    while let Some(msg) = stream.next().await {
        if let Payload::InputEvent(event) = msg.payload {
            if event.is_down {
                let mut state = shared_state.lock().await;

                match event.key {
                    Some(37) => state.move_falling(Delta::LEFT),
                    Some(39) => state.move_falling(Delta::RIGHT),
                    Some(40) => state.move_falling(Delta::DOWN),
                    _ => {},
                };
            }
        }
    }
}
