mod commands;
mod config;
mod consts;
mod error;
mod handlers;
mod jwt;
mod server;
mod storage;

use logging;

#[tokio::main]
async fn main() {
    logging::logging_init(consts::LOG_PATH).unwrap();

    commands::run_command().await;
}
