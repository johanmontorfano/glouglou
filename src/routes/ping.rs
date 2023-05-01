use std::collections::HashMap;
use crate::{web::utils::{request::Request, response::Response}, config::generic::GenericConfiguration};

pub fn route(_: &Request, _conf: &GenericConfiguration) -> Response {
    Response {
        code: 200,
        status: "OK".into(),
        headers: HashMap::new(),
        body: "pong".into(),
    }
}
