use std::sync::Arc;

use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, TokioWebSocket};

use crate::model::State;

pub async fn run(mut lh: Lighthouse<TokioWebSocket>, shared_state: Arc<Mutex<State>>) {
    // TODO
}
