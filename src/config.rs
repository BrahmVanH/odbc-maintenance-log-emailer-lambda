use serde::Deserialize;
use std::{ env, collections::HashMap };

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
    pub to: LettreAddress,
    pub reply_to: LettreAddress,
    pub from: LettreAddress,
    pub subject: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LettreAddress {
    pub name: String,
    pub email: String,
}

impl Config {

    /// Safe way to create config with SSM parameters
    pub fn from_env_and_ssm(ssm_params: HashMap<String, String>) -> Self {
        let email_address = env::var("LETTRE_EMAIL_ADDRESS").unwrap_or_else(|_| "".to_string());

        Self {
            dropbox: DropboxConfig {
                refresh_token: ssm_params
                    .get("DBX_OAUTH_TOKEN")
                    .cloned()
                    .unwrap_or_else(||
                        env::var("DBX_OAUTH_TOKEN").unwrap_or_else(|_| "".to_string())
                    ),
                app_key: env::var("DBX_APP_KEY").unwrap_or_else(|_| "".to_string()),
                app_secret: ssm_params
                    .get("DBX_APP_SECRET")
                    .cloned()
                    .unwrap_or_else(||
                        env::var("DBX_APP_SECRET").unwrap_or_else(|_| "".to_string())
                    ),
                shared_link: env::var("DBX_SHARED_LINK").unwrap_or_else(|_| "".to_string()),
            },
            lettre: LettreConfig {
                smtp_username: env::var("SMTP_ADDRESS").unwrap_or_else(|_| "".to_string()),
                smtp_password: ssm_params
                    .get("SMTP_PASSWORD")
                    .cloned()
                    .unwrap_or_else(||
                        env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string())
                    ),
                to: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                reply_to: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                from: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                subject: "Maintenance Tasks - Lambda".to_string(),
            },
            allow_origins: "".to_string(),
            log_level: "error".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let email_address = env::var("LETTRE_EMAIL_ADDRESS").unwrap_or_else(|_| "".to_string());
        Self {
            dropbox: DropboxConfig {
                refresh_token: env::var("DBX_OAUTH_TOKEN").unwrap_or_else(|_| "".to_string()),
                app_key: env::var("DBX_APP_KEY").unwrap_or_else(|_| "".to_string()),
                app_secret: env::var("DBX_APP_SECRET").unwrap_or_else(|_| "".to_string()),
                shared_link: env::var("DBX_SHARED_LINK").unwrap_or_else(|_| "".to_string()),
            },
            lettre: LettreConfig {
                smtp_username: env::var("SMTP_ADDRESS").unwrap_or_else(|_| "".to_string()),
                smtp_password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
                to: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                reply_to: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                from: LettreAddress {
                    name: "Brahm".to_string(),
                    email: email_address.clone(),
                },
                subject: "Maintenance Tasks".to_string(),
            },
            allow_origins: "".to_string(),
            log_level: "error".to_string(),
        }
    }
}
