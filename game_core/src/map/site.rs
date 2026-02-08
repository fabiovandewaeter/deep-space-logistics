// game_core/src/map/site.rs
use hecs::{Bundle, Entity};
use serde::{Deserialize, Serialize};

use crate::map::node::Node;

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub base_production: u32,
    pub terrain_type: TerrainType,
    pub region: Entity,
    pub planet: Entity,
}
#[derive(Bundle)]
pub struct SiteBundle {
    pub site: Site,
    pub node: Node,
}

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
