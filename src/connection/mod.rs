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

impl Connection {
    pub(crate) fn peer(&self) -> &Peer {
        match self {
            Connection::NotConnected(nc) => &nc.peer,
            Connection::Connected(c) => &c.peer,
        }
    }
}
