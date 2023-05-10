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
    {
        response.code = 200;
        response.status = "OK".into();
        response.body = "CHECK STATUS CODE".into();

        let remote_api_key = decode_uri(req.params.get("api-key").unwrap().clone());
        let local_api_key_data = APIKeyData::load_api_key().unwrap();

        // Checks if everything matches, then it sends the email.
        if remote_api_key == local_api_key_data.api_key {
            let send_to = decode_uri(req.params.get("to").unwrap().clone());
            let send_body = decode_uri(req.params.get("body").unwrap().clone());
            let send_subject = decode_uri(req.params.get("subject").unwrap().clone());

            // As `to` is formatted like NAME <EMAIL>, it's parsed.
            let (name, mut email) = send_to.split_once("<").unwrap();
            let strng_email = email.replace(">", "");
            email = &strng_email;

            let sender = Turkey::make_smtp(&conf.email);
            let mut built_email = Email {
                from_name: conf.email.name.clone(),
                to_email: email.into(),
                to_name: name.into(),
                subject: send_subject,
                body: send_body,
                cc_email: Option::None,
                cc_name: Option::None,
            };

            if req.params.contains_key("cc") {
                let (name, mut email) = send_to.split_once("<").unwrap();
                let strng_email = email.replace(">", "");
                email = &strng_email;

                built_email.cc_email = Option::Some(email.into());
                built_email.cc_name =  Option::Some(name.into());
            }

            if sender.send_email(built_email).is_err() {
                response.status = "FETCHING_FAILED".into();
            } else {
                response.status = "MAIL_SENT".into();
            }
        } else {
            response.status = "INVALID_API_KEY".into();
        }
    }

    return response;
}
