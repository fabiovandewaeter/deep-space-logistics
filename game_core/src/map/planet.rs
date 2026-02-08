// game_core/src/map/planet.rs
use hecs::{Bundle, Entity, World};
use serde::{Deserialize, Serialize};

use crate::map::{
    node::Node,
    stellar_system::{self, StellarSystem},
};

#[derive(Debug)]
pub struct Atmosphere {
    pub pressure: f32,
    // TODO: add test to make sure sum of purcents are equals to 100.0
    pub composition: Vec<(GasType, f32)>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GasType {
    pub name: String,
}
impl GasType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Planet {
    pub name: String,

    pub stellar_system: Entity,
    pub regions: Vec<Entity>,
}
/// can add an Atmosphere
#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub node: Node,
}

pub fn spawn_default_planet(world: &mut World, stellar_system_ent: Entity, name: &str) -> Entity {
    let planet_ent = world.spawn(PlanetBundle {
        planet: Planet {
            name: name.to_string(),
            stellar_system: stellar_system_ent,
            regions: Vec::new(),
        },
        node: Node::default(),
    });

    if let Ok(mut stellar_system) = world.get::<&mut StellarSystem>(stellar_system_ent) {
        stellar_system.celestial_objects.push(planet_ent.clone());
    }

    planet_ent
}
