# Shuttle

## What
[Shuttle](https://shuttle.rs) is a Rust-native cloud development platform that lets you deploy your app while also taking care of all of your infrastructure. It provisions db, static files and secrets. 

## Why
For this I wanted to see if I can use [Shuttle](https://shuttle.rs) to host different ides and projects in one place. The main motivation is to learn how to use it, but also because it looks to be very easy and extendable for my needs since it offers a way to serve more than just static files without having to setup a complicated infrastructure.

## How
It is created as a cargo workspace where Shuttle uses Axum to serve everything. It is made to extend to different projects which will be their own crates.

You can look at the code <https://github.com/ad-ops/ad-ops>.