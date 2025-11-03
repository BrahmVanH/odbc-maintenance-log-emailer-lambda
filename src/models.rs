use calamine::{ Data, DataType };
use chrono::{ DateTime, Utc };
use serde::Deserialize;

use crate::dropbox::parse::convert_excel_date_to_datetime;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct MaintenanceTask {
    pub title: String,
    pub description: String,
    pub last_completed: TaskDate,
    pub completed_by: TaskDate,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TaskDate {
    DateVariant(DateTime<Utc>),
    StringVariant(String),
}

impl MaintenanceTask {
    pub fn new(task_row: &Vec<Data>) -> Self {
        let last_completed_f64 = match task_row[14] {
            Data::DateTime(excel_dt) => Some(excel_dt.as_f64()),
            _ => None,
        };

        info!("last completed: {:?}", &last_completed_f64);
        info!("last completed: {:?}", &task_row[14]);

        let last_completed = match last_completed_f64 {
            Some(date) =>
                match convert_excel_date_to_datetime(date) {
                    Some(datetime) => datetime.to_string(),
                    None => "N/A".to_string(),
                }
            None => "N/A".to_string(),
        };

        let completed_by_f64 = match task_row[16] {
            Data::DateTime(excel_dt) => Some(excel_dt.as_f64()),
            _ => None,
        };

        let completed_by = match completed_by_f64 {
            Some(date) =>
                match convert_excel_date_to_datetime(date) {
                    Some(datetime) => datetime.to_string(),
                    None => "N/A".to_string(),
                }
            None => "N/A".to_string(),
        };

        Self {
            title: task_row[9].to_string(),
            description: task_row[10].to_string(),
            last_completed,
            completed_by,
        }
    }
}
