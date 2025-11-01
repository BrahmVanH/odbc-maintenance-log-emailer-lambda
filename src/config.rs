use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub dropbox: DropboxConfig,
    pub lettre: LettreConfig,
    pub allow_origins: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DropboxConfig {
    pub refresh_token: String,
    pub app_key: String,
    pub app_secret: String,
    pub shared_link: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LettreConfig {
    pub smtp_username: String,
    pub smtp_password: String,
    pub to: String,
    pub reply_to: String,
    pub from: String,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::AppError> {
        envy::from_env().map_err(|e| {
            crate::AppError::ConfigError(format!("Failed to load config from environment: {}", e))
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dropbox: DropboxConfig {
                refresh_token: env::var("DROPBOX_REFRESH_TOKEN").unwrap_or_else(|_| "".to_string()),
                app_key: env::var("DROPBOX_APP_KEY").unwrap_or_else(|_| "".to_string()),
                app_secret: env::var("DROPBOX_APP_SECRET").unwrap_or_else(|_| "".to_string()),
                shared_link: env::var("DROPBOX_SHARED_LINK").unwrap_or_else(|_| "".to_string()),
            },
            lettre: LettreConfig {
                smtp_username: env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
                smtp_password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
                to: env::var("LETTRE_EMAIL_ADDRESS").unwrap_or_else(|_| "".to_string()),
                reply_to: env::var("LETTRE_EMAIL_ADDRESS").unwrap_or_else(|_| "".to_string()),
                from: env::var("LETTRE_EMAIL_ADDRESS").unwrap_or_else(|_| "".to_string()),
            },
            allow_origins: "".to_string(),
            log_level: "error".to_string(),
        }
    }
}
