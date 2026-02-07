// game_core/src/map/map.rs
use hecs::{Entity, World};

use crate::game_data::TerrainType;

/// marker for Site, Planet etc.
#[derive(Debug, Default)]
pub struct Node {
    /// vec of (destination Node, NodeConnection)
    pub neighbor_connections: Vec<(Entity, Entity)>,
}

/// bidirectional
#[derive(Debug)]
pub struct NodeConnection {
    pub node_a: Entity,
    pub node_b: Entity,
    pub distance: f32,
    pub terrain_type: TerrainType,
}

/// spawns a NodeConnection and adds it to NodeNeighbors of both nodes
pub fn connect_nodes(
    world: &mut World,
    node_a: Entity,
    node_b: Entity,
    distance: f32,
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
        game_data::TerrainType,
        load_game_data,
        map::{
            Node, connect_nodes,
            planet::{Atmosphere, GasType, Planet, PlanetBundle, Site, SiteBundle},
        },
    };

    #[test]
    fn test_connect_nodes_make_two_nodes_connected_bidirectionally() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let earth = game.world_mut().spawn((
            PlanetBundle {
                planet: Planet {
                    name: "Earth".to_string(),
                    total_score: 0,
                },
                node: Node::default(),
            },
            Atmosphere {
                pressure: 0.,
                composition: vec![(GasType::new("oxygen"), 100.0)],
            },
        ));

        let paris = game.world_mut().spawn(SiteBundle {
            site: Site {
                name: "Paris".to_string(),
                base_production: 10,
                terrain_type: TerrainType::new("land"),
                planet: earth,
            },
            node: Node::default(),
        });

        let london = game.world_mut().spawn(SiteBundle {
            site: Site {
                name: "London".to_string(),
                base_production: 10,
                terrain_type: TerrainType::new("land"),
                planet: earth,
            },
            node: Node::default(),
        });

        {
            let paris_node = game.world_mut().get::<&Node>(paris).unwrap();
            assert_eq!(paris_node.neighbor_connections.len(), 0);
        }
        {
            let london_node = game.world_mut().get::<&Node>(london).unwrap();
            assert_eq!(london_node.neighbor_connections.len(), 0);
        }

        connect_nodes(
            game.world_mut(),
            paris,
            london,
            100.0,
            TerrainType::new("water"),
        );

        {
            let paris_node = game.world_mut().get::<&Node>(paris).unwrap();
            assert_eq!(paris_node.neighbor_connections.len(), 1);
            assert_eq!(paris_node.neighbor_connections[0].0, london);
        }
        {
            let london_node = game.world_mut().get::<&Node>(london).unwrap();
            assert_eq!(london_node.neighbor_connections.len(), 1);
            assert_eq!(london_node.neighbor_connections[0].0, paris);
        }
    }
}
