use crate::GameState;
use crate::Player;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameWrapper {
    game: GameState,
}


#[wasm_bindgen]
impl GameWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameWrapper {
        GameWrapper {
            game: GameState::new(),
        }
    }

    pub fn current_player(&self) -> Player {
        self.game.current_player()
    }

    pub fn make_move(&mut self, x:usize, y:usize) -> Result<(), JsValue> {
        self.game.make_move(x, y).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    pub fn check_win(&self) -> Option<Player> {
        self.game.check_win()
    }
}
