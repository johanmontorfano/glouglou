use std::collections::HashMap;

use crate::{web::utils::{request::Request, response::Response}, config::generic::GenericConfiguration};

// A router is a structure meant to be used to automatically call the correct function depending on the user input.
pub struct Router {
    routes: HashMap<String, Box<dyn Fn(&Request, &GenericConfiguration) -> Response>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn on_target<T: std::fmt::Display>(
        &mut self,
        path: T,
        route: Box<dyn Fn(&Request, &GenericConfiguration) -> Response>,
    ) {
        self.routes.insert(path.to_string(), route);
    }

    pub fn routing(&self, req: Request, gen_conf: &GenericConfiguration) -> Response {
        // Parses the request at `?` to ensure it makes a match only with the path, not the parameters. The path is stored here as a String.
        let path;
        match req.raw_url.split_once("?") {
            Some(splitted_url) => path = splitted_url.0.into(),
            None => path = req.clone().raw_url,
        };

        if self.routes.contains_key(&path) {
            return self.routes.get(&path).unwrap()(&req, gen_conf);
        }
        Response {
            code: 404,
            status: "NOT_FOUND".into(),
            headers: HashMap::new(),
            body: "404 NOT FOUND".into(),
        }
    }
}
