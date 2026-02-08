// game_core/game.rs
use hecs::{ComponentError, ComponentRef, Entity, World};
use serde::Serialize;

use crate::{game_data::GameData, scheduler};

pub struct Game {
    world: World,
    game_data: GameData,
}

impl Game {
    pub fn new(game_data: GameData) -> Self {
        let world = World::new();
        let mut game = Self { world, game_data };

        game.spawn_entities();

        game
    }

    pub fn step_world(&mut self) {
        scheduler(&mut self.world, &self.game_data);
    }

    pub fn spawn_entities(&mut self) {
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
        self.world.spawn((0i32, 0u32));
    }

    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn game_data(&self) -> &GameData {
        &self.game_data
    }

    pub fn get<'a, T: ComponentRef<'a>>(
        &'a self,
        entity: Entity,
    ) -> Result<T::Ref, ComponentError> {
        self.world.get::<T>(entity)
    }

    /// TODO: create snapshot functions for smaller part of the world
    pub fn full_snapshot(&self) -> Vec<CompSnapshot> {
        self.world
            .query::<(&i32, &u32)>()
            .iter()
            .map(|(a, b)| CompSnapshot { a: *a, b: *b })
            .collect()
    }
}

#[derive(Serialize)]
pub struct CompSnapshot {
    a: i32,
    b: u32,
}
