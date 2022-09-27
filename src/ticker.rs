use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;

use tokio::time;
use tracing::debug;

use crate::model::State;

pub async fn run(shared_state: Arc<Mutex<State>>) {
    loop {
        debug!("Ticking");

        {
            let mut state = shared_state.lock().await;
            state.tick();
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}
