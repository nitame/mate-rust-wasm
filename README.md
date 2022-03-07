# Just a small project to play with Rust and Web Assembly

This project is aimed to show how to build and deploy a Rust code to wasm and wasi.

There are 3 main example projects.

Each project build and run a simple rust code into different target
- standalone rust code targeted wasm
- rust code targeted wasm with wasm-pack tools
- rust code targeted wasmer runtime with wasi

## Prerequisites
Tools required to launch examples
- [rustup & rustc & cargo](https://www.rust-lang.org/learn/get-started)
- [wasm-pack & wasm-bindgen](https://rustwasm.github.io/wasm-pack/installer/)
- [wasmer](https://github.com/wasmerio/wasmer#install)

rustup targets toolchain required:

```text
wasm32-unknown-unknown
wasm32-wasi
```

To add a target:
`rustup add --target wasm32-unknowm-unknown`