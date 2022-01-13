# Rustlings Fix
## Quickstart
```bash
# Install rustlings-fix from cargo
cargo install rustlings-fix

# Change directory into wherever rustlings is cloned
cd ~/src/rustlings

# Run the binary
rustlings-fix
```

## Description
because `rustlings` is a special type of project where we don't have a `cargo.toml` linking to each exercise, we need a way to tell `rust-analyzer` how to parse the exercises. `rust-analyzer` can use a `rust-project.json` at the root of the folder you're working from, to work without having a `Cargo.toml` specifying the binary or library. `rustlings-fix` generates that file by looping through each exercise to build the configuration in a way that allows rust-analyzer to work with the exercises.

## More information
[Find more information about how this works here](https://rust-analyzer.github.io/manual.html#non-cargo-based-projects)