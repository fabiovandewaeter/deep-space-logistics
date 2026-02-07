// game_core/src/game_data.rs
use serde::{Deserialize, Serialize};

use crate::{item::recipe::Recipe, map::planet::GasType};

/// read from json files
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameData {
    pub recipes: Recipes,
    pub terrain_types: TerrainTypes,
    pub transport_types: TransportTypes,
    pub gas_types: GasTypes,
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
impl TerrainType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipes {
    pub recipes: Vec<Recipe>,
}

// gas type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GasTypes {
    pub types: Vec<GasType>,
}
