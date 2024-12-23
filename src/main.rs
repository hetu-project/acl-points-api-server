use app_service::commands;
use logging;

pub const LOG_PATH: &str = "logs";

#[tokio::main]
async fn main() {
    logging::logging_init(LOG_PATH).unwrap();

    commands::run_command().await;
}
