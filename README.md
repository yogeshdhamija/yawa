# yawa: Yet Another Workout App

[![ci](https://github.com/yogeshdhamija/yawa/actions/workflows/ci.yaml/badge.svg)](https://github.com/yogeshdhamija/yawa/actions/workflows/ci.yaml)
[![cd](https://github.com/yogeshdhamija/yawa/actions/workflows/cd.yaml/badge.svg)](https://github.com/yogeshdhamija/yawa/actions/workflows/cd.yaml)

https://user-images.githubusercontent.com/4468354/167468920-88c5e704-1090-47e2-8e47-0e463f1ac92d.mp4


## To install:

Simply download the latest release from the releases page.

## Developer Instructions:
- `cargo test` to test.
- `cargo build` to build.
- `cargo run -- start -r 300` to run, as if you had run `yawa start -r 300`.
- `cargo doc --open` to open the documentation.
- Prerequisites:
  - Cargo, the package manager for [the Rust programming language](https://www.rust-lang.org/). Installed by default, alongside Rust, through the `rustup` tool, recommended on their website.
- Code design:
  - `src/lib.rs` is the root, which is built by `cargo` as a library. The various modules of the project are documented there.
  - `src/bin/yawa.rs` is the binary, where the `main` function lives (where execution begins). `cargo` builds this binary and the CD pipeline attaches it to each latest release.
- Pipelines:
  - All commit messages to master should begin with either `chore: `, `fix: `, `feat: `, or `break: `. 
  - The CI pipeline uses this to automatically bump the version (according to semver), and create release tags. 
  - The CD pipeline will run nightly and create a release from the latest tag.
