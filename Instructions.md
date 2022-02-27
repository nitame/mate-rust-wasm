## Build

`cargo build --target wasm32-wasi --release`

## Run
`wasmer target/wasm32-wasi/release/mate-rust-wasm.wasm`

Output:
```text
What's ye name?
nit
Ho hi, nit!
```