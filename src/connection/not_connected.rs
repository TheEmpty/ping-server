use super::{Connected, Connection, Peer};
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub(crate) struct NotConnected {
    pub(super) peer: Peer,
}

impl NotConnected {
    pub(crate) fn new(peer: Peer) -> NotConnected {
        NotConnected { peer }
    }

    pub(crate) fn into_connection(self) -> Connection {
        Connection::NotConnected(self)
    }

    pub(crate) fn connect(self, key: &[u8]) -> Connection {
        match TcpStream::connect(self.peer.to_uri()) {
            Ok(mut stream) => {
                log::trace!("Connected to peer {}, sending key.", self.peer.to_uri());
                let _ = stream.write(key);
                log::trace!("Sent key - assuming it's correct and moving to Connected");
                Connected {
                    peer: self.peer,
                    stream,
                }
                .into_connection()
            }
            Err(e) => {
                log::error!("Failed to connect to peer {}: {e}", self.peer.to_uri());
                self.into_connection()
            }
        }
    }
}
