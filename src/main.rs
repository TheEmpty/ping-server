mod config;
mod connection;

use crate::{
    config::Config,
    connection::{Connection, NotConnected, PING_BYTES},
};
use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

async fn connect_to_peers(config: &Config) {
    for peer in config.peers() {
        let mut state = NotConnected::new(peer.clone()).into_connection();
        let wait_seconds = *peer.wait_seconds();
        let key = peer.key().as_bytes().to_vec();
        tokio::spawn(async move {
            loop {
                state = match state {
                    Connection::NotConnected(nc) => nc.connect(&key),
                    Connection::Connected(c) => c.ping(),
                };
                // TODO: better way of handling this.
                tokio::time::sleep(Duration::from_secs(wait_seconds)).await;
            }
        });
    }
}

async fn check_key(key: Vec<u8>, socket: &mut TcpStream, addr: SocketAddr) -> Result<(), ()> {
    log::trace!("Waiting for key from {addr:?}");
    let mut sent_key = vec![0; key.len()];

    match socket.read_exact(&mut sent_key) {
        Ok(_) => {
            if sent_key != key {
                log::error!("{addr:?} did not send the correct key");
                Err(())
            } else {
                Ok(())
            }
        }
        Err(e) => {
            log::error!("{addr:?}: key check: {e}");
            Err(())
        }
    }
}

async fn listen(mut socket: TcpStream, addr: SocketAddr) {
    tokio::spawn(async move {
        let mut buff = vec![0; 4];
        loop {
            log::trace!("Waiting for {} bytes from {addr:?}", buff.len());
            match socket.read_exact(&mut buff) {
                Ok(_) => {
                    if buff != PING_BYTES {
                        log::error!("{addr:?} did not send 'PING'. Got {buff:?}");
                        break;
                    }
                }
                Err(e) => {
                    log::error!("{addr:?}: ping failed: {e}");
                    break;
                }
            }
        }
    });
}

async fn server(config: &Config) {
    if let Some(server) = config.server() {
        let host_uri = format!("{}:{}", server.host(), server.port());
        let listener = TcpListener::bind(host_uri).expect("BIND FAILED");
        let key = server.key().as_bytes();
        let read_timeout = server.read_timeout();
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    let _ = socket.set_read_timeout(Some(Duration::from_secs(*read_timeout)));
                    if check_key(key.to_vec(), &mut socket, addr).await.is_ok() {
                        listen(socket, addr).await;
                    }
                }
                Err(e) => log::error!("Couldn't connect to client: {e:?}"),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::load_from_arg();
    connect_to_peers(&config).await;
    server(&config).await;

    if config.server().is_none() {
        loop {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        }
    }
}