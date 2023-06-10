use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Empty,
    X,
    O,
}

pub struct GameState {
    board: Vec<Vec<Player>>,
    current_player: Player,
}

impl GameState {
    // Initialize a new game
    pub fn new() -> GameState {
        GameState {
            board: vec![vec![Player::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }
    // Make a move
    pub fn make_move(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if x > 2 || y > 2 {
            return Err("Move is out of bounds");
        }
        if self.board[x][y] != Player::Empty {
            return Err("Cell is already occupied");
        }

        self.board[x][y] = self.current_player;
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => unreachable!(),
        };

        Ok(())
    }

    // Check if a player has won
    pub fn check_win(&self) -> Option<Player> {
        // Check rows
        for row in &self.board {
            if row[0] != Player::Empty && row[0] == row[1] && row[0] == row[2] {
                return Some(row[0]);
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] != Player::Empty && self.board[0][col] == self.board[1][col] && self.board[0][col] == self.board[2][col] {
                return Some(self.board[0][col]);
            }
        }

        // Check diagonals
        if self.board[0][0] != Player::Empty && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            return Some(self.board[0][0]);
        }
        if self.board[0][2] != Player::Empty && self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] {
            return Some(self.board[0][2]);
        }

        None
    }
}

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
        self.game.current_player
    }

    pub fn make_move(&mut self, x:usize, y:usize) -> Result<(), JsValue> {
        self.game.make_move(x, y).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    pub fn check_win(&self) -> Option<Player> {
        self.game.check_win()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        for row in game.board.iter() {
            for cell in row.iter() {
                assert_eq!(*cell, Player::Empty);
            }
        }
        assert_eq!(game.current_player, Player::X);
    }

    #[test]
    fn test_make_move() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0, 0), Ok(()));
        assert_eq!(game.board[0][0], Player::X);
        assert_eq!(game.current_player, Player::O);
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let mut game = GameState::new();
        assert!(game.make_move(3, 3).is_err());
    }

    #[test]
    fn test_make_move_cell_occupied() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0, 0), Ok(()));
        assert!(game.make_move(0, 0).is_err());
    }

    #[test]
    fn test_check_win() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0, 0), Ok(()));
        assert_eq!(game.make_move(1, 0), Ok(()));
        assert_eq!(game.make_move(0, 1), Ok(()));
        assert_eq!(game.make_move(1, 1), Ok(()));
        assert_eq!(game.make_move(0, 2), Ok(()));
        assert_eq!(game.check_win(), Some(Player::X));
    }
}
