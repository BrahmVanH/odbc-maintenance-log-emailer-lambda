use opendal::Operator;
use tokio_cron_scheduler::{ Job, JobScheduler, JobSchedulerError };
use sentry::capture_error;

use crate::{ config::LettreConfig, handle_send_email };

pub async fn init_job_scheduler(
    dbx: &Operator,
    lettre_config: &LettreConfig
) -> Result<JobScheduler, JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    let schedule = "0 4 * * 1-5";

    let dbx_clone = dbx.clone();
    let lettre_config_clone = lettre_config.clone();

    let job = Job::new_async(schedule, move |_uuid, _l| {
        let dbx = dbx_clone.clone();
        let lettre_config = lettre_config_clone.clone();
        Box::pin(async move {
            match handle_send_email(&dbx, &lettre_config).await {
                Ok(_) => (),
                Err(e) => {
                    capture_error(&e);
                }
            }
        })
    })?;
    sched.add(job).await?;

    Ok(sched)
}
