use std::env;

// use sentry::ClientInitGuard;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")] DatabaseError(String),

    #[error("Validation error: {0}")] ValidationError(String),

    #[error("Not found: {0}")] NotFound(String),

    #[error("Unauthorized: {0}")] Unauthorized(String),

    #[error("Forbidden: {0}")] Forbidden(String),

    #[error("Internal server error: {0}")] InternalServerError(String),

    #[error("Configuration error: {0}")] ConfigError(String),

    #[error("Authentication error: {0}")] AuthError(String),
}

pub type AppResult<T> = Result<T, AppError>;

// pub fn initiate_sentry() -> ClientInitGuard {
//     let dsn = env::var("SENTRY_DSN").unwrap_or_else(|_| "".to_string());
//     sentry::init((
//         dsn,
//         sentry::ClientOptions {
//             release: sentry::release_name!(),
//             send_default_pii: true,
//             ..Default::default()
//         },
//     ))
// }
