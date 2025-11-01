use opendal::{ services::Dropbox, Builder, Operator };
use tracing::info;

use crate::{ config::{ Config, DropboxConfig }, AppError };

async fn setup_dbx_client(config: &Config) -> Result<(), AppError> {
    if let Some(dbx_config) = config.dropbox {
        info!("Setting up dbx operator with dbx config: {:?}", dbx_config);

        create_dbx_operator(dbx_config)?;

        Ok(())
    }
}

async fn create_dbx_operator(dbx_config: DropboxConfig) -> Result<Operator, AppError> {
    let DropboxConfig { refresh_token, app_key, app_secret, shared_link } = dbx_config;
    if refresh_token.is_empty() {
        return Err(AppError::InternalServerError("Refresh token not provided".to_string()));
    }
    let op = Operator::new(
        Dropbox::default()
            .refresh_token(&refresh_token)
            .client_id(&app_key)
            .client_secret(&app_secret)
            .build()
            .map_err(|e| {
                return Err(
                    AppError::InternalServerError(format!("Error in building dbx operator: {}", e))
                );
            })?
    ).map_err(|e| {
        return Err(AppError::InternalServerError(format!("Error in building dbx operator: {}", e)));
    })?;

    Ok(op)
}

async fn get_xlsx_from_dbx() {}
