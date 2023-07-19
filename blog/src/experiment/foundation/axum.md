# Axum

## What
[Axum](https://docs.rs/axum) is a Rust web application framework. It is one that Shuttle provides as a services (among others).

## Why
One thing that is nice about Shuttle compared to many other services is that it uses many of the common Rust crates instead of inventing new. This means that documentation and examples are more common and that if you want to migrate away it is not very difficult.

For Axum itself it is simple, but powerful and flexible enough for most web-related things.

## How
From start I did not break the code into different modules, but this will probably happen soon that it becomes larger.

Using a seperate `shared` crate means that structs (and libs) can be shared with for example the frontend.

For me this is one of the main benefits in using Rust in different areas since the compiler will help ensure that all the code works.