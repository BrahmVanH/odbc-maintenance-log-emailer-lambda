use std::env;

use lambda_runtime::{ service_fn, Error, LambdaEvent };
use serde_json::Value;
use dotenvy::dotenv;
use tracing::info;
use maintenance_sheet_emailer::{ config::Config, dropbox::create_dbx_operator, handle_send_email };

async fn function_handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    dotenv().ok();

    let dsn = env::var("SENTRY_DSN").unwrap_or_else(|_| "".to_string());
    let _guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            ..Default::default()
        },
    ));

    info!("starting up...");

    let config = Config::default();

    let dbx = create_dbx_operator(&config.dropbox).await.map_err(|e|
        format!("Dropbox error: {}", e)
    )?;

    handle_send_email(&dbx, &config.lettre).await.map_err(|e| format!("Email error: {}", e))?;

    Ok(serde_json::json!({"statusCode": 200, "body": "Email sent successfully"}))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    lambda_runtime::run(service_fn(function_handler)).await
}
