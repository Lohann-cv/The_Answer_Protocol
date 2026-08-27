use crate::Player;
use crate::{Quest, QuestId, QuestManager};
use rand::prelude::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct NpcId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Npc {
    pub id: NpcId,
    pub name: String,
    pub description: String,
    pub text: Vec<String>,
    pub state: Player,
    pub quest: VecDeque<QuestId>,
}

impl Npc {
    pub fn new(
        id: NpcId,
        name: impl Into<String>,
        description: impl Into<String>,
        text: impl IntoIterator<Item = impl Into<String>>,
        state: Player,
        quest: impl IntoIterator<Item = QuestId>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            text: text.into_iter().map(|s| s.into()).collect(),
            state,
            quest: quest.into_iter().collect(),
        }
    }

    pub fn take_quest<'a>(&self, manager: &'a QuestManager) -> Option<&'a Quest> {
        if self.quest.is_empty() {
            return None;
        }
        let first_quest = self.quest.front()?;
        manager.get_quest(first_quest)
    }

    pub fn achieve_quest(&mut self) -> Option<QuestId> {
        self.quest.pop_front()
    }

    pub fn speak(&self) -> Option<&String> {
        let mut randome_value = rand::rng();
        if let Some(text) = self.text.choose(&mut randome_value) {
            Some(text)
        } else {
            None
        }
    }
}

impl Default for Npc {
    fn default() -> Self {
        Self::new(
            NpcId::default(),
            "Jhone Doe",
            "Your regular joe",
            ["Hey", "Hello", "Good morning"],
            Player::default(),
            [],
        )
    }
}
