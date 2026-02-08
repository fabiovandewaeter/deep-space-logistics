// game_core/src/map/planet.rs
use hecs::{Bundle, Entity, World};
use serde::{Deserialize, Serialize};

use crate::map::node::Node;

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

    pub regions: Vec<Entity>,
}
/// can add an Atmosphere
#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub node: Node,
}

pub fn spawn_default_planet(world: &mut World, name: &str) -> Entity {
    let planet_ent = world.spawn(PlanetBundle {
        planet: Planet {
            name: name.to_string(),
            regions: Vec::new(),
        },
        node: Node::default(),
    });

    // if let Ok(mut galaxy) = world.get::<&mut Galaxy>(galaxy) {
    //     galaxy.celestial_bodies.push(planet_ent.clone());
    // }

    planet_ent
}
