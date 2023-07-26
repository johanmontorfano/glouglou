use routes::{access_ok, ping, send};
use tokio::spawn;
use utils::{api_keys::sks_routine, fs::read_file, log::Log, router::Router};
use web::{http::HTTP, server::Server};

mod config;
mod email;
mod routes;
mod utils;
mod web;

pub fn get_router() -> Router {
    let mut router = Router::new();
    router.on_target("/ping", Box::new(ping::route));
    router.on_target("/access-ok", Box::new(access_ok::route));
    router.on_target("/send", Box::new(send::route));
    return router;
}

#[tokio::main]
async fn main() {
    let log = Log::new("🦃".into());

    spawn(async {
        // ! There is no method implemented to make it recover from this fail.
        sks_routine().await;
    });

    // Read the config file.
    if let Ok(content) = read_file("./glouglou.conf.toml") {
        let converted_config = toml::from_str::<config::generic::GenericConfiguration>(&content)
            .expect("Failed to parse the GlouGlou configuration file.");

        let http_server = Server::new(converted_config.server.http_port);

        log.out(format!(
            "Will send turkeys from {}.",
            converted_config.email.address
        ));
        log.out(format!(
            "Will open a port at {}(HTTP) for turkeys.",
            converted_config.server.http_port
        ));

        http_server.listen(Some(true), converted_config);
    } else {
        log.out("The GlouGlou configuration file is not found, please create a `glouglou.conf.toml` alongside this executable.");
    }
}
