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

    }
}
