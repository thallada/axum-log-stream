use anyhow::Result;
use axum::{routing::get, Router};
use bytes::Bytes;
use dotenvy::dotenv;
use tokio::sync::watch::channel;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::debug;

pub mod config;
pub mod handlers;
pub mod log;
pub mod partials;
pub mod state;

use crate::config::Config;
use crate::log::init_tracing;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = Config::new()?;

    let (log_sender, log_receiver) = channel::<Bytes>(Bytes::new());
    let _guards = init_tracing(&config, log_sender)?;

    let addr = format!("{}:{}", &config.host, &config.port).parse()?;
    let app = Router::new()
        .route("/", get(handlers::home::get))
        .route("/other", get(handlers::other::get))
        .route("/log", get(handlers::log::get))
        .route("/log/stream", get(handlers::log::stream))
        .with_state(AppState {
            config,
            log_receiver,
        })
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
