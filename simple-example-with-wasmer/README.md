# Rust & Wasm with wasmer

## Build
`cargo build --target wasm32-wasi --release`

## Run
`wasmer target/wasm32-wasi/release/simple-example-with-wasmer.wasm`

## Output
```text
What's ye name?
nitame
Ho hi, nitame!
```

### links
- https://github.com/wasmerio/wasmer