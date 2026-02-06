// game_core/src/native.rs
use crate::create_world;

use hecs::World;

pub fn run_native() {
    let mut world = create_world();

    loop {
        use crate::step_world;

        step_world(&mut world);
        sys_print_test(&mut world);
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn sys_print_test(world: &mut World) {
    for (a, b) in world.query::<(&i32, &u32)>().iter() {
        if *a % 100_000 == 0 {
            println!("{} {}", a, b);
        }
    }
}
