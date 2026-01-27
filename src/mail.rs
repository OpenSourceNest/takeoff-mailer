use lettre::{
    Message, SmtpTransport, Transport,
    message::{Body, SinglePart},
    transport::smtp::authentication::Credentials,
};

use crate::{
    config::Config, email_system::EmailTask,
    template::registration_successful::AttendeeRegistrationSuccessful,
};

pub async fn send_email_logic(config: &Config, task: &EmailTask) -> Result<(), String> {
    let singlepart;

    if task.category == "Attendee_Registration_Successful" {
        singlepart = SinglePart::html(AttendeeRegistrationSuccessful(&task.category));
    } else {
        singlepart = SinglePart::plain(Body::new(vec![]));
    }

    let mailer = SmtpTransport::relay(&config.mail_host)
        .unwrap()
        .credentials(Credentials::new(
            (&config.mail_username).to_string(),
            (&config.mail_password).to_string(),
        ))
        .port(config.mail_port)
        .build();

    let email = Message::builder()
        .from(config.mail_sender.parse().unwrap())
        .to(task.to.parse().unwrap())
        .subject(&task.subject)
        .singlepart(singlepart)
        .unwrap();

    let result = mailer.send(&email);

    match result {
        Ok(_) => {
            println!("[EMAIL SENT] To: {}, Subject: {}", task.to, task.subject);
        }

        Err(e) => {
            eprintln!("[EMAIL ERROR] Could not send email: {:?}", e);
            return Err(format!("Could not send email: {:?}", e));
        }
    }

    Ok(())
}
