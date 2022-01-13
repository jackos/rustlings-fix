/// because `rustlings` is a special type of project where we don't have a
/// cargo.toml linking to each exercise, we need a way to tell `rust-analyzer`
/// how to parse the exercises. This functionality is built into rust-analyzer
/// by putting a `rust-project.json` at the root of the repository. This module generates
/// that file by finding the default toolchain used and looping through each exercise
/// to build the configuration in a way that allows rust-analyzer to work with the exercises.
mod project;
use project::RustAnalyzerProject;

fn main() {
    let mut project = RustAnalyzerProject::new();
    project
        .get_sysroot_src()
        .expect("Couldn't find toolchain path, are you using `rustup`?");
    project
        .exercies_to_json()
        .expect("Couldn't parse rustlings exercises files");

    if project.crates.len() == 0 {
        println!("Failed find any exercises, make sure you're in the `rustlings` folder");
    } else if let Err(_) = project.write_to_disk() {
        println!("Failed to write rust-project.json to disk for rust-analyzer");
    } else {
        println!("Successfully fixed rustlings to work with rust-analyzer, restart your language server or editor")
    }
}
