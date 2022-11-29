use super::{Connection, NotConnected, Peer};
use std::io::Write;
use std::net::TcpStream;

pub(crate) const PING_BYTES: &[u8] = "P".as_bytes();

#[derive(Debug)]
pub(crate) struct Connected {
    pub(super) peer: Peer,
    pub(super) stream: TcpStream,
}

impl Connected {
    pub(crate) fn ping(mut self) -> Connection {
        match self.stream.write(PING_BYTES) {
            Ok(_) => {
                log::trace!("Sent ping bytes to {}", self.peer.to_uri());
                self.into_connection()
            }
            Err(e) => {
                log::error!("Ping failed: {}", e);
                self.not_connected().into_connection()
            }
        }
    }

    pub(crate) fn not_connected(self) -> NotConnected {
        NotConnected { peer: self.peer }
    }

    pub(crate) fn into_connection(self) -> Connection {
        Connection::Connected(self)
    }
}
