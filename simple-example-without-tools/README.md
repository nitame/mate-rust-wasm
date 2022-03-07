# Rust and Wasm

## Build
`cargo build --target wasm32-unknown-unknown --release`

## Run
`python3 -m http.server`

## Output
Go to http://localhost:8000 and open browser console

### Links
- https://stackoverflow.com/questions/47529643/how-to-return-a-string-or-similar-from-rust-in-webassembly
- https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly
- https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/