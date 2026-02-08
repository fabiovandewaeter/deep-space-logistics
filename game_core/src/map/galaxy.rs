// game_core/src/map/galxy.rs
use hecs::{Bundle, Entity, World};

use crate::map::node::Node;

#[derive(Debug)]
pub struct Galaxy {
    pub name: String,

    pub stellar_systems: Vec<Entity>,
}
#[derive(Bundle)]
pub struct GalaxyBundle {
    pub galaxy: Galaxy,
    pub node: Node,
}

pub fn spawn_default_galaxy(world: &mut World, name: &str) -> Entity {
    let galaxy_ent = world.spawn(GalaxyBundle {
        galaxy: Galaxy {
            name: name.to_string(),
            stellar_systems: Vec::new(),
        },
        node: Node::default(),
    });

    galaxy_ent
}
