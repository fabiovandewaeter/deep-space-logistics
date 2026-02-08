// game_core/src/scheduler.rs
use hecs::*;

use crate::{
    game_data::GameData,
    map::{
        machine::sys_process_machine,
        planet::{Leader, Planet},
        site::Site,
        transport::sys_move_transport,
    },
};

pub fn scheduler(world: &mut World, game_data: &GameData) {
    sys_add_produciton(world);
    sys_process_machine(world);
    sys_move_transport(world);
}

fn sys_add_produciton(world: &mut World) {
    for (entity, site) in world.query::<(Entity, &Site)>().iter() {
        let maybe_leader = world.get::<&Leader>(entity).ok();
        let mut planet = world.get::<&mut Planet>(site.planet).unwrap();

        planet.total_score = match maybe_leader {
            Some(leader) => {
                (site.base_production as f64 * (1.0 + (leader.bonus_percent as f64 / 100.0))) as u64
            }
            None => (site.base_production) as u64,
        };
    }
}
