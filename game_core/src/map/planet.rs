// game_core/src/map/planet.rs
use hecs::{Bundle, Entity};
use serde::{Deserialize, Serialize};

use crate::{game_data::TerrainType, map::Node};

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
    pub total_score: u64,
}
/// can add an Atmosphere
#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub node: Node,
}

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub base_production: u32,
    pub terrain_type: TerrainType,
    pub planet: Entity,
}
#[derive(Bundle)]
pub struct SiteBundle {
    pub site: Site,
    pub node: Node,
}

#[derive(Debug)]
pub struct Leader {
    pub name: String,
    pub bonus_percent: u32,
}

#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{game::Game, load_game_data};

    use super::*;

    #[test]
    fn test_leader_bonus() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        let pa = game.world_mut().spawn((Planet {
            name: "A".to_string(),
            total_score: 0,
        },));
        game.world_mut().spawn((Site {
            name: "LA".to_string(),
            base_production: 10,
            terrain_type: TerrainType::new("land"),
            planet: pa,
        },));

        assert_eq!(game.world().get::<&Planet>(pa).unwrap().total_score, 0);

        game.step_world();

        assert_eq!(game.world().get::<&Planet>(pa).unwrap().total_score, 10);

        let pb = game.world_mut().spawn((Planet {
            name: "B".to_string(),
            total_score: 0,
        },));
        game.world_mut().spawn((
            Site {
                name: "LB".to_string(),
                base_production: 10,
                terrain_type: TerrainType::new("land"),
                planet: pb,
            },
            Leader {
                name: "Boss".to_string(),
                bonus_percent: 10,
            },
        ));

        assert_eq!(game.world().get::<&Planet>(pb).unwrap().total_score, 0);

        game.step_world();

        assert_eq!(game.world().get::<&Planet>(pb).unwrap().total_score, 11);
    }
}
