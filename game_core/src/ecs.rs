// game_core/src/ecs.rs
use hecs::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct CompSnapshot {
    a: i32,
    b: u32,
}

pub fn create_world() -> World {
    let mut world = World::new();

    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));
    world.spawn((0i32, 0u32));

    world
}

pub fn step_world(world: &mut World) {
    sys_increment(world);
}

pub fn snapshot(world: &World) -> Vec<CompSnapshot> {
    world
        .query::<(&i32, &u32)>()
        .iter()
        .map(|(a, b)| CompSnapshot { a: *a, b: *b })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setp_world_increments() {
        let mut world = create_world();
        step_world(&mut world);
        let snap = snapshot(&world);

        assert_eq!(snap[0].a, 1);
    }

    #[test]
    fn test_setp_world_increments_without_snapshot() {
        let mut world = create_world();
        step_world(&mut world);

        for (a, _) in world.query::<(&i32, &u32)>().iter() {
            assert_eq!(*a, 1);
        }
    }
}
