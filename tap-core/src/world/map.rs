use super::NpcId;
use crate::items::ItemId;
use crate::quest::QuestId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldMap {
    pub id: WorldId,
    pub name: String,
    pub description: String,
    pub rooms: Vec<RoomId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub description: String,
    pub items: Vec<ItemId>,
    pub npcs: Vec<NpcId>,
    pub quests: Vec<QuestId>,
    pub exits: RoomExit,
    pub is_safe: bool,
    pub world_exit: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomExit {
    pub south: bool,
    pub north: bool,
    pub west: bool,
    pub east: bool,
}
