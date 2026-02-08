// game_core/src/map/stellar_system.rs
use hecs::{Bundle, Entity, World};

use crate::map::{galaxy::Galaxy, node::Node};

#[derive(Debug)]
pub struct StellarSystem {
    pub name: String,

    pub galaxy: Entity,
    /// Planet, Star etc.
    pub celestial_objects: Vec<Entity>,
}
#[derive(Bundle)]
pub struct StellarBundle {
    pub stellar_system: StellarSystem,
    pub node: Node,
}

#[derive(Bundle)]
pub struct CelestialObject;

pub fn spawn_default_stellar_system(world: &mut World, galaxy_ent: Entity, name: &str) -> Entity {
    let stellar_system_ent = world.spawn(StellarBundle {
        stellar_system: StellarSystem {
            name: name.to_string(),
            galaxy: galaxy_ent,
            celestial_objects: Vec::new(),
        },
        node: Node::default(),
    });

    if let Ok(mut galaxy) = world.get::<&mut Galaxy>(galaxy_ent) {
        galaxy.stellar_systems.push(stellar_system_ent.clone());
    }

    stellar_system_ent
}
