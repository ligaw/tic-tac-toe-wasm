** Building
- wasm-pack build

** Tests
- cargo test
- wasm-pack test --headless --chrome

** Notes
When the wasm module is built it is put in the pkg directory which is referenced by the package.json in the www directory.
