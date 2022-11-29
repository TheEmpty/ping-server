mod client;
mod config;
pub(crate) mod connection;
mod server;

use crate::config::Config;
use std::time::Duration;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::load_from_arg();

    // TODO:
    // share mutex with peers - so web metrics can update

    client::connect_to_peers(&config).await;
    server::start(&config).await;

    if config.server().is_none() {
        loop {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    }
}
