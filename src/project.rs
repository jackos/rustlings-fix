use glob::glob;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize, Deserialize)]
pub struct RustAnalyzerProject {
    sysroot_src: String,
    pub crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize)]
pub struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<String>,
    cfg: Vec<String>,
}

impl RustAnalyzerProject {
    pub fn new() -> RustAnalyzerProject {
        RustAnalyzerProject {
            sysroot_src: String::new(),
            crates: Vec::new(),
        }
    }

    /// Write rust-project.json to disk
    pub fn write_to_disk(&self) -> Result<(), std::io::Error> {
        std::fs::write(
            "./rust-project.json",
            serde_json::to_vec(&self).expect("Failed to serialize to JSON"),
        )?;
        Ok(())
    }

    /// If path contains .rs extension, add a crate to `rust-project.json`
    fn path_to_json(&mut self, path: String) {
        if let Some((_, ext)) = path.split_once(".") {
            if ext == "rs" {
                self.crates.push(Crate {
                    root_module: path,
                    edition: "2021".to_string(),
                    deps: Vec::new(),
                    cfg: vec!["test".to_string()],
                })
            }
        }
    }

    /// Parse the exercises folder for .rs files, any matches will create
    /// a new `crate` in rust-project.json which allows rust-analyzer to
    /// treat it like a normal binary
    pub fn exercies_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        let glob = glob("./exercises/**/*")?;
        for e in glob {
            let path = e?.to_string_lossy().to_string();
            self.path_to_json(path);
        }
        Ok(())
    }

    /// Use `rustup` command to determine the default toolchain, if it exists
    /// it will be put in RustAnalyzerProject.sysroot_src, otherwise an error will be returned
    pub fn get_sysroot_src(&mut self) -> Result<(), Box<dyn Error>> {
        let mut sysroot_src = home::rustup_home()?.to_string_lossy().to_string();

        let output = Command::new("rustup").arg("default").output()?;

        let toolchain = String::from_utf8_lossy(&output.stdout).to_string();

        sysroot_src += "/toolchains/";
        sysroot_src += toolchain
            .split_once(' ')
            .ok_or("Unable to determine toolchain")?
            .0;
        sysroot_src += "/lib/rustlib/src/rust/library";
        println!(
            "Determined toolchain to use with Rustlings: {}\n",
            sysroot_src
        );
        self.sysroot_src = sysroot_src;
        Ok(())
    }
}
