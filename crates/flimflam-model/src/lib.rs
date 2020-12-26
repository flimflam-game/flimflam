use serde::{Deserialize, Serialize};
use ultraviolet::Vec2;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    PlayerMoved(Vec2),
    JoinGame(Client),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    uuid: Uuid,
}

impl Client {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}
