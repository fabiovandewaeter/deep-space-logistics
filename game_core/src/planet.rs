// game_core/src/planet.rs
use hecs::Entity;

#[derive(Debug)]
pub struct Planet {
    pub name: String,
    pub total_score: u64,
}
//
#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub base_production: u32,
    pub planet: Entity,
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
        // let world = game.world_mut();

        let pa = game.world_mut().spawn((Planet {
            name: "A".to_string(),
            total_score: 0,
        },));
        game.world_mut().spawn((Site {
            name: "LA".to_string(),
            base_production: 10,
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
