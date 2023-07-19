# Dioxus

## What
[Dioxus](https://dioxuslabs.com) is one of many Rust frontend frameworks. It offers great performance and a React-like syntax and aims to target all different areas of frontends (Web, SSR, desktop, App, TUI).

## Why
Fast and simple enough to use. It is also nice that you can build different types of frontends with the same code so it is very easy to also make a desktop app from a web app.

Using Rust as the frontend is also great since I can reuse my structs and functions where I need. This makes it much easier to make sure the code works.

## How
This website uses Dioxus [WebAssembly](https://dioxuslabs.com) target and serves it as a static website from Shuttle (Axum).

Like with the backend (Axum) code I started writing everything in `main.rs`, but I will probably soon break it up into modules and components.