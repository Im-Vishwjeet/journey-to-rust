# Guessing Game

Interactive number guessing game from Chapter 2 of *The Rust Programming Language*.

## What You Learn
- Bringing standard library modules into scope (`std::io`, `std::cmp::Ordering`)
- Using external crates (`rand::thread_rng().gen_range`) via `Cargo.toml`
- Reading user input, trimming, and parsing to `u32`
- Looping with `loop`, pattern matching with `match`, and early `continue`/`break`

## Run the Game
```sh
cargo run
```
Respond to each prompt with a number between 1 and 100. The program will guide you with **Too small!**, **Too big!**, or **You win!**

## Project Notes
- Invalid input is ignored and the loop continues
- Build artefacts live under `target/`
- Update dependencies with `cargo update`; reproducible versions are tracked in `Cargo.lock`

## Q&A
**What happens if `let mut guess = String::new();` is placed before the loop?**

The same `String` buffer would be reused for every turn. `read_line` appends to existing contents, so each guess would accumulate previous input (for example `"10\n99\n"`). Keeping the declaration inside the loop resets `guess` to an empty string on each iteration and avoids manual clearing.

