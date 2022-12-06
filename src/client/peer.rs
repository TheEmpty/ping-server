use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Getters)]
pub(crate) struct Peer {
    host: String,
    port: usize,
    #[serde(skip_deserializing, default = "thirty")]
    pub(super) wait_seconds: u8,
    key: String,
    name: String,
}

fn thirty() -> u8 {
    30
}

impl Peer {
    pub(crate) fn to_uri(&self) -> String {
        format!("{}:{}", self.host(), self.port())
    }
}
