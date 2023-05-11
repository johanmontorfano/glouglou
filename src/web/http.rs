use super::{
    server::Server,
    utils::{request::Request, response::Response},
};
use crate::{config::generic::GenericConfiguration, get_router, utils::log::Log};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

// Serve an HTTP server using the `Server` structure.:
// TODO: SETUP HTTPS on `https.rs`
pub trait HTTP {
    fn from_server(server_config: Server) -> Self;
    fn listen(&self, panic: Option<bool>, gen_conf: GenericConfiguration) -> ();
}

impl HTTP for Server {
    fn from_server(mut server_config: Server) -> Server {
        server_config.log.out(format!(
            "The server instance made to listen at port {}, will be used as an HTTP server.",
            server_config.port
        ));

        // check if the server security is set to `false`.
        if server_config.secure {
            server_config.secure = false;
        }

        // replace the log instance by a custom log instance.
        server_config.log = Log::new(format!("HTTP@{}", server_config.port));
        server_config
    }

    fn listen(&self, panic: Option<bool>, gen_conf: GenericConfiguration) {
        // Start listening at requests
        match TcpListener::bind(format!("127.0.0.1:{}", self.port)) {
            Ok(listener) => {
                for connection in listener.incoming() {
                    if connection.is_err() {
                        self.log.out("Connection to client failed.");
                    } else {
                        let mut connection = connection.unwrap();

                        self.log.out(format!(
                            "Connection to {} opened.",
                            connection.peer_addr().unwrap().to_string()
                        ));

                        // Read the stream.
                        let reader = BufReader::new(&mut connection);
                        // Transcripts the request data into a String to later parses it.
                        let local_payload: Vec<String> = reader
                            .lines()
                            .map(|result| result.unwrap())
                            .take_while(|line| !line.is_empty())
                            .collect();
                        let bytes_read = local_payload.join("\n");

                        self.log.out(format!(
                            "Read {} bytes from {:?}",
                            bytes_read.len(),
                            connection.peer_addr().unwrap()
                        ));

                        // When it has finished reading the data from the request body, it's Arced and the rest of the operations happens on
                        // a thread. The connection is also arced.
                        let arc_payload = Arc::new(bytes_read);
                        let tarc_payload = arc_payload.clone();
                        let mtx_connection = Mutex::new(connection);

                        let gen_conf_copy = gen_conf.clone();

                        thread::spawn(move || {
                            let mut conn = mtx_connection.lock().unwrap();
                            // Create a new log session for this thread.
                            let log = Log::new(format!(
                                "ReqHandler::{:?}",
                                conn.peer_addr().unwrap().to_string()
                            ));

                            log.out(format!(
                                "Processing request from {}.",
                                conn.peer_addr().unwrap().to_string()
                            ));

                            match Request::parse(tarc_payload.to_string()) {
                                Ok(mut parsed_req) => {
                                    // If the content has been successfully parsed, the source is added below because the raw request data doesn't contains
                                    // the request source.
                                    parsed_req.source = conn.peer_addr().unwrap().to_string();

                                    // Store the response.
                                    let response_data = get_router()
                                        .routing(parsed_req.clone(), &gen_conf_copy)
                                        .stringify(&parsed_req);
                                    match conn.write_all(response_data.as_bytes()) {
                                        Ok(()) => {
                                            log.out(format!(
                                                "Sent {} bytes to {}.",
                                                response_data.len(),
                                                conn.peer_addr().unwrap().to_string()
                                            ));
                                        }
                                        Err(_) => {
                                            log.out(format!(
                                                "Failed to send {} bytes to {}.",
                                                response_data.len(),
                                                conn.peer_addr().unwrap().to_string()
                                            ));
                                        }
                                    };
                                }
                                Err(()) => {
                                    log.out(format!(
                                        "Cannot parse request from {:?}, malformed.",
                                        conn.peer_addr().unwrap()
                                    ));
                                    match conn.write_all(
                                        Response {
                                            headers: HashMap::new(),
                                            code: 400,
                                            status: "MALFORMED_REQ".into(),
                                            body: "".into(),
                                        }
                                        .stringify(&Request {
                                            params: HashMap::new(),
                                            headers: HashMap::new(),
                                            body: "".into(),
                                            source: "".into(),
                                            method: "".into(),
                                            protocol: "HTTP/1.1".into(),
                                            raw_url: "".into(),
                                        })
                                        .as_bytes(),
                                    ) {
                                        Ok(()) => log.out(format!(
                                            "Sent a response to {:?}",
                                            conn.peer_addr().unwrap()
                                        )),
                                        Err(_) => log.out(format!(
                                            "Failed to send a response to {:?}",
                                            conn.peer_addr().unwrap()
                                        )),
                                    }
                                }
                            };
                        })
                        .join()
                        .unwrap();
                    }
                }
            }
            Err(err) => {
                self.log
                    .out(format!("Failed to start the server: {}", err.to_string()));
                if panic.is_some() && panic.unwrap() {
                    panic!("Failed to start the server.");
                }
            }
        }
    }
}
