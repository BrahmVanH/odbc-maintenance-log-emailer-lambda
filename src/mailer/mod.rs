use lettre::message::{ Mailbox, header::ContentType };
use lettre::transport::smtp::authentication::Credentials;
use lettre::{ Message, SmtpTransport, Transport };

use crate::config::{ LettreAddress, LettreConfig };
use crate::AppError;

use tracing::info;

pub mod email_body;

pub fn send_email(body: &str, lettre_config: &LettreConfig) -> Result<(), AppError> {
    let LettreConfig { smtp_username, smtp_password, to, reply_to, from, subject } = lettre_config;
    info!("sending email");
    let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| AppError::InternalServerError(format!("Could not create email: {}", e)))?
        .credentials(creds)
        .build();

    let email = create_email(body, to, from, reply_to, subject)?;
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => {
            return Err(AppError::InternalServerError(format!("Could not send email: {}", e)));
        }
    }

    Ok(())
}

fn create_email(
    body: &str,
    to: &LettreAddress,
    from: &LettreAddress,
    reply_to: &LettreAddress,
    subject: &String
) -> Result<Message, AppError> {
    let email = Message::builder()
        .from(
            Mailbox::new(
                Some(from.name.clone()),
                from.email
                    .parse()
                    .map_err(|e|
                        AppError::InternalServerError(format!("Could not parse from email: {}", e))
                    )?
            )
        )
        .reply_to(
            Mailbox::new(
                Some(reply_to.name.clone()),
                reply_to.email
                    .parse()
                    .map_err(|e|
                        AppError::InternalServerError(
                            format!("Could not parse reply_to email: {}", e)
                        )
                    )?
            )
        )

        .to(
            Mailbox::new(
                Some(to.name.clone()),
                to.email
                    .parse()
                    .map_err(|e|
                        AppError::InternalServerError(format!("Could not parse to email: {}", e))
                    )?
            )
        )
        .subject(subject.clone())
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| AppError::InternalServerError(format!("Could not create email: {}", e)))?;

    Ok(email)
}
