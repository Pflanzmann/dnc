use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub rarity: String,
    pub description: String,
    pub flavor: String,
}

impl Item {
    pub fn new(name: String, kind: String, rarity: String, description: String, flavor: String) -> Self {
        Self { name, kind, rarity, description, flavor }
    }
}