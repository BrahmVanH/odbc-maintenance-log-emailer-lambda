use calamine::Data;
use chrono::{ DateTime, Utc };
use serde::Deserialize;

use crate::dropbox::parse::convert_excel_date_to_datetime;

#[derive(Debug, Clone, Deserialize)]
pub struct MaintenanceTask {
    pub title: String,
    pub description: String,
    pub last_completed: Option<DateTime<Utc>>,
    pub completed_by: Option<DateTime<Utc>>,
}

impl MaintenanceTask {
    pub fn new(task_row: &Vec<Data>) -> Self {
        let last_completed = match task_row[14] {
            Data::DateTime(excel_dt) => convert_excel_date_to_datetime(excel_dt.as_f64()),
            _ => None,
        };

        let completed_by = match task_row[16] {
            Data::DateTime(excel_dt) => convert_excel_date_to_datetime(excel_dt.as_f64()),
            _ => None,
        };

        Self {
            title: task_row[9].to_string(),
            description: task_row[10].to_string(),
            last_completed,
            completed_by,
        }
    }
}
