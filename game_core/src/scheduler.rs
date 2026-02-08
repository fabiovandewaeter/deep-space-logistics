// game_core/src/scheduler.rs
use hecs::*;

use crate::{
    game_data::GameData,
    map::{machine::sys_process_machine, transport::sys_move_transport},
};

pub fn scheduler(world: &mut World, game_data: &GameData) {
    sys_process_machine(world);
    sys_move_transport(world);
}
