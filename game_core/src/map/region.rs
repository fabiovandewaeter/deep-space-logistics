// game_core/src/map/region.rs
use hecs::{Bundle, Entity, World};

use crate::map::{node::Node, planet::Planet};

#[derive(Debug)]
pub struct Region {
    pub name: String,

    pub planet: Entity,
    pub sites: Vec<Entity>,
}
#[derive(Bundle)]
pub struct RegionBundle {
    pub region: Region,
    pub node: Node,
}

pub fn spawn_default_region(world: &mut World, planet: Entity, name: &str) -> Entity {
    let region_ent = world.spawn(RegionBundle {
        region: Region {
            name: name.to_string(),
            planet,
            sites: Vec::new(),
        },
        node: Node::default(),
    });

    if let Ok(mut planet) = world.get::<&mut Planet>(planet) {
        planet.regions.push(region_ent.clone());
    }

    region_ent
}
