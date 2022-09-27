mod controller;
mod model;
mod renderer;
mod ticker;

use std::{env, sync::Arc};

use futures::lock::Mutex;
use lighthouse_client::{Authentication, Lighthouse};
use model::State;
use tokio::task;
use tracing_subscriber::EnvFilter;

fn env_var(var: &str) -> String {
    match env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("Please make sure to set the {} environment variable: {:?}", var, e),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let username = env_var("LIGHTHOUSE_USER");
    let token = env_var("LIGHTHOUSE_TOKEN");
    let auth = Authentication::new(&username, &token);
    let state = Arc::new(Mutex::new(State::new()));
    
    let mut lh = Lighthouse::connect_with_tokio(auth).await.unwrap();
    let stream = lh.stream_model().await.unwrap();

    let renderer_handle = task::spawn(renderer::run(lh, state.clone()));
    let ticker_handle = task::spawn(ticker::run(state.clone()));
    let controller_handle = task::spawn(controller::run(stream, state));

    renderer_handle.await.unwrap().unwrap();
    ticker_handle.await.unwrap();
    controller_handle.await.unwrap();
}
