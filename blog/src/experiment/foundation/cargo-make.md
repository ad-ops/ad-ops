# cargo-make
## What
[cargo-make](https://shuttle.rs) is a Rust [Make]() replacement for all application tasks.

## Why
Shuttle uses different working directories for cargo workspaces when running and deploying which meant causes problems with static files. Also since I have multiple static web apps such as `mdBook` and `Dioxus` it means they need to move files.

One other nice part with `cargo-make` is that you can set `install_crate = "mdbook"` and it will install this crate before running the task. So you don't need to write down all setup instructions.

## How
The `Makefile.toml` mainly uses a `dev` and `deploy` task. The first for running everything locally and the later for building and deploying to Shuttle.