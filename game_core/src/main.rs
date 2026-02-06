// game_core/src/main.rs
#[cfg(feature = "native")]
use game_core::run_native;

pub fn main() {
    #[cfg(feature = "native")]
    run_native();
}
