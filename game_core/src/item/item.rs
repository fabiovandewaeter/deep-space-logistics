use std::collections::HashMap;

// game_core/src/item/item.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct ItemType {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemStack {
    pub item_type: ItemType,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Inventory {
    pub items: HashMap<ItemType, u32>,
    pub max_capacity: u32,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            max_capacity: 100,
        }
    }
}
impl Inventory {
    pub fn add(&mut self, item: ItemType, quantity: u32) {
        *self.items.entry(item).or_insert(0) += quantity;
    }

    pub fn has(&self, item: &ItemType, quantity: u32) -> bool {
        self.items.get(item).unwrap_or(&0) >= &quantity
    }

    pub fn remove(&mut self, item: &ItemType, quantity: u32) -> bool {
        if self.has(item, quantity) {
            *self.items.get_mut(item).unwrap() -= quantity;
            return true;
        }

        false
    }
}
