use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TerrainType {
    pub name: String,
}
impl TerrainType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
