// game_core/src/game_data.rs
use serde::{Deserialize, Serialize};

/// read from json files
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameData {
    pub recipes: Recipes,
    pub terrain_types: TerrainTypes,
    pub transport_types: TransportTypes,
}

// terrain types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerrainTypes {
    pub types: Vec<TerrainType>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerrainType {
    pub name: String,
}

// transport type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransportTypes {
    pub types: Vec<TransportType>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransportType {
    pub name: String,
    pub weight_capacity: f64,
    pub volume_capacity: f64,
    pub capabilities: Vec<TerrainType>,
}

// recipes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipes {
    pub recipes: Vec<Recipe>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub inputs: Vec<ItemStack>,
    pub outputs: Vec<ItemStack>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemStack {
    pub item_id: String,
    pub quantity: u32,
}
