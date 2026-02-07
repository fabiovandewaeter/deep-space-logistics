// game_core/src/native.rs
use std::fs;

use crate::{
    game::Game,
    game_data::{GameData, Recipes, TerrainTypes, TransportTypes},
};

pub fn run_native() {
    let game_data = load_game_data();
    let mut game = Game::new(game_data);

    loop {
        game.step_world();
    }
}

pub fn load_game_data() -> GameData {
    let json_content = fs::read_to_string("./data/recipes.json").expect("Couldn't read JSON files");
    let recipes: Recipes = serde_json::from_str(&json_content).expect("Invalid JSON");

    let json_content =
        fs::read_to_string("./data/terrain_types.json").expect("Couldn't read JSON files");
    let terrain_types: TerrainTypes = serde_json::from_str(&json_content).expect("Invalid JSON");

    let json_content =
        fs::read_to_string("./data/transport_types.json").expect("Couldn't read JSON files");
    let transport_types: TransportTypes =
        serde_json::from_str(&json_content).expect("Invalid JSON");

    GameData {
        recipes,
        terrain_types,
        transport_types,
    }
}
