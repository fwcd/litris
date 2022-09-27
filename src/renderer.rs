use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, TokioWebSocket, Result};
use tokio::time;
use tracing::debug;

use crate::model::State;

pub async fn run(mut lh: Lighthouse<TokioWebSocket>, shared_state: Arc<Mutex<State>>) -> Result<()> {
    loop {
        debug!("Rendering");

        let frame = {
            let state = shared_state.lock().await;
            state.render()
        };

        lh.put_model(frame).await?;

        time::sleep(Duration::from_millis(100)).await;
    }
}
