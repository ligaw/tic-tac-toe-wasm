use crate::player::Player;

pub struct GameState {
    board: [Player; 9],
    current_player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: [Player::Empty; 9],
            current_player: Player::X,
        }
    }

    pub fn board(&self) -> [Player; 9] {
        self.board
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

    fn check_row(&self, cell: usize) -> Option<Player> {
        if self.board[cell] == self.board[cell + 1] && self.board[cell + 1] == self.board[cell + 2]
        {
            return Some(self.board[cell]);
        }
        None
    }

    fn check_column(&self, cell: usize) -> Option<Player> {
        if self.board[cell] == self.board[cell + 3] && self.board[cell + 3] == self.board[cell + 6]
        {
            return Some(self.board[cell]);
        }
        None
    }
    pub fn check_win(&self) -> Option<Player> {
        let mandatory_cells = [0, 1, 2, 3, 6];
        for cell in mandatory_cells {
            if self.board[cell] == Player::Empty {
                continue;
            }
            match cell {
                0 => {
                    match self.check_column(cell) {
                        Some(_) => return Some(self.board[cell]),
                        None => {}
                    };
                    match self.check_row(cell) {
                        None => {}
                        Some(_) => return Some(self.board[cell]),
                    };
                }
                1 => {
                    match self.check_column(cell) {
                        Some(_) => return Some(self.board[cell]),
                        None => {}
                    };
                }
                2 => {
                    match self.check_column(cell) {
                        Some(_) => return Some(self.board[cell]),
                        None => {}
                    };
                }
                3 => {
                    match self.check_row(cell) {
                        Some(_) => return Some(self.board[cell]),
                        None => {}
                    };
                }
                6 => {
                    match self.check_row(cell) {
                        Some(_) => return Some(self.board[cell]),
                        None => {}
                    };
                }
                _ => {}
            }
        }

        //check diagonals
        if self.board[0] != Player::Empty
            && self.board[0] == self.board[4]
            && self.board[4] == self.board[8]
        {
            return Some(self.board[0]);
        }

        if self.board[2] != Player::Empty
            && self.board[2] == self.board[4]
            && self.board[4] == self.board[6]
        {
            return Some(self.board[2]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;
    use super::Player;

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
    fn test_no_winner() {
        let mut game = GameState::new();
        assert_eq!(game.check_win(), None);

        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(1), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.check_win(), None);
    }

    #[test]
    fn test_check_row_win() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(1), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.make_move(2), Ok(()));
        assert_eq!(game.check_win(), Some(Player::X));
    }

    #[test]
    fn test_check_column_win() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.make_move(2), Ok(()));
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.make_move(6), Ok(()));
        assert_eq!(game.check_win(), Some(Player::X));
    }
    #[test]
    fn test_check_diagonal_win() {
        let mut game = GameState::new();
        assert_eq!(game.make_move(0), Ok(()));
        assert_eq!(game.make_move(2), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(8), Ok(()));
        assert_eq!(game.check_win(), Some(Player::X));
    }
}
