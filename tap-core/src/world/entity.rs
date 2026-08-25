use crate::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NpcId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Npc {
    id: NpcId,
    name: String,
    description: String,
    text: Vec<String>,
    state: Player,
}
