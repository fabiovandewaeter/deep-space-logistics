// game_core/src/scheduler.rs
use hecs::*;

use crate::{
    game_data::GameData,
    planet::{Leader, Planet, Site},
};

pub fn scheduler(world: &mut World, game_data: &GameData) {
    sys_increment(world);
    sys_add_produciton(world);
}

fn sys_increment(world: &mut World) {
    for (a, b) in world.query_mut::<(&mut i32, &mut u32)>() {
        *a += 1;
        if (u32::MAX - *a as u32) < *b {
            *b = 0;
        }
        *b += *a as u32;
    }
}

fn sys_add_produciton(world: &mut World) {
    for (entity, loc) in world.query::<(Entity, &Site)>().iter() {
        let maybe_leader = world.get::<&Leader>(entity).ok();
        let mut planet = world.get::<&mut Planet>(loc.planet).unwrap();

        planet.total_score = match maybe_leader {
            Some(leader) => {
                (loc.base_production as f64 * (1.0 + (leader.bonus_percent as f64 / 100.0))) as u64
            }
            None => (loc.base_production) as u64,
        };
    }
}

#[cfg(feature = "native")]
#[cfg(test)]
mod tests {
    use crate::{game::Game, load_game_data};

    use super::*;

    #[test]
    fn test_setp_world_increments_without_snapshot() {
        let game_data = load_game_data();
        let mut game = Game::new(game_data);

        game.step_world();

        for (a, _) in game.world().query::<(&i32, &u32)>().iter() {
            assert_eq!(*a, 1);
        }
    }
}
