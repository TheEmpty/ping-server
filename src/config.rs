use derive_getters::Getters;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Getters)]
pub(crate) struct Config {
    peers: Vec<Peer>,
    server: Option<Server>,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub(crate) struct Peer {
    host: String,
    port: usize,
    wait_seconds: u64, // TODO: better term?
    key: String,
}

#[derive(Clone, Deserialize, Getters)]
pub(crate) struct Server {
    host: String,
    port: usize,
    key: String,
    read_timeout: u64,
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
        let config_file = File::open(file_path).expect("Could not find {file_path}");
        let config_reader = BufReader::new(config_file);
        serde_json::from_reader(config_reader).expect("Error reading configuration.")
    }
}
