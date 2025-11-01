use tracing::{ info, error };


use crate::config::Config;

pub mod config;
pub mod error;

pub use error::{ AppError, AppResult };


#[tokio::main]
async fn main() {
    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .init();

    info!("starting up...");

    let config = Config::from_env().unwrap_or_else(|e| {
        error!("Failed to load configuration, using defaults: {}", e);
        Config::default()
    });
}
