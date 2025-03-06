mod controller;
mod model;
mod renderer;
mod ticker;

use std::sync::Arc;

use clap::Parser;
use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, LIGHTHOUSE_URL, protocol::{Authentication, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS}};
use model::State;
use tokio::task;
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();

    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let state = Arc::new(Mutex::new(State::<LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS>::new()));
    
    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await.unwrap();
    info!("Connected to the lighthouse.");

    let stream = lh.stream_model().await.unwrap();

    let renderer_handle = task::spawn(renderer::run(lh, state.clone()));
    let ticker_handle = task::spawn(ticker::run(state.clone()));
    let controller_handle = task::spawn(controller::run(stream, state));

    renderer_handle.await.unwrap().unwrap();
    ticker_handle.await.unwrap();
    controller_handle.await.unwrap().unwrap();
}
