# Build
A collection of thoughts using Shuttle written in Rust.

There are many different projects in the same workspace and this is done intentionally to have one place for everything.

## Usage
### Setup
This projects uses `Rust` and you need to run `cargo install cargo-make`.

### Run
```sh
# Dev build and run Shuttle locally
cargo make dev
# Release builds and deploy to Shuttle
cargo make deploy
```