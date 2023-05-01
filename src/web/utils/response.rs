use crate::web::utils::request::Request;
use std::{collections::HashMap, ops::Add};

// This function is used to build Responses that can be sent to the client.
pub struct Response {
    pub code: u64,
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    // Create a new Response string from a Response data.
    pub fn stringify(&mut self, from_req: &Request) -> String {
        let mut str_response = String::new();

        // If the status is empty, a custom status is set.
        if self.status.is_empty() {
            self.status = "OK".into();
        }

        // Add the header line of the response.
        str_response.push_str(&format!(
            "{} {} {}\r\n",
            from_req.protocol, self.code, self.status
        ));

        // Add headers to the response on a string. Before doing this, the `Server` + `Content-Length` header is set to Groom.
        self.headers.insert("Server".into(), "Groom".into());
        self.headers
            .insert("Content-Length".into(), self.body.len().add(2).to_string());

        for header in &self.headers {
            str_response.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }
        // Add the body of the response.
        str_response.push_str(&format!("\r\n\r\n{}", self.body));
        str_response
    }
}
