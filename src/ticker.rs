use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;

use tokio::time;
use tracing::debug;

use crate::model::State;

pub async fn run<const W: usize, const H: usize>(shared_state: Arc<Mutex<State<W, H>>>) {
    loop {
        debug!("Ticking");

        {
            let mut state = shared_state.lock().await;
            state.tick();
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}
