use wasm_bindgen::prelude::*;

mod game_state;
pub mod game_wrapper;
pub mod player;

pub use game_wrapper::GameWrapper;

#[wasm_bindgen]
pub fn health_check() -> String {
    "202307212137".to_string()
}
