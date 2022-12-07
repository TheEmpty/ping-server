use crate::client::Peer;
use derive_getters::Getters;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Getters)]
pub(crate) struct Config {
    peers: Option<Vec<Peer>>, // TODO: these aren't really peers.
    server: Option<Server>,
}

#[derive(Clone, Deserialize, Getters)]
pub(crate) struct Server {
    host: String,
    port: usize,
    key: String,
    read_timeout: u64, // socket.set_read_timeout takes u64
    wait_seconds: u8,
}

fn get_config_path() -> String {
    match std::env::args().nth(1) {
        Some(x) => {
            log::debug!("Using argument for config file: '{x}'.");
            x
        }
        None => {
            log::debug!("Using default config file path, ./config.json");
            "config.json".to_string()
        }
    }
}

impl Config {
    pub(crate) fn load_from_arg() -> Self {
        let file_path = get_config_path();
        let msg = format!("Could not find {file_path}");
        let config_file = File::open(file_path).expect(&msg);
        let config_reader = BufReader::new(config_file);
        serde_json::from_reader(config_reader).expect("Error reading configuration.")
    }
}
