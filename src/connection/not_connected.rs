use super::{Connected, Connection, Peer};
use std::net::TcpStream;
use std::{
    io,
    io::{Read, Write},
};

#[derive(Debug)]
pub(crate) struct NotConnected {
    pub(super) peer: Peer,
}

fn read_bool(stream: &mut TcpStream) -> Result<bool, io::Error> {
    let mut buff = vec![0; 1];
    stream.read_exact(&mut buff)?;
    Ok(buff == "1".as_bytes())
}

// TODO: read wait_time from server
#[allow(dead_code)]
fn read_u8(stream: &mut TcpStream) -> Result<u8, io::Error> {
    let mut buff = [0];
    stream.read_exact(&mut buff)?;
    Ok(u8::from_le_bytes(buff))
}

impl NotConnected {
    pub(crate) fn new(peer: Peer) -> NotConnected {
        NotConnected { peer }
    }

    pub(crate) fn into_connection(self) -> Connection {
        Connection::NotConnected(self)
    }

    fn send_key(&self, stream: &mut TcpStream, key: &[u8]) -> Result<bool, io::Error> {
        log::trace!("Connecting to peer {}, sending key.", self.peer.to_uri());
        let _ = stream.write(key);
        read_bool(stream)
    }

    fn send_name(&self, stream: &mut TcpStream) {
        log::trace!(
            "Connecting to peer {}, sending name {}.",
            self.peer.to_uri(),
            self.peer.name()
        );
        let mut name = self.peer.name().as_bytes().to_vec();
        name.resize(100, 0);
        let _ = stream.write(&name);
    }

    pub(crate) fn connect(self, key: &[u8]) -> Connection {
        match TcpStream::connect(self.peer.to_uri()) {
            Ok(mut stream) => match self.send_key(&mut stream, key) {
                Ok(true) => {
                    self.send_name(&mut stream);
                    Connected {
                        peer: self.peer,
                        stream,
                    }
                    .into_connection()
                }
                Ok(false) => {
                    log::trace!("Key is incorrect");
                    self.into_connection()
                }
                Err(e) => {
                    log::error!("Failed to send key: {e}");
                    self.into_connection()
                }
            },
            Err(e) => {
                log::error!("Failed to connect to peer {}: {e}", self.peer.to_uri());
                self.into_connection()
            }
        }
    }
}
