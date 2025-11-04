use std::io::Cursor;
use chrono::{ DateTime, Utc };
use tracing::info;

use calamine::{ open_workbook_auto_from_rs, Data, Reader };

use crate::{
    mailer::email_body::get_time_difference_from_now,
    models::MaintenanceTask,
    AppError,
    NaiveDate,
    Duration,
    TimeDelta,
};

pub fn parse_xlsx_file(xlsx_buffer: Vec<u8>) -> Result<Vec<Vec<Data>>, AppError> {
    let cursor = Cursor::new(xlsx_buffer);

    let mut workbook = open_workbook_auto_from_rs(cursor).map_err(|e|
        AppError::InternalServerError(format!("Could not parse xlsx file: {}", e))
    )?;

    let range = workbook
        .worksheet_range("Maintenance Log")
        .map_err(|e|
            AppError::InternalServerError(format!("Could not access workbook range: {}", e))
        )?;
    let mut rows: Vec<Vec<Data>> = Vec::new();
    for row in range.rows() {
        rows.push(row.to_vec());
    }

    Ok(rows)
}

pub fn convert_excel_date_to_datetime(excel_serial_number: f64) -> Option<DateTime<Utc>> {
    let excel_epoch = NaiveDate::from_ymd_opt(1900, 1, 1)?;

    let adjusted_serial_number = if excel_serial_number > 60.0 {
        excel_serial_number - 1.0
    } else {
        excel_serial_number
    };

    let days = adjusted_serial_number.trunc() as i64;
    let fractional_day = adjusted_serial_number.fract();

    let duration_days = Duration::days(days);
    let duration_seconds = Duration::seconds((fractional_day * 24.0 * 60.0 * 60.0) as i64);

    let datetime = excel_epoch.and_hms_opt(0, 0, 0)? + duration_days + duration_seconds;

    Some(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
}

pub fn extract_maintenance_tasks(task_rows: Vec<Vec<Data>>) -> Vec<MaintenanceTask> {
    let tasks: Vec<MaintenanceTask> = task_rows
        .iter()
        .filter(
            |task|
                task[9] != Data::Empty &&
                task[10] != Data::Empty &&
                task[16] != Data::Empty &&
                task[0].to_string() != "Cadence".to_string() &&
                task[0].to_string() != "Daily".to_string() &&
                task[11].to_string() == "Brahm".to_string() &&
                task[14] != Data::Empty
        )
        .map(|task| MaintenanceTask::new(&task))
        .collect();

    tasks
}

pub fn get_next_weeks_tasks(tasks: &Vec<MaintenanceTask>) -> Vec<MaintenanceTask> {
    tasks
        .iter()
        .filter(|t| {
            if let Some(completed_by) = &t.completed_by {
                let delta = get_time_difference_from_now(&completed_by);
                info!("delta: {:?}", delta);

                delta < TimeDelta::weeks(1)
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

pub fn get_next_months_tasks(tasks: &Vec<MaintenanceTask>) -> Vec<MaintenanceTask> {
    tasks
        .iter()
        .filter(|t| {
            if let Some(completed_by) = &t.completed_by {
                let delta = get_time_difference_from_now(&completed_by);

                info!("true");

                delta < TimeDelta::weeks(4)
            } else {
                info!("false");

                false
            }
        })
        .cloned()
        .collect()
}
