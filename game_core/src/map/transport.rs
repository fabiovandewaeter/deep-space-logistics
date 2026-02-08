// game_core/src/map/transport.rs
use hecs::{Entity, World};
use serde::{Deserialize, Serialize};

use crate::map::{node::NodeConnection, terrain::TerrainType};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransportType {
    pub name: String,
    pub weight_capacity: f64,
    pub volume_capacity: f64,
    pub allowed_terrains: Vec<TerrainType>,
    pub speed: u32,
}

#[derive(Debug, PartialEq)]
pub enum Position {
    AtNode(Entity),
    OnConnection {
        connection: Entity,
        from: Entity,
        to: Entity,
        /// distance from the start
        progress: u32,
    },
}

#[derive(Debug)]
pub struct Transport {
    pub name: String,
    pub transport_type: TransportType,
    pub position: Position,
}
#[derive(Debug, PartialEq)]
pub enum TraverseError {
    NotOnSameNode,
    IncompatibleTerrain,
    AlreadyOnConnection,
}
impl Transport {
    pub fn can_start_traverse(&self, world: &World, connection_entity: Entity) -> bool {
        let connection = world.get::<&NodeConnection>(connection_entity).unwrap();
        self.try_traverse_check(&connection).is_ok()
    }

    fn try_traverse_check(&self, connection: &NodeConnection) -> Result<(), TraverseError> {
        // must be at node, and that node must be one of connection.node_a or connection.node_b
        let current_node = match self.position {
            Position::AtNode(entity) => entity,
            _ => return Err(TraverseError::AlreadyOnConnection),
        };

        if current_node != connection.node_a && current_node != connection.node_b {
            return Err(TraverseError::NotOnSameNode);
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

    pub fn start_traverse(
        &mut self,
        world: &World,
        connection_entity: Entity,
    ) -> Result<(), TraverseError> {
        let connection = world.get::<&NodeConnection>(connection_entity).unwrap();
        self.try_traverse_check(&connection)?;

        let (from, to) = match self.position {
            Position::AtNode(n) if n == connection.node_a => (connection.node_a, connection.node_b),
            Position::AtNode(n) if n == connection.node_b => (connection.node_b, connection.node_a),
            _ => unreachable!(),
        };

        self.position = Position::OnConnection {
            connection: connection_entity,
            from,
            to,
            progress: 0,
        };

        Ok(())
    }

    pub fn update_traverse(&mut self, world: &World) {
        let distance_to_move = self.get_speed();

        if let Position::OnConnection {
            connection,
            from,
            to,
            progress,
        } = &mut self.position
        {
            let connection = world.get::<&NodeConnection>(*connection).unwrap();

            *progress = progress.saturating_add(distance_to_move);

            if *progress >= connection.distance {
                self.position = Position::AtNode(*to);
            }
        }
    }

    pub fn get_speed(&self) -> u32 {
        self.transport_type.speed
    }
}

pub fn sys_move_transport(world: &mut World) {
    for transport in &mut world.query::<&mut Transport>() {
        transport.update_traverse(world);
    }
}

#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{
        game::Game,
        load_game_data,
        map::{
            galaxy::spawn_default_galaxy,
            node::{Node, NodeConnection, connect_nodes},
            planet::{Atmosphere, GasType, spawn_default_planet},
            region::spawn_default_region,
            site::spawn_default_site,
            stellar_system::spawn_default_stellar_system,
            transport::{Position, Transport, TraverseError},
        },
    };

    #[test]
    fn test_transport_can_travel() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let world = game.world_mut();

        let galaxy_ent = spawn_default_galaxy(world, "Galaxy");
        let stellar_system_ent = spawn_default_stellar_system(world, galaxy_ent, "Stellar system");
        let earth_ent = spawn_default_planet(world, stellar_system_ent, "Earth");
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
        let marseille_ent = spawn_default_site(
            world,
            north_ent,
            "Marseille",
            game_data.get_terrain_type("land"),
        );
        connect_nodes(
            game.world_mut(),
            paris_ent,
            marseille_ent,
            100,
            game_data.get_terrain_type("land"),
        );

        let transport = game.spawn((Transport {
            name: "T".to_string(),
            transport_type: game_data.get_transport_type("truck"),
            position: Position::AtNode(paris_ent),
        },));

        {
            let transport_data = game.get::<&Transport>(transport).unwrap();
            assert_eq!(transport_data.position, Position::AtNode(paris_ent));
        }

        let paris_node = game.get::<&Node>(paris_ent).unwrap();
        let neighbor_connection = paris_node.neighbor_connections[0].1;
        drop(paris_node);
        let mut transport_data = game.get::<&mut Transport>(transport).unwrap();
        let result = transport_data.start_traverse(game.world(), neighbor_connection);

        // traverse started
        assert!(result.is_ok());
        assert!(matches!(
            transport_data.position,
            Position::OnConnection { .. }
        ));

        let connection = game.get::<&NodeConnection>(neighbor_connection).unwrap();
        let distance = connection.distance;
        drop(connection);
        let transport_speed = transport_data.get_speed();
        let number_steps = distance / transport_speed;
        drop(transport_data);

        for _ in 0..number_steps {
            game.step_world();
        }

        let world = game.world_mut();
        let transport_data = world.get::<&Transport>(transport).unwrap();
        assert!(matches!(transport_data.position, Position::AtNode(..)));
    }

    #[test]
    fn test_transport_cant_travel_on_incompatible_terrain_type() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let game_data = game.game_data();
        let world = game.world_mut();

        let galaxy_ent = spawn_default_galaxy(world, "Galaxy");
        let stellar_system_ent = spawn_default_stellar_system(world, galaxy_ent, "Stellar system");
        let earth_ent = spawn_default_planet(world, stellar_system_ent, "Earth");
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
            "London",
            game_data.get_terrain_type("land"),
        );
        connect_nodes(
            game.world_mut(),
            paris_ent,
            london_ent,
            100,
            game_data.get_terrain_type("water"),
        );

        let transport = game.spawn((Transport {
            name: "T".to_string(),
            transport_type: game_data.get_transport_type("truck"),
            position: Position::AtNode(paris_ent),
        },));

        {
            let transport_data = game.get::<&Transport>(transport).unwrap();
            assert_eq!(transport_data.position, Position::AtNode(paris_ent));
        }

        let paris_node = game.get::<&Node>(paris_ent).unwrap();
        let neighbor_connection = paris_node.neighbor_connections[0].1;
        let mut transport_data = game.get::<&mut Transport>(transport).unwrap();
        let result = transport_data.start_traverse(game.world(), neighbor_connection);

        assert!(matches!(result, Err(TraverseError::IncompatibleTerrain)));
        assert_eq!(transport_data.position, Position::AtNode(paris_ent));
    }
}
