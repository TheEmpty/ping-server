mod client;
mod config;
mod server;
mod web_server;

#[macro_use]
extern crate rocket;

use crate::config::Config;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::load_from_arg();

    client::connect_to_server(&config).await;
    server::start(&config).await;

    if config.server().is_none() {
        loop {
            sleep(Duration::from_secs(60 * 60)).await;
        }
    }
}
