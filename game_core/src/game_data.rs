// game_core/src/game_data.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    item::{ItemType, recipe::Recipe},
    map::{
        planet::{GasType, TerrainType},
        transport::TransportType,
    },
};

/// read from json files
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameData {
    pub recipes: Recipes,
    pub terrain_types: TerrainTypes,
    pub transport_types: TransportTypes,
    pub gas_types: GasTypes,
    pub item_types: ItemTypes,
}
impl GameData {
    pub fn get_recipe(&self, name: &str) -> Recipe {
        self.recipes.recipes.get(name).unwrap().clone()
    }

    pub fn get_terrain_type(&self, name: &str) -> TerrainType {
        self.terrain_types.types.get(name).unwrap().clone()
    }

    pub fn get_transport_type(&self, name: &str) -> TransportType {
        self.transport_types.types.get(name).unwrap().clone()
    }

    pub fn get_gas_type(&self, name: &str) -> GasType {
        self.gas_types.types.get(name).unwrap().clone()
    }

    pub fn get_item_type(&self, name: &str) -> ItemType {
        self.item_types.types.get(name).unwrap().clone()
    }
}

// recipes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipes {
    pub recipes: HashMap<String, Recipe>,
}

// terrain types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerrainTypes {
    pub types: HashMap<String, TerrainType>,
}

// transport type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransportTypes {
    pub types: HashMap<String, TransportType>,
}

// gas type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GasTypes {
    pub types: HashMap<String, GasType>,
}

// item type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemTypes {
    pub types: HashMap<String, ItemType>,
}
