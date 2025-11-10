# Hello Cargo

A minimal Rust project created with `cargo new` that prints `Hello, world!`. Use it to verify your Rust toolchain and learn the Cargo workflow.

## Quick Start
1. Install latest Rust and Cargo with `rustup` (verify via `cargo --version`).
2. Create and enter the project:
   ```sh
   cargo new hello_cargo
   cd hello_cargo
   ```
3. Build or run:
   ```sh
   cargo build      # compile debug binary to target/debug
   cargo run        # build and run in one step
   cargo check      # fast compile check without binary
   cargo build --release  # optimized build to target/release
   ```

Cargo organizes source files in `src/`, manages dependencies through `Cargo.toml`, and tracks locked versions in `Cargo.lock`. It initializes a Git repo by default (`--vcs` customizes or disables this).

## Project Layout
- `src/main.rs` contains the generated hello-world program
- `Cargo.toml` defines package metadata and dependencies
- `target/` stores build artifacts (not checked into version control)

For existing code, `cargo init` converts a non-Cargo project by creating the manifest and expected layout. Once you’re comfortable, explore more commands (`cargo fmt`, `cargo doc --open`) and start building larger programs like the classic guessing game.
