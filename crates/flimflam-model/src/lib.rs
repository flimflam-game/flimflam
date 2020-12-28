use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use ultraviolet::Vec2;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    PlayerUpdate(Client, Player),
    JoinGame(Client, Player),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub position: Vec2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Client {
    address: SocketAddr,
}

impl Client {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}
