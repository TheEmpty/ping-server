use crate::{config::Config, connection::PING_BYTES};
use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

async fn check_key(key: Vec<u8>, socket: &mut TcpStream, addr: SocketAddr) -> Result<(), ()> {
    log::trace!("[{addr:?}] Waiting for key.");
    let mut sent_key = vec![0; key.len()];

    match socket.read_exact(&mut sent_key) {
        Ok(_) => {
            if sent_key != key {
                log::error!("[{addr:?}] Did not send the correct key.");
                Err(())
            } else {
                log::trace!("[{addr:?}] Recieved the correct key.");
                Ok(())
            }
        }
        Err(e) => {
            log::error!("[{addr:?}] key check: {e}");
            Err(())
        }
    }
}

async fn listen(mut socket: TcpStream, addr: SocketAddr) {
    tokio::spawn(async move {
        let mut buff = vec![0; 4];
        loop {
            log::trace!("[{addr:?}] Waiting for {} bytes.", buff.len());
            match socket.read_exact(&mut buff) {
                Ok(_) => {
                    if buff != PING_BYTES {
                        log::error!("[{addr:?}] did not send 'PING'. Got {buff:?}");
                        break;
                    }
                }
                Err(e) => {
                    log::error!("[{addr:?}] ping failed: {e}");
                    break;
                }
            }
        }
    });
}

pub(crate) async fn start(config: &Config) {
    if let Some(server) = config.server() {
        let host_uri = format!("{}:{}", server.host(), server.port());
        let listener = TcpListener::bind(host_uri).expect("BIND FAILED");
        let key = server.key().as_bytes();
        let read_timeout = server.read_timeout();
        loop {
            // TODO: some form of rate limiting on accepts
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
