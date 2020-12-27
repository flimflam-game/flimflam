use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use ultraviolet::Vec2;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    PlayerMoved(Vec2),
    JoinGame(Client),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Update {
    PlayerMoved(Vec2),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Client {
    uuid: Uuid,
    address: SocketAddr,
}

impl Client {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            address,
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}
