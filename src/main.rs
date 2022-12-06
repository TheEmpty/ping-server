mod client;
mod config;
mod server;

use crate::config::Config;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::load_from_arg();
    let clients = Arc::new(Mutex::new(HashMap::new()));

    client::connect_to_server(&config).await;
    server::start(&config, clients.clone()).await;
    // TODO: web server / Rocket for metrics using clients

    if config.server().is_none() {
        loop {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    }
}
