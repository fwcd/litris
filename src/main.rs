mod controller;
mod model;
mod updater;

use std::{env, sync::Arc};

use futures::lock::Mutex;
use lighthouse_client::{Authentication, Lighthouse, Result};
use model::State;
use tokio::task;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(&username, &token);
    let state = Arc::new(Mutex::new(State::new()));
    
    let mut lh = Lighthouse::connect_with_tokio(auth).await?;
    let stream = lh.stream_model().await?;

    task::spawn(updater::run(lh, state.clone()));
    task::spawn(controller::run(stream, state)).await;

    Ok(())
}
