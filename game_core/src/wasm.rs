// game_core/src/wasm.rs
use std::cell::RefCell;

use crate::{create_world, snapshot, step_world};

use hecs::World;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsValue, prelude::*};

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(create_world());
}

#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();
    // world already created in thread_local! part
}

#[wasm_bindgen]
pub fn step() -> JsValue {
    WORLD.with(|w| {
        let mut world = w.borrow_mut();
        step_world(&mut world);
        let snap = snapshot(&world);
        to_value(&snap).expect("Failed to serialize snapshot")
    })
}
