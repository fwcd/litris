use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, TokioWebSocket, Result};
use tokio::time;
use tracing::debug;

use crate::model::State;

pub async fn run<const W: usize, const H: usize>(
    lh: Lighthouse<TokioWebSocket>,
    shared_state: Arc<Mutex<State<W, H>>>
) -> Result<()> {
    loop {
        debug!("Rendering");

        let frame = {
            let mut state = shared_state.lock().await;
            state.input_tick();
            state.render()
        };

        lh.put_model(frame).await?;

        time::sleep(Duration::from_millis(32)).await;
    }
}
