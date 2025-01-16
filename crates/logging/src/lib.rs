use chrono::Local;
use std::fs;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

pub const LOG_TIME_FORMAT: &str = "%Y-%m-%d_%H-%M-%S";
pub const LOG_KEY_ENV: &str = "RUST_LOG";
pub const LOG_DEFAULT_LEVEL: &str = "info";
pub const LOG_BASE_NAME: &str = "app";

pub fn logging_init(log_dir: &str) -> Result<(), String> {
    let log_file = format!(
        "{}_{}.log",
        Local::now().format(LOG_TIME_FORMAT),
        LOG_BASE_NAME
    );

    // Create a rolling file appender that does not rotate automatically.
    let file_appender = RollingFileAppender::new(Rotation::NEVER, log_dir, log_file);
    //let (file_writer, _guard) = non_blocking(file_appender);
    let file_writer = file_appender;

    // Ensure the log directory exists, create if necessary.
    fs::create_dir_all(log_dir).map_err(|e| e.to_string())?;

    // Define a logging layer for writing to log files with timestamps and line numbers.
    let file_layer = fmt::Layer::default()
        .with_writer(file_writer)
        .with_line_number(true)
        .with_ansi(false); // Disable ANSI colors for log files.

    // Define a logging layer for console output with timestamps and line numbers.
    let stdout_layer = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_line_number(true);

    // Get the logging level from the environment or use the default.
    let rust_log = std::env::var(LOG_KEY_ENV).unwrap_or_else(|_| LOG_DEFAULT_LEVEL.to_string());

    // Create a tracing subscriber with environment-based filtering and layered output.
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new(rust_log))
        .with(stdout_layer)
        .with(file_layer);

    // Set the global default subscriber for tracing.
    tracing::subscriber::set_global_default(subscriber).map_err(|e| e.to_string())?;

    Ok(())
}
