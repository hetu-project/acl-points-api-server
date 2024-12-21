use app_service::commands;
use app_service::common::consts;
use logging;

#[tokio::main]
async fn main() {
    logging::logging_init(consts::LOG_PATH).unwrap();

    commands::run_command().await;
}
