use crate::{config::generic::{GenConfEmail, GenConfDkim}, 
    utils::fs::read_file};
use lettre::{
    message::{
        Mailbox,
        dkim::DkimConfig, DkimSigningKey, dkim_sign
    },
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};

// This `struct` allows the usage of `lettre` inside the whole application by setting only one perpetuous connection
// with the server that is reused within the app.
pub struct Turkey {
    pub creds: Credentials,
    pub connection: SmtpTransport,
    pub email_config_copy: GenConfEmail,
    pub dkim_configuration: Option<DkimConfig>,
    pub dkim_enabled: bool
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
    pub fn make_smtp(email_config: &GenConfEmail,
                     dkim_config: &Option<GenConfDkim>) -> Self {
        let creds = Credentials::new(email_config.address.clone(), email_config.password.clone());
        let connection = SmtpTransport::relay(&email_config.host)
            .unwrap()
            .credentials(creds.clone())
            .build();
        let dkim_config = if dkim_config.is_some() {
            let dkim_config: GenConfDkim = dkim_config.clone().unwrap();

            if let Ok(pv_key) = read_file(&dkim_config.private_key_path) {
                let dkim_signing_key = DkimSigningKey::new(
                        &pv_key,
                        lettre::message::DkimSigningAlgorithm::Rsa
                    ).expect("Failed to build DKIM statement.");
            
                Option::Some(DkimConfig::default_config(
                    dkim_config.selector, 
                    dkim_config.domain,
                    dkim_signing_key
                ))
            } else { Option::None }
        } else { Option::None };

        Self {
            creds,
            connection,
            email_config_copy: email_config.clone(),
            dkim_enabled: dkim_config.is_some(),
            dkim_configuration: dkim_config
        }
    }

    // Send an email.
    pub fn send_email(&self, email: Email) -> Result<Response, String> {
        // Build mailboxes now to prevent any `MissingParts` server crash.
        let source_mailbox =
            format!("{} <{}>", 
                    email.from_name, 
                    self.email_config_copy.address).parse::<Mailbox>();
        let recipient_mailbox =
            format!("{} <{}>", 
                    email.to_name, 
                    email.to_email).parse::<Mailbox>();
        let cc_mailbox =
            format!("{} <{}>", 
                    email.cc_name.unwrap_or("".into()), 
                    email.cc_email.unwrap_or("".into())).parse::<Mailbox>();

        if source_mailbox.is_err() || recipient_mailbox.is_err() {
            return Err("Malformed source/recipient identity: format it as 'John Doe <johndoe@glouglou.johanmontorfano.com>'".into());
        } else {
            let source_mailbox = source_mailbox.unwrap();
            let recipient_mailbox = recipient_mailbox.unwrap();

            // Creates the email.
            let mut email_builder = Message::builder()
                .from(source_mailbox)
                .to(recipient_mailbox)
                .subject(email.subject);

            // If parsing has been done for CC, it means that a CC has been 
            // used and will be added to the email.
            if cc_mailbox.is_ok() {
                email_builder = email_builder.cc(cc_mailbox.unwrap());
            }

            let mut email = email_builder.body(email.body).unwrap();

            // If DKIM is provided, the message is signed.
            if self.dkim_enabled {
                dkim_sign(&mut email, 
                          self.dkim_configuration.as_ref().unwrap());
            }

            return match self.connection.send(&email) {
                Ok(response) => Ok(response),
                Err(reason) => Err(reason.to_string()),
            };
        }
    }
}
