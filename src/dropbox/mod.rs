use opendal::{ services::Dropbox, Operator };

use crate::{ config::DropboxConfig, dropbox::parse::{extract_maintenance_tasks, get_next_weeks_tasks, parse_xlsx_file}, AppError };
use tracing::{ info, error };

pub mod parse;

pub async fn create_dbx_operator(dbx_config: &DropboxConfig) -> Result<Operator, AppError> {
    let DropboxConfig { refresh_token, app_key, app_secret, shared_link: _ } = dbx_config;
    if refresh_token.is_empty() {
        return Err(AppError::InternalServerError("Refresh token not provided".to_string()));
    }
    let dropbox = Dropbox::default()
        .refresh_token(&refresh_token)
        .client_id(&app_key)
        .client_secret(&app_secret);

    let op = Operator::new(dropbox)
        .map_err(|e| {
            AppError::InternalServerError(format!("Error in building dbx operator: {}", e))
        })?
        .finish();

    Ok(op)
}

pub async fn get_xlsx_from_dbx(op: &Operator) -> Result<Vec<u8>, AppError> {
    let file_buffer = op
        .read("ODBC shared/Maintenance/Maintenance Log.xlsx").await
        .map_err(|e|
            AppError::InternalServerError(format!("Failed to read file from Dropbox: {}", e))
        )?
        .to_vec();
    Ok(file_buffer)
}
