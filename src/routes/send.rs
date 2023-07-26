use std::collections::HashMap;

use crate::{
    config::generic::GenericConfiguration,
    email::send::{Email, Turkey},
    utils::api_keys::APIKeyData,
    web::utils::{request::Request, response::Response, uri::decode_uri},
};

// This route is used to send emails.
pub fn route(req: &Request, conf: &GenericConfiguration) -> Response {
    let mut response = Response {
        code: 500,
        status: "MISSING_PARAMS".into(),
        headers: HashMap::new(),
        body: "ERROR 500 MISSING PARAMS".into(),
    };

    if req.params.contains_key("api-key")
        && req.params.contains_key("to")
        && req.params.contains_key("subject")
        && req.params.contains_key("body")
        && req.params.contains_key("from-name")
    {
        response.code = 200;
        response.status = "OK".into();
        response.body = "OK".into();

        let remote_api_key = decode_uri(req.params.get("api-key").unwrap().clone());
        let local_api_key_data = APIKeyData::load_api_key().unwrap();

        // Checks if everything matches, then it sends the email.
        if remote_api_key == local_api_key_data.api_key {
            let send_to = decode_uri(req.params.get("to").unwrap().clone());
            let send_body = decode_uri(req.params.get("body").unwrap().clone());
            let send_subject = decode_uri(req.params.get("subject").unwrap().clone());
            let send_with_name = decode_uri(req.params.get("from-name").unwrap().clone());

            // As `to` is formatted like NAME <EMAIL>, it's parsed.
            let (name, mut email) = send_to.split_once("<").unwrap();
            let strng_email = email.replace(">", "");
            email = &strng_email;

            let sender = Turkey::make_smtp(&conf.email, &conf.dkim);
            let mut built_email = Email {
                from_name: send_with_name.into(),
                to_email: email.into(),
                to_name: name.into(),
                subject: send_subject,
                body: send_body,
                cc_email: Option::None,
                cc_name: Option::None,
            };

            if req.params.contains_key("cc") {
                let cc_to = decode_uri(req.params.get("cc").unwrap().clone());
                let (name, mut email) = cc_to.split_once("<").unwrap();
                let strng_email = email.replace(">", "");
                email = &strng_email;

                built_email.cc_email = Option::Some(email.into());
                built_email.cc_name = Option::Some(name.into());
            }

            match sender.send_email(built_email) {
                Ok(res) => {
                    response.status = "MAIL_SENT".into();
                    println!("{}: {}", res.code(), res.code().detail);
                }
                Err(reason) => {
                    println!("Failed to fetch email: {}", reason);
                    response.status = "FETCHING_FAILED".into();
                    response.code = 500;
                }
            }
        } else {
            println!("Incorrect API key.");
            response.status = "INVALID_API_KEY".into();
            response.code = 400;
        }
    }

    return response;
}
