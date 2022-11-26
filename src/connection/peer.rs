use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Getters)]
pub(crate) struct Peer {
    host: String,
    port: usize,
    wait_seconds: u64,
    key: String,
}

impl Peer {
    pub(crate) fn to_uri(&self) -> String {
        format!("{}:{}", self.host(), self.port())
    }
}
