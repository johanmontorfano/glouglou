use crate::utils::log::Log;

// This `struct` is made to store any data that a server may need to contain.
pub struct Server {
    pub port: u16,
    pub secure: bool,
    pub log: Log,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            secure: false,
            log: Log::new(format!("server@{}", port)),
        }
    }
}
