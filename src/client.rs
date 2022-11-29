use crate::{
    config::Config,
    connection::{Connection, NotConnected},
};
use std::time::Duration;

pub(crate) async fn connect_to_peers(config: &Config) {
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
