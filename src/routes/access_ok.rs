use crate::{web::utils::{request::Request, response::Response, uri::decode_uri}, utils::api_keys::APIKeyData, config::generic::GenericConfiguration};
use std::collections::HashMap;

// Checks if the API Key sent by the client matches with the one stored locally.
pub fn route(req: &Request, _conf: &GenericConfiguration) -> Response {
    let mut response = Response {
        code: 500,
        status: "MISSING_PARAMS".into(),
        headers: HashMap::new(),
        body: "{'ok': false}".into(),
    };

    // Checks if arguments are provided on the request.
    if req.params.contains_key("api-key") {
        response.code = 200;
        response.status = "OK".into();

        let remote_api_key = decode_uri(req.params.get("api-key").unwrap().clone());
        let local_api_key_data = APIKeyData::load_api_key().unwrap();

        // Checks if everything matches.
        if remote_api_key == local_api_key_data.api_key {
            response.body = "{'ok': true}".into();
        }
    } 


    return response;
}
