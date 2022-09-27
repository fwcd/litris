use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, TokioWebSocket, Result};
use tokio::time;

use crate::model::State;

pub async fn run(mut lh: Lighthouse<TokioWebSocket>, shared_state: Arc<Mutex<State>>) -> Result<()> {
    loop {
        let frame = {
            let mut state = shared_state.lock().await;
            state.tick();
            state.render()
        };

        lh.put_model(frame).await?;

        time::sleep(Duration::from_secs(1)).await;
    }
}
