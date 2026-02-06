// game_core/src/lib.rs
mod ecs;

#[cfg(feature = "native")]
mod native;
#[cfg(feature = "wasm")]
mod wasm;

pub use ecs::*;
#[cfg(feature = "native")]
pub use native::*;
#[cfg(feature = "wasm")]
pub use wasm::*;
