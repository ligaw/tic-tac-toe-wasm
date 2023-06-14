use serde::{Serialize, Serializer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Empty,
    X,
    O,
}


impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Player::Empty => "Empty",
            Player::X => "X",
            Player::O => "O",
        };
        serializer.serialize_str(value)
    }
}

