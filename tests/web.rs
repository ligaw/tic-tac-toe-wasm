#[cfg(test)]
use tic_tac_toe::game_wrapper::GameWrapper;
use tic_tac_toe::player::Player;
use wasm_bindgen_test::*;
use web_sys::window;
wasm_bindgen_test_configure!(run_in_browser);

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

#[wasm_bindgen_test]
fn test_click_on_board() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Assume that the text you are looking for is in a paragraph element
    let text_element = document.create_element("p").unwrap();
    text_element.set_inner_html("Expected Text");
    body.append_child(&text_element)
        .expect("Failed to append text element");

    let text_present = body.inner_html().contains("Expected Text");
    assert!(text_present, "Expected text was not found on the page");
}
