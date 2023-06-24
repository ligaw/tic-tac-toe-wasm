pub mod game_state;
pub mod game_wrapper;
pub mod player;

use wasm_bindgen::prelude::*;

pub use game_wrapper::GameWrapper;

#[wasm_bindgen]
pub fn health_check() -> String {
    "healthcheck successful".to_string()
}
