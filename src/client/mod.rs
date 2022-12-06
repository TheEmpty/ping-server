mod connected;
mod not_connected;
mod peer;

pub(crate) use connected::{Connected, PING_BYTES};
pub(crate) use not_connected::NotConnected;
pub(crate) use peer::Peer;

use crate::config::Config;
use std::time::Duration;

#[derive(Debug)]
pub(crate) enum Connection {
    NotConnected(NotConnected),
    Connected(Connected),
}

impl Connection {
    pub(crate) fn peer(&self) -> &Peer {
        match self {
            Connection::NotConnected(nc) => &nc.peer,
            Connection::Connected(c) => &c.peer,
        }
    }
}

pub(crate) async fn connect_to_server(config: &Config) {
    for peer in config.peers() {
        let mut state = NotConnected::new(peer.clone()).into_connection();
        let key = peer.key().as_bytes().to_vec();
        tokio::spawn(async move {
            loop {
                state = match state {
                    Connection::NotConnected(nc) => nc.connect(&key),
                    Connection::Connected(c) => c.ping(),
                };

                let wait_seconds = *state.peer().wait_seconds();
                let wait_duration = Duration::from_secs(wait_seconds.into());
                log::trace!("Waiting {wait_seconds}s");
                tokio::time::sleep(wait_duration).await;
                log::trace!("Done waiting {wait_seconds}s");
            }
        });
    }
}
