// game_core/src/map/transport.rs
use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::map::{NodeConnection, planet::TerrainType};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransportType {
    pub name: String,
    pub weight_capacity: f64,
    pub volume_capacity: f64,
    pub allowed_terrains: Vec<TerrainType>,
}

#[derive(Debug)]
pub struct Transport {
    pub name: String,
    pub transport_type: TransportType,
    pub current_node: Entity,
}
#[derive(Debug, PartialEq)]
pub enum TraverseError {
    NotOnConnection,
    IncompatibleTerrain,
}
impl Transport {
    pub fn can_traverse(&self, connection: &NodeConnection) -> bool {
        self.try_traverse_check(connection).is_ok()
    }

    fn try_traverse_check(&self, connection: &NodeConnection) -> Result<(), TraverseError> {
        if self.current_node != connection.node_a && self.current_node != connection.node_b {
            return Err(TraverseError::NotOnConnection);
        }

        if !self
            .transport_type
            .allowed_terrains
            .contains(&connection.terrain_type)
        {
            return Err(TraverseError::IncompatibleTerrain);
        }

        Ok(())
    }

    pub fn try_traverse(&mut self, connection: &NodeConnection) -> Result<(), TraverseError> {
        self.try_traverse_check(connection)?;

        self.current_node = if self.current_node == connection.node_a {
            connection.node_b
        } else {
            connection.node_a
        };

        Ok(())
    }
}

#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{
        game::Game,
        load_game_data,
        map::{
            Node, NodeConnection, connect_nodes,
            planet::{Planet, PlanetBundle, Site, SiteBundle},
            transport::{Transport, TraverseError},
        },
    };

    #[test]
    fn test_transport_can_travel() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let terrain_type_land = game.game_data().get_terrain_type("land");
        let transport_type = game.game_data().get_transport_type("truck");

        let world = game.world_mut();

        let earth = world.spawn(PlanetBundle {
            planet: Planet {
                name: "Earth".to_string(),
                total_score: 0,
            },
            node: Node::default(),
        });
        let paris = world.spawn(SiteBundle {
            site: Site {
                name: "Paris".to_string(),
                base_production: 10,
                terrain_type: terrain_type_land.clone(),
                planet: earth,
            },
            node: Node::default(),
        });
        let marseille = world.spawn(SiteBundle {
            site: Site {
                name: "Marseille".to_string(),
                base_production: 10,
                terrain_type: terrain_type_land.clone(),
                planet: earth,
            },
            node: Node::default(),
        });
        connect_nodes(world, paris, marseille, 100.0, terrain_type_land);

        let transport = world.spawn((Transport {
            name: "T".to_string(),
            transport_type,
            current_node: paris,
        },));

        {
            let transport_data = world.get::<&Transport>(transport).unwrap();
            assert_eq!(transport_data.current_node, paris);
        }

        let paris_node = world.get::<&Node>(paris).unwrap();
        let neighbor_connection = paris_node.neighbor_connections[0].1;
        let connection = world.get::<&NodeConnection>(neighbor_connection).unwrap();
        let mut transport_data = world.get::<&mut Transport>(transport).unwrap();
        let result = transport_data.try_traverse(&connection);

        assert!(result.is_ok());
        assert_eq!(transport_data.current_node, marseille);
    }

    #[test]
    fn test_transport_cant_travel_on_incompatible_terrain_type() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let terrain_type_land = game_data.get_terrain_type("land");
        let terrain_type_water = game_data.get_terrain_type("water");
        let transport_type = game_data.get_transport_type("truck");
        // drop(game_data);

        let world = game.world_mut();

        let earth = world.spawn(PlanetBundle {
            planet: Planet {
                name: "Earth".to_string(),
                total_score: 0,
            },
            node: Node::default(),
        });
        let paris = world.spawn(SiteBundle {
            site: Site {
                name: "Paris".to_string(),
                base_production: 10,
                terrain_type: terrain_type_land.clone(),
                planet: earth,
            },
            node: Node::default(),
        });
        let london = world.spawn(SiteBundle {
            site: Site {
                name: "London".to_string(),
                base_production: 10,
                terrain_type: terrain_type_land,
                planet: earth,
            },
            node: Node::default(),
        });
        connect_nodes(world, paris, london, 100.0, terrain_type_water);

        let transport = world.spawn((Transport {
            name: "T".to_string(),
            transport_type,
            current_node: paris,
        },));

        {
            let transport_data = world.get::<&Transport>(transport).unwrap();
            assert_eq!(transport_data.current_node, paris);
        }

        let paris_node = world.get::<&Node>(paris).unwrap();
        let neighbor_connection = paris_node.neighbor_connections[0].1;
        let connection = world.get::<&NodeConnection>(neighbor_connection).unwrap();
        let mut transport_data = world.get::<&mut Transport>(transport).unwrap();
        let result = transport_data.try_traverse(&connection);

        assert!(matches!(result, Err(TraverseError::IncompatibleTerrain)));
        assert_eq!(transport_data.current_node, paris);
    }
}
