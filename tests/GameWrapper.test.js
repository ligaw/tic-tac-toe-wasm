import { GameWrapper, Player } from '../dist/tic_tac_toe_bg.wasm';

test('make_move should update the board and current player', () => {
  const game = new GameWrapper();
  game.make_move(0, 0);
  expect(game.check_win()).toBe(null);
  expect(game.current_player).toBe(Player.O);
});

