use std::sync::Arc;

use futures::{Stream, lock::Mutex};
use lighthouse_client::ServerMessage;

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = ServerMessage> + Unpin, shared_state: Arc<Mutex<State>>) {
    // TODO
}
