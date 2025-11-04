use chrono::{ DateTime, Datelike, Duration, Utc };
use chrono_humanize::HumanTime;

use crate::models::MaintenanceTask;

use tracing::info;

pub fn format_basic_date(date: &DateTime<Utc>) -> String {
    // Formats into "MM/DD/YYYY"
    format!("{}/{}/{}", date.month(), date.day(), date.year())
}

pub fn get_time_difference_from_now(date: &DateTime<Utc>) -> Duration {
    let now = Utc::now();
    let delta = date.clone() - now;

    delta
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
            let last_completed = match &task.last_completed {
                Some(dt) => format_basic_date(dt),
                None => "N/A".to_string(),
            };

            let completed_by;
            let completed_by_human;

            match task.completed_by {
                Some(dt) => {
                    completed_by = format_basic_date(&dt);
                    completed_by_human = format!(" - {}", handle_format_distance_to_now(dt));
                }
                None => {
                    completed_by = "N/A".to_string();
                    completed_by_human = "".to_string();
                }
            }

            email_text = format!(
                "{}	Title: {}\n	Description: {}\n	Last Completed: {:?}\n	Complete By: {:?} - {:?}\n\n",
                email_text,
                task.title,
                task.description,
                last_completed,
                completed_by,
                completed_by_human
            );
        }
    }
    email_text = format!("{} \n\nNext 30 days:\n\n", email_text);
    if next_months_tasks.len() > 0 {
        for task in next_months_tasks {
            let last_completed = match &task.last_completed {
                Some(dt) => format_basic_date(dt),
                None => "N/A".to_string(),
            };

            let completed_by;
            let completed_by_human;

            match task.completed_by {
                Some(dt) => {
                    completed_by = format_basic_date(&dt);
                    completed_by_human = format!(" - {}", handle_format_distance_to_now(dt));
                }
                None => {
                    completed_by = "N/A".to_string();
                    completed_by_human = "".to_string();
                }
            }

            email_text = format!(
                "{}	Title: {}\n	Description: {}\n	Last Completed: {:?}\n	Complete By: {:?} - {:?}\n\n",
                email_text,
                task.title,
                task.description,
                last_completed,
                completed_by,
                completed_by_human
            );
        }
    }
    info!("email text: {:?}", email_text);
    email_text
}
