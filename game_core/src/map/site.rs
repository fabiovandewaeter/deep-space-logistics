// game_core/src/map/site.rs
use hecs::{Bundle, Entity, World};

use crate::map::{node::Node, region::Region, terrain::TerrainType};

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub terrain_type: TerrainType,
    /// Machine etc.
    pub structure: Option<Entity>,

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
            structure: None,
            region,
        },
        node: Node::default(),
    });

    if let Ok(mut region) = world.get::<&mut Region>(region) {
        region.sites.push(site_ent.clone());
    }

    site_ent
}

/// returns the previous structure
pub fn add_structure_to_site(
    world: &mut World,
    site_ent: Entity,
    structure_ent: Entity,
) -> Option<Entity> {
    let mut site = world.get::<&mut Site>(site_ent).unwrap();

    let previous_structure_ent = site.structure;
    site.structure = Some(structure_ent);

    previous_structure_ent
}
