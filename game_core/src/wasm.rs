// game_core/src/wasm.rs

use crate::game::Game;

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{JsValue, prelude::*};

#[wasm_bindgen]
pub struct WasmGameHandle {
    game: Game,
}

#[wasm_bindgen]
impl WasmGameHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(js_data: JsValue) -> WasmGameHandle {
        let game_data = from_value(js_data).expect("Invalid game data");
        let game = Game::new(game_data);

        WasmGameHandle { game }
    }

    pub fn step_world(&mut self) -> JsValue {
        self.game.step_world();
        let snap = self.game.full_snapshot();

        to_value(&snap).unwrap()
    }

    pub fn game_data(&self) -> JsValue {
        to_value(&self.game.game_data()).expect("Failed to serialize game data")
    }
}
