# Rustlings Fix
## Quickstart
```bash
# Install rustlings from cargo
cargo install rustlings-fix
# Change directory into wherever rustlings is cloned
cd ~/src/rustlings
# Run the binary
rustlings-fix
```

## Description
because `rustlings` is a special type of project where we don't have a `cargo.toml` linking to each exercise, we need a way to tell `rust-analyzer` how to parse the exercises. `rust-analyzer` can use a `rust-project.json` at the root of the folder you're working from to treat single files like cargo linking to a main.rs binary. `rustlings-fix` generates that file by finding the default toolchain used and looping through each exercise to build the configuration in a way that allows rust-analyzer to work with the exercises.

## Rust Analyzer Manual Link
[Find more information about how this works here](https://rust-analyzer.github.io/manual.html#non-cargo-based-projects)