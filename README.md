# Just a small project to play with Rust and Web Assembly

This project is aimed to show how to build and deploy a Rust code to wasm and wasi.

There are 3 main steps as follow:

* run a standalone Rust code in the terminal
* run a wasm_bindgen Rust code in the browser
* run a wasi rust code in the terminal

Each step is commit with a tag to ease the switch between code version

## Instructions

[Follow step here](Instructions.md)

## Prerequisites
Tools required to launch examples
- [rustup & rustc & cargo](https://www.rust-lang.org/learn/get-started)
- [wasm-pack & wasm-bindgen](https://rustwasm.github.io/wasm-pack/installer/)
- [wasmer](https://github.com/wasmerio/wasmer#install)

rustup target toolchains:

```text
wasm32-unknown-unknown
wasm32-wasi
```