use super::NpcId;
use crate::items::ItemId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
pub struct WorldId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
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
    pub exits: RoomExit,
    pub is_safe: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RoomExit {
    pub south: Option<RoomId>,
    pub north: Option<RoomId>,
    pub west: Option<RoomId>,
    pub east: Option<RoomId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WorldStock {
    worlds: HashMap<WorldId, WorldMap>,
    rooms: HashMap<RoomId, Room>,
}

impl WorldMap {
    pub fn new(
        id: WorldId,
        name: impl Into<String>,
        description: impl Into<String>,
        rooms: impl IntoIterator<Item = RoomId>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            rooms: rooms.into_iter().collect(),
        }
    }
}

impl Room {
    pub fn new(
        id: RoomId,
        name: impl Into<String>,
        description: impl Into<String>,
        items: impl IntoIterator<Item = ItemId>,
        npcs: impl IntoIterator<Item = NpcId>,
        exits: RoomExit,
        is_safe: bool,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            items: items.into_iter().collect(),
            npcs: npcs.into_iter().collect(),
            exits,
            is_safe,
        }
    }
}

impl WorldStock {
    pub fn new(
        worlds: impl IntoIterator<Item = WorldMap>,
        rooms: impl IntoIterator<Item = Room>,
    ) -> Self {
        Self {
            worlds: worlds.into_iter().map(|w| (w.id, w)).collect(),
            rooms: rooms.into_iter().map(|r| (r.id, r)).collect(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Option<Room> {
        self.rooms.insert(room.id, room)
    }

    pub fn remove_room(&mut self, id: RoomId) -> Option<Room> {
        self.rooms.remove(&id)
    }

    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
    }

    pub fn get_room_mut(&mut self, id: RoomId) -> Option<&mut Room> {
        self.rooms.get_mut(&id)
    }

    pub fn add_world(&mut self, world: WorldMap) -> Option<WorldMap> {
        self.worlds.insert(world.id, world)
    }

    pub fn remove_world(&mut self, id: WorldId) -> Option<WorldMap> {
        self.worlds.remove(&id)
    }

    pub fn get_world(&self, id: WorldId) -> Option<&WorldMap> {
        self.worlds.get(&id)
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        Self::new(WorldId::default(), "Blank World", "A blank world", [])
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new(
            RoomId::default(),
            "Blank Room",
            "A blank room",
            [],
            [],
            RoomExit::default(),
            false,
        )
    }
}
