use crate::{config::Config, connection::PING_BYTES};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

const TRUE: &[u8] = "1".as_bytes();
const FALSE: &[u8] = "1".as_bytes();

async fn check_key(key: Vec<u8>, socket: &mut TcpStream, addr: SocketAddr) -> Result<(), ()> {
    log::trace!("[{addr:?}] Waiting for key.");
    let mut sent_key = vec![0; key.len()];

    match socket.read_exact(&mut sent_key) {
        Ok(_) => {
            if sent_key != key {
                log::error!("[{addr:?}] Did not send the correct key.");
                // we do not care if the result got through since we will drop connection.
                // this is just to let client log it.
                let _ = socket.write(FALSE);
                Err(())
            } else {
                log::trace!("[{addr:?}] Recieved the correct key.");
                //
                match socket.write(TRUE) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        log::error!("[{addr:?}] Failed to send TRUE after getting key. {e}");
                        Err(())
                    }
                }
            }
        }
        Err(e) => {
            log::error!("[{addr:?}] key check: {e}");
            // we do not care if the result got through since we will drop connection.
            // this is just to let client log it.
            let _ = socket.write(FALSE);
            Err(())
        }
    }
}

enum NameError {
    Io(std::io::Error),
    NoNull,
    Utf8(std::string::FromUtf8Error),
}

async fn get_name(socket: &mut TcpStream) -> Result<String, NameError> {
    let mut buff = vec![0; 100];
    socket.read_exact(&mut buff).map_err(NameError::Io)?;
    let first_null = buff
        .iter()
        .position(|x| *x == b'\0')
        .ok_or(NameError::NoNull)?;
    String::from_utf8(buff[0..first_null].to_vec()).map_err(NameError::Utf8)
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
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    if let Err(e) =
                        socket.set_read_timeout(Some(Duration::from_secs(*read_timeout)))
                    {
                        log::warn!("[{addr:?}] Failed to set read timeout: {e}");
                    }
                    if check_key(key.to_vec(), &mut socket, addr).await.is_ok() {
                        let _name = get_name(&mut socket).await;
                        match socket.write(&[*server.wait_seconds()]) {
                            Ok(_) => listen(socket, addr).await,
                            Err(e) => log::warn!("[{addr:?}] Could not send wait_seconds. {e}"),
                        }
                    }
                }
                Err(e) => log::error!("Couldn't connect to client: {e:?}"),
            }
            // Allow one new connection, success or failure, ever 30s.
            // Verrryy rudimentary prevention of attacks.
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }
}
