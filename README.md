# Tic Tac Toe wasm

## Building
- wasm-pack build --target bundler --out-name wasm --out-dir ./pkg"

## Tests
- cargo test
- wasm-pack test --headless --chrome
- wasm-pack test --headless --firefox

## Notes
When the wasm module is built it is put in the pkg directory which is referenced by the package.json in the www directory.
Important the the target when building is bundler as using Vite.

