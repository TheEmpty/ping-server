mod connected;
mod not_connected;
mod peer;

pub(crate) use connected::{Connected, PING_BYTES};
pub(crate) use not_connected::NotConnected;
pub(crate) use peer::Peer;

#[derive(Debug)]
pub(crate) enum Connection {
    NotConnected(NotConnected),
    Connected(Connected),
}
