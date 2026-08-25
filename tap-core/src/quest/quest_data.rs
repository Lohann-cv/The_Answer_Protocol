use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Quest {
    id: QuestId,
    name: String,
    description: String,
}
