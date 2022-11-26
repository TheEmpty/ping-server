use crate::config::Peer;
use std::io::Write;
use std::net::TcpStream;

// TODO: organize into mod/sub files

pub(crate) const PING_BYTES: &[u8] = "PING".as_bytes();

#[derive(Debug)]
pub(crate) enum Connection {
    NotConnected(NotConnected),
    Connected(Connected),
}

#[derive(Debug)]
pub(crate) struct NotConnected {
    peer: Peer,
}

#[derive(Debug)]
pub(crate) struct Connected {
    peer: Peer,
    stream: TcpStream,
}

impl NotConnected {
    pub(crate) fn new(peer: Peer) -> NotConnected {
        NotConnected { peer }
    }

    pub(crate) fn into_connection(self) -> Connection {
        Connection::NotConnected(self)
    }

    pub(crate) fn connect(self, key: &[u8]) -> Connection {
        let uri = format!("{}:{}", self.peer.host(), self.peer.port());
        match TcpStream::connect(uri.clone()) {
            Ok(mut stream) => {
                log::trace!("Connected to peer {uri}, sending key.");
                let _ = stream.write(key);
                Connected {
                    peer: self.peer,
                    stream,
                }
                .into_connection()
            }
            Err(e) => {
                log::error!("Failed to connect to peer {uri}: {e}");
                self.into_connection()
            }
        }
    }
}

impl Connected {
    pub(crate) fn ping(mut self) -> Connection {
        match self.stream.write(PING_BYTES) {
            Ok(_) => {
                // TODO: move this to a Display for peer
                let uri = format!("{}:{}", self.peer.host(), self.peer.port());
                log::trace!("Sent ping bytes to {}", uri);
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
