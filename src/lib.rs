pub mod config;
pub mod error;
pub mod models;
pub mod dropbox;
pub mod mailer;
pub mod ssm;

pub use error::{ AppError, AppResult };
pub use chrono::{ Duration, NaiveDate, NaiveDateTime, TimeDelta };

use opendal::Operator;

use crate::{
    config::LettreConfig,
    dropbox::{
        get_xlsx_from_dbx,
        parse::{
            extract_maintenance_tasks,
            get_next_months_tasks,
            get_next_weeks_tasks,
            parse_xlsx_file_async,
        },
    },
    mailer::{ email_body::format_email_text, send_email },
};

pub async fn handle_send_email(
    dbx: &Operator,
    lettre_config: &LettreConfig
) -> Result<(), AppError> {
    let xlsx_buffer = get_xlsx_from_dbx(dbx).await?;

    let rows = parse_xlsx_file_async(xlsx_buffer).await?;

    let maintenance_tasks = extract_maintenance_tasks(rows);

    let next_weeks_tasks = get_next_weeks_tasks(&maintenance_tasks);
    let next_months_tasks = get_next_months_tasks(&maintenance_tasks);

    let email_body = format_email_text(next_weeks_tasks, next_months_tasks);

    send_email(&email_body, lettre_config)?;

    Ok(())
}
