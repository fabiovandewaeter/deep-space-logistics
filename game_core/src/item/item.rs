// game_core/src/item/item.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemStack {
    pub item_id: String,
    pub quantity: u32,
}
