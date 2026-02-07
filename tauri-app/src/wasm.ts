// tauri-app/src/lib/wasm.ts
import { writable } from "svelte/store";
import init, { WasmGameHandle } from "./pkg/game_core.js";
import wasmUrl from "./pkg/game_core_bg.wasm?url";

export const gameData = writable<any>(null);
export const gameSnapshot = writable<Array<{ a: number; b: number }>>([]);

let game: WasmGameHandle;
let initialized = false;

export async function initGame() {
    if (initialized) return;

    await init(wasmUrl);

    const [recipes, terrain_types, transport_types, gas_types] = await Promise.all([
        fetch("./data/recipes.json").then(r => r.json()),
        fetch("./data/terrain_types.json").then(r => r.json()),
        fetch("./data/transport_types.json").then(r => r.json()),
        fetch("./data/gas_types.json").then(r => r.json()),
    ]);

    const combined = { recipes, terrain_types, transport_types, gas_types };

    game = new WasmGameHandle(combined);
    gameData.set(game.game_data());

    initialized = true;

    gameLoop();
}

function gameLoop() {
    const snapshot = game.step_world();
    gameSnapshot.set(snapshot);
    requestAnimationFrame(gameLoop);
}
