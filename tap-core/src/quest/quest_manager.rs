use super::{Quest, QuestId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QuestManager {
    pub quest_list: HashMap<QuestId, Quest>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quest_list: HashMap::new(),
        }
    }

    pub fn register_quest(&mut self, quest: Quest) {
        self.quest_list.insert(quest.id, quest);
    }

    pub fn get_quest(&self, id: &QuestId) -> Option<&Quest> {
        self.quest_list.get(id)
    }
}
