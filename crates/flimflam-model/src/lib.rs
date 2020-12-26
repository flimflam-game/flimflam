use serde::{Deserialize, Serialize};
use ultraviolet::Vec2;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    PlayerMoved(Vec2),
}
