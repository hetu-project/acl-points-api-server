mod auth;
mod health;
mod message;
pub mod middlewares;
mod router;
mod user;
mod webset;

use crate::{app::SharedState, common::error::AppResult};
use router::app_router;

pub async fn http_server_start(state: SharedState) -> AppResult<()> {
    let router = app_router(state.clone());

    let addr = format!("{}:{}", state.config.server.host, state.config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("server is running on {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
