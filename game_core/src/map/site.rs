// game_core/src/map/site.rs
use hecs::{Bundle, Entity, World};

use crate::map::{node::Node, region::Region, terrain::TerrainType};

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub terrain_type: TerrainType,

    pub region: Entity,
}
#[derive(Bundle)]
pub struct SiteBundle {
    pub site: Site,
    pub node: Node,
}

pub fn spawn_default_site(
    world: &mut World,
    region: Entity,
    name: &str,
    terrain_type: TerrainType,
) -> Entity {
    let site_ent = world.spawn(SiteBundle {
        site: Site {
            name: name.to_string(),
            terrain_type,
            region,
        },
        node: Node::default(),
    });

    if let Ok(mut region) = world.get::<&mut Region>(region) {
        region.sites.push(site_ent.clone());
    }

    site_ent
}
