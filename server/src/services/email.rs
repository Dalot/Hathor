use crate::models::invitation::Invitation;
use crate::errors::service::ServiceError;
use lettre::message::header::{ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    let email_body = format!(
        "<html>
        <head>
        </head>
        <body>
        Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>
         </body>
         </html>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    let header = ContentType::html();

    let email = Message::builder()
        .from("test@test.com".parse().unwrap())
        .to(invitation.email.parse().unwrap())
        .subject("Happy new year")
        .header(header)
        .body(email_body)
        .unwrap();

    let creds = Credentials::new("226a2849b719fd".to_string(), "9b0bff359cefa3".to_string());
    let mailer = SmtpTransport::starttls_relay("smtp.mailtrap.io")
        .unwrap()
        .credentials(creds)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        } // TODO: Make this return a service error?
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
