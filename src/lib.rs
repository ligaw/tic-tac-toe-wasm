use wasm_bindgen::prelude::*;

mod game_state;
mod game_wrapper;
mod player;

pub use game_state::GameState;
pub use game_wrapper::GameWrapper;
pub use player::Player;


#[wasm_bindgen]
pub fn healthcheck() -> String {
    "OK".to_string()
}
