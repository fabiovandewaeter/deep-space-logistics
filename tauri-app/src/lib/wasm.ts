// tauri-app/src/lib/wasm.ts
import init, { step } from "../pkg/game_core.js";
import wasmUrl from "../pkg/game_core_bg.wasm?url";

let initialized = false;

export async function initWasm() {
    if (initialized) return;
    await init(wasmUrl);
    initialized = true;
}

export function stepOnce() {
    return step();
}
