use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GenericConfiguration  {
    pub email: GenConfEmail,
    pub server: GenConfServer
}

// Configuration of the email address to use.
#[derive(Deserialize, Clone)]
pub struct GenConfEmail {
    pub address: String,
    pub password: String,
    pub host: String,
    pub port: u32
}

// Configuration of the server for the REST API.
#[derive(Deserialize, Clone)]
pub struct GenConfServer{
    pub http_port: u16,
    pub https_port: Option<u32>,
    pub https_cert_path: Option<String>,
    pub https_key_path: Option<String>
}