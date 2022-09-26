mod controller;
mod model;
mod updater;

use std::env;

use lighthouse_client::{Authentication, Lighthouse, Result};
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
    
    let mut lh = Lighthouse::connect_with_tokio(auth).await?;
    let stream = lh.stream_model().await?;

    // TODO

    Ok(())
}
