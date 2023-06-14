#[cfg(test)]
mod wasm {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use wasm_bindgen_test::wasm_bindgen_test;
    use tic_tac_toe::GameWrapper;
    use tic_tac_toe::Player;

    #[wasm_bindgen_test]
    pub fn test_start_game() {
        assert_eq!(GameWrapper::new().current_player(), Player::X);
    }

    pub fn test_move() {
        let mut game_wrapper = GameWrapper::new();
        //let board: Vec<Vec<Player>> = game_wrapper.board().into_serde().unwrap();
        game_wrapper.make_move(1,1);
        //assert_eq!(board, Player::X);
        assert_eq!(game_wrapper.current_player(), Player::O);
    }
}
