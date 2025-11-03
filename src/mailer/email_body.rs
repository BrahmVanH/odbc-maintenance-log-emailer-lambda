use chrono::{ DateTime, Datelike, Duration, NaiveDate, Utc };
use chrono_humanize::HumanTime;

use crate::{ models::{ MaintenanceTask, TaskDate }, AppError };

use tracing::info;

pub fn format_basic_date(date: &DateTime<Utc>) -> String {
    // Formats into "MM/DD/YYYY"
    let formatted_date = format!("{}/{}/{}", date.month(), date.day(), date.year());

    formatted_date
}

pub fn get_time_difference_from_now(date: &DateTime<Utc>) -> Duration {
    let now = Utc::now();

    date.clone() - now
}

fn handle_format_distance_to_now(date: DateTime<Utc>) -> String {
    let human_time = HumanTime::from(date);
    format!("{}", human_time)
}

pub fn format_email_text(
    next_weeks_tasks: Vec<MaintenanceTask>,
    next_months_tasks: Vec<MaintenanceTask>
) -> String {
    let mut email_text = "This week:\n\n".to_string();

    if next_weeks_tasks.len() > 0 {
        for task in next_weeks_tasks {
            email_text = format!(
                "{}	Title: {}\n	Description: {}\n	Last Completed: {:?}\n	Complete By: {:?}\n\n",
                email_text,
                task.title,
                task.description,
                task.last_completed,
                task.completed_by
            );
        }
    }
    email_text = format!("{} \n\nNext 30 days:\n\n", email_text);
    if next_months_tasks.len() > 0 {
        for task in next_months_tasks {
            email_text = format!(
                "{}	Title: {}\n	Description: {}\n	Last Completed: {:?}\n	Complete By: {:?}\n\n",
                email_text,
                task.title,
                task.description,
                task.last_completed,
                task.completed_by
            );
        }
    }

    info!("email text: {:?}", email_text);
    email_text
}
