# Chapter 1: Hello, World!

This directory mirrors the first exercises from *The Rust Programming Language*.

## Plain `rustc` Example (`hello.rs`)
- Prints two greetings with `println!`
- Compile: `rustc hello.rs`
- Run: `./hello`

## Cargo Project (`hello_cargo/`)
- Generated via `cargo new hello_cargo`
- Contains a minimal app in `src/main.rs` that prints `Hello, world!`
- See `hello_cargo/README.md` for a compact walkthrough of creating, building, running, and checking the project with Cargo commands.

## Tips
- `cargo run` rebuilds only when source changes are detected
- `cargo check` offers fast compile-time validation without producing a binary
- `cargo build --release` produces an optimized binary under `target/release`