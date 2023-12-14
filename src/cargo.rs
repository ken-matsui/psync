use std::collections::HashSet;
use std::process::{exit, Command};
use std::str;

use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Cargo {
    crates: Option<Vec<String>>,
}

fn get_installed_crates() -> HashSet<String> {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("cargo install --list failed");
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).expect("Not UTF8");
    let lines: Vec<&str> = output_str.lines().collect();

    let mut crates = HashSet::new();

    for line in lines {
        if line.contains(':') {
            let package = line.split(':').next().unwrap().trim();
            let package_name = package.split_whitespace().next().unwrap();
            crates.insert(package_name.to_string());
        }
    }

    crates
}

struct State {
    installed: HashSet<String>,
    planned: HashSet<String>,
}

impl State {
    fn new() -> Self {
        Self {
            installed: get_installed_crates(),
            planned: HashSet::new(),
        }
    }

    fn is_installed(&self, crat: &str) -> bool {
        self.installed.contains(crat)
    }

    fn plan_install(&mut self, crat: String) {
        if !self.is_installed(&crat) {
            self.planned.insert(crat);
        }
    }
}

fn install_crate(crat: &str) -> Result<()> {
    let output = Command::new("cargo").arg("install").arg(crat).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to install crate: {}", crat))
    }
}

pub fn install(cargo: Cargo) -> Result<()> {
    if let Some(crates) = cargo.crates {
        let mut state = State::new();

        for crat in crates {
            state.plan_install(crat);
        }

        println!("=== Cargo ===");
        println!("Crates to install: {:?}", state.planned);

        for crat in state.planned {
            install_crate(&crat)?;
        }
    }

    Ok(())
}
