## Build

Build project with optimized option
`cargo build --release`

## Run

`cargo run`

output:

```text
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/mate-rust-wasm`
nitame
Ho hi, nitame!
```

### without cargo

Build the program with:
`rustc scr/main.rs`

Output:

```text
.
├── Cargo.lock
├── Cargo.toml
├── Instructions.md
├── main <-- executable file here
├── README.md
└── src
  └── main.rs
```

Run the program with:
`./main`

### Note

Cargo is a wrapper to rustc, and is prefered to build and run Rust project. It has a lot of functionalities and is less
verbose than use rustc.