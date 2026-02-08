// game_core/src/map/region.rs
use hecs::{Bundle, Entity};

use crate::map::node::Node;

#[derive(Debug)]
pub struct Region {
    pub name: String,
    pub planet: Entity,
}
#[derive(Bundle)]
pub struct RegionBundle {
    pub region: Region,
    pub node: Node,
}
