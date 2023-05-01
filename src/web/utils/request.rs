use std::collections::HashMap;
use crate::utils::log::Log;

#[derive(Clone)]
pub struct Request {
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub source: String,
    pub method: String,
    pub protocol: String,
    pub raw_url: String,
}

// This function is made to parse a request into a `Req` struct
impl Request {
    // Parses the URL parameters of a request
    pub fn url_parse(url: String) -> HashMap<String, String> {
        // HashMap to fill.
        let mut params = HashMap::new();
        // Determines if the url contains params.
        match url.split_once("?") {
            Some(splitted_content) => {
                // We only keep the 2nd part of the url as it's the one which contains the URL's params.
                // We split the params part at each '&' to handle every parameter
                for line in splitted_content.1.split("&") {
                    // We determine if the entry contains an "=" to split to be sure we'll have two distinguished items,
                    // the value's name and the value itself.
                    match line.split_once("=") {
                        Some(items) => {
                            params.insert(items.0.to_string(), items.1.to_string());
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }

        params
    }

    // Parses the content of a request.
    pub fn parse(raw_req: String) -> Result<Request, ()> {
        let log = Log::new("Req::parse".to_string());
        let mut parsed_req = Request {
            params: HashMap::new(),
            headers: HashMap::new(),
            body: String::new(),
            source: String::new(),
            method: String::new(),
            protocol: String::new(),
            raw_url: String::new(),
        };

        // Flag the parser as putting anything in the body.
        let mut wait_for_body = false;

        // Read each line of the request, if anything weird happens the line is ignored
        for line in raw_req.lines() {
            if parsed_req.protocol.is_empty() {
                // Split the first line of the request to properly handle `method`, `protocol_version` and `raw_url`
                let splitted: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                if splitted.len() != 3 {
                    log.out("The head line of the request is malformed.");
                    return Err(());
                } else {
                    parsed_req.method = splitted[0].to_string();
                    parsed_req.raw_url = splitted[1].to_string();
                    parsed_req.protocol = splitted[2].to_string();

                    // After doing anything else, we break ddown URL params.
                    parsed_req.params = Request::url_parse(splitted[1].to_string());
                }
            } else {
                // If the body data is expected, everything else is aborted.
                if wait_for_body {
                    parsed_req.body.push_str(&format!("{}\n", line));
                } else {
                    match line.split_once(": ") {
                        Some(splitted) => {
                            parsed_req
                                .headers
                                .insert(splitted.0.to_string(), splitted.1.to_string());
                        }
                        None => {
                            // If the line cannot be parsed as a line header, the parser is made aware that anything after
                            // this will be the body of the request. It also requires the line to be empty.
                            if line.is_empty() {
                                wait_for_body = true;
                            }
                        }
                    }
                }
            }
        }

        Ok(parsed_req)
    }
}
