// game_core/src/item/recipe.rs
use serde::{Deserialize, Serialize};

use crate::item::ItemStack;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub inputs: Vec<ItemStack>,
    pub outputs: Vec<ItemStack>,
    /// number of ticks
    pub duration: u32,
}
