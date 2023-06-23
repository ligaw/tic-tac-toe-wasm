use crate::Player;

pub struct GameState {
    board: [Player; 9],
    current_player: Player,
}

impl GameState {
    // Initialize a new game
    pub fn new() -> GameState {
        GameState {
            board: [Player::Empty; 9],
            current_player: Player::X,
        }
    }

    pub fn board(&self) -> [Player; 9] {
        self.board.clone()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn make_move(&mut self, cell: usize) -> Result<(), &'static str> {
        if cell > 8 {
            return Err("Move is out of bounds");
        }
        if self.board[cell] != Player::Empty {
            return Err("Cell is already occupied");
        }

        self.board[cell] = self.current_player;
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => unreachable!(),
        };

        Ok(())
    }

    pub fn check_win(&self) -> Option<Player> {
        // Check rows
        if self.board[0] != Player::Empty
            && self.board[0] == self.board[1]
            && self.board[0] == self.board[2]
        {
            return Some(self.board[0]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;
    use crate::Player;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        for cell in game.board().iter() {
            assert_eq!(*cell, Player::Empty);
        }
        assert_eq!(game.current_player(), Player::X);
    }

    #[test]
    fn test_make_move() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.board()[0], Player::X);
        assert_eq!(game.current_player(), Player::O);
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let mut game = GameState::new();
        assert!(game.make_move(9).is_err());
    }

    #[test]
    fn test_make_move_cell_occupied() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert!(game.make_move(0).is_err());
    }

    #[test]
    fn test_check_win() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(1), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.make_move(2), Ok(()));
        assert_eq!(game.check_win(), Some(Player::X));
    }
}
