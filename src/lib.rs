use wasm_bindgen::prelude::*;
use web_sys::console;

mod game_state;
mod game_wrapper;
mod player;

pub use game_state::GameState;
pub use game_wrapper::GameWrapper;
pub use player::Player;

#[wasm_bindgen]
pub fn health_check() -> String {
    console::log_1(&"healthcheck".into());
    "successful".to_string()
}
