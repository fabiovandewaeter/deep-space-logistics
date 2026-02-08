// game_core/src/map/map.rs
use hecs::{Entity, World};

use crate::map::terrain::TerrainType;

/// marker for Site, Planet etc.
#[derive(Debug, Default)]
pub struct Node {
    /// Vec(destination Node, NodeConnection)
    pub neighbor_connections: Vec<(Entity, Entity)>,
}

/// bidirectional
#[derive(Debug)]
pub struct NodeConnection {
    pub node_a: Entity,
    pub node_b: Entity,
    pub distance: u32,
    pub terrain_type: TerrainType,
}

/// spawns a NodeConnection and adds it to NodeNeighbors of both nodes
pub fn connect_nodes(
    world: &mut World,
    node_a: Entity,
    node_b: Entity,
    distance: u32,
    terrain_type: TerrainType,
) {
    let node_connection = world.spawn((NodeConnection {
        node_a,
        node_b,
        distance,
        terrain_type,
    },));

    {
        let mut node_a_data = world.get::<&mut Node>(node_a).unwrap();
        node_a_data
            .neighbor_connections
            .push((node_b, node_connection));
    }

    let mut node_b_data = world.get::<&mut Node>(node_b).unwrap();
    node_b_data
        .neighbor_connections
        .push((node_a, node_connection));
}

// peut pas aller si transporter peut pas aller sur terrain_type du NodeConnection ou du Node de destination
#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{
        game::Game,
        load_game_data,
        map::{
            node::{Node, connect_nodes},
            planet::{Atmosphere, GasType, spawn_default_planet},
            region::spawn_default_region,
            site::spawn_default_site,
        },
    };

    #[test]
    fn test_connect_nodes_make_two_nodes_connected_bidirectionally() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let world = game.world_mut();

        let earth_ent = spawn_default_planet(world, "Earth");
        world
            .insert(
                earth_ent,
                (Atmosphere {
                    pressure: 0.,
                    composition: vec![(GasType::new("oxygen"), 100.0)],
                },),
            )
            .unwrap();
        let north_ent = spawn_default_region(world, earth_ent, "North");

        let paris_ent = spawn_default_site(
            world,
            north_ent,
            "Paris",
            game_data.get_terrain_type("land"),
        );
        let london_ent = spawn_default_site(
            world,
            north_ent,
            "Paris",
            game_data.get_terrain_type("land"),
        );

        {
            let paris_node = game.world_mut().get::<&Node>(paris_ent).unwrap();
            assert_eq!(paris_node.neighbor_connections.len(), 0);
        }
        {
            let london_node = game.world_mut().get::<&Node>(london_ent).unwrap();
            assert_eq!(london_node.neighbor_connections.len(), 0);
        }

        connect_nodes(
            game.world_mut(),
            paris_ent,
            london_ent,
            100,
            game_data.get_terrain_type("water"),
        );

        {
            let paris_node = game.world_mut().get::<&Node>(paris_ent).unwrap();
            assert_eq!(paris_node.neighbor_connections.len(), 1);
            assert_eq!(paris_node.neighbor_connections[0].0, london_ent);
        }
        {
            let london_node = game.world_mut().get::<&Node>(london_ent).unwrap();
            assert_eq!(london_node.neighbor_connections.len(), 1);
            assert_eq!(london_node.neighbor_connections[0].0, paris_ent);
        }
    }
}
