#[cfg(test)]
mod wasm {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use tic_tac_toe::GameWrapper;
    use tic_tac_toe::Player;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    pub fn test_start_game() {
        assert_eq!(GameWrapper::new().current_player(), Player::X);
    }

    #[wasm_bindgen_test]
    pub fn test_move() {
        let mut game_wrapper = GameWrapper::new();
        game_wrapper.make_move(0).unwrap();
        assert_eq!(game_wrapper.current_player(), Player::O);
    }
}
