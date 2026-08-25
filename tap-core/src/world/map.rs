use serde::{Deserialize, Serialize};
use crate::items::ItemId;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldMap {
    id: WorldId,
    name: str,
    description: str,
    rooms: Vector<RoomId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Room {
    id: RoomId,
    name: str,
    description: str,
    items: Vector<ItemId>,
    npcs: Vector<NPC>,
    quests: Vector<Quest>,
    exits: RoomExit,
    is_safe: bool,
    world_exit: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomExit {
    pub south: bool,
    pub north: bool,
    pub west: bool,
    pub east: bool,
}