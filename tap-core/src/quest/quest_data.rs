use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct QuestId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Quest {
    pub id: QuestId,
    pub name: String,
    pub description: String,
}

impl Quest {
    pub fn new(id: QuestId, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}

impl Default for Quest {
    fn default() -> Self {
        Self::new(QuestId::default(), "Blank Quest", "A blank quest")
    }
}
