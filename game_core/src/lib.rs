// game_core/src/lib.rs
mod game;
mod game_data;
mod planet;
mod scheduler;

#[cfg(feature = "native")]
mod native;
#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "native")]
pub use native::*;
pub use scheduler::*;
#[cfg(feature = "wasm")]
pub use wasm::*;
