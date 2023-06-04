use crate::config::generic::GenConfEmail;
use lettre::{
    message::Mailbox,
    transport::smtp::{authentication::Credentials, response::Response, Error},
    Message, SmtpTransport, Transport,
};

// This `struct` allows the usage of `lettre` inside the whole application by setting only one perpetuous connection
// with the server that is reused within the app.
pub struct Turkey {
    pub creds: Credentials,
    pub connection: SmtpTransport,
    pub email_config_copy: GenConfEmail,
}

pub struct Email {
    pub from_name: String,
    pub to_email: String,
    pub to_name: String,
    pub cc_email: Option<String>,
    pub cc_name: Option<String>,
    pub subject: String,
    pub body: String,
}

impl Turkey {
    // Implements the `Turkey` using `SMTP`.
    pub fn make_smtp(email_config: &GenConfEmail) -> Self {
        let creds = Credentials::new(email_config.address.clone(), email_config.password.clone());
        let connection = SmtpTransport::relay(&email_config.host)
            .unwrap()
            .credentials(creds.clone())
            .build();

        Self {
            creds,
            connection,
            email_config_copy: email_config.clone(),
        }
    }

    // Send an email.
    pub fn send_email(&self, email: Email) -> Result<Response, Error> {
        // Build mailboxes now to prevent any `MissingParts` server crash.
        let source_mailbox =
            format!("{} <{}>", email.from_name, self.email_config_copy.address).parse::<Mailbox>();

        // Creates the email.
        let mut email_builder = Message::builder()
            .from(
                format!("{} <{}>", email.from_name, self.email_config_copy.address)
                    .parse()
                    .unwrap(),
            )
            .to(format!("{} <{}>", email.to_name, email.to_email)
                .parse()
                .unwrap())
            .subject(email.subject);

        // Determines if there is a CC, otherwise it does nothing.
        if email.cc_email.is_some() && email.cc_name.is_some() {
            email_builder = email_builder.cc(format!(
                "{} <{}>",
                email.cc_name.unwrap(),
                email.cc_email.unwrap()
            )
            .parse()
            .unwrap());
        }

        let email = email_builder.body(email.body).unwrap();

        return self.connection.send(&email);
    }
}
