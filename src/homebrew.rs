use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::process::{exit, Command};
use std::str;

use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum Formula {
    Simple(String),
    Detailed { name: String, on: String },
}

impl Formula {
    fn name(&self) -> &str {
        match self {
            Formula::Simple(name) => name,
            Formula::Detailed { name, .. } => name,
        }
    }

    fn should_install(&self) -> bool {
        match self {
            Formula::Simple(_) => true,
            Formula::Detailed { on, .. } => {
                if cfg!(target_os = "macos") && on == "macos" {
                    true
                } else if cfg!(target_os = "linux") && on == "linux" {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Hash for Formula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Formula {}

impl Debug for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize)]
pub(crate) struct Homebrew {
    taps: Option<Vec<Formula>>,
    formulae: Option<Vec<Formula>>,
    casks: Option<Vec<String>>, // casks are only supported on macOS
}

fn get_installed_taps() -> HashSet<String> {
    let output = Command::new("brew")
        .arg("tap")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("brew tap failed");
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).expect("Not UTF8");
    let lines: Vec<&str> = output_str.lines().collect();

    let mut taps = HashSet::new();
    for tap in lines {
        taps.insert(tap.to_string());
    }
    taps
}

fn get_installed_formulae() -> HashSet<String> {
    let output = Command::new("brew")
        .arg("list")
        .arg("--formula")
        .arg("-1")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("brew list --formula -1 failed");
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).expect("Not UTF8");
    let lines: Vec<&str> = output_str.lines().collect();

    let mut formulae = HashSet::new();
    for formula in lines {
        formulae.insert(formula.to_string());
    }
    formulae
}

#[cfg(target_os = "macos")]
fn get_installed_casks() -> HashSet<String> {
    let output = Command::new("brew")
        .arg("list")
        .arg("--cask")
        .arg("-1")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("brew list --cask -1 failed");
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).expect("Not UTF8");
    let lines: Vec<&str> = output_str.lines().collect();

    let mut casks = HashSet::new();
    for cask in lines {
        casks.insert(cask.to_string());
    }
    casks
}

struct State {
    installed_taps: HashSet<String>,
    installed_formulae: HashSet<String>,
    installed_casks: HashSet<String>,
    planned_taps: HashSet<Formula>,
    planned_formulae: HashSet<Formula>,
    planned_casks: HashSet<String>,
}

impl State {
    fn new() -> Self {
        Self {
            installed_taps: get_installed_taps(),
            installed_formulae: get_installed_formulae(),
            installed_casks: get_installed_casks(),
            planned_taps: HashSet::new(),
            planned_formulae: HashSet::new(),
            planned_casks: HashSet::new(),
        }
    }

    fn is_installed_tap(&self, tap: &str) -> bool {
        self.installed_taps.contains(tap)
    }

    fn is_installed_formula(&self, formula: &str) -> bool {
        self.installed_formulae.contains(formula)
    }

    #[cfg(target_os = "macos")]
    fn is_installed_cask(&self, cask: &str) -> bool {
        self.installed_casks.contains(cask)
    }

    fn plan_install_tap(&mut self, tap: Formula) {
        if !self.is_installed_tap(tap.name()) {
            self.planned_taps.insert(tap);
        }
    }

    fn plan_install_formula(&mut self, formula: Formula) {
        if !self.is_installed_formula(formula.name()) {
            self.planned_formulae.insert(formula);
        }
    }

    #[cfg(target_os = "macos")]
    fn plan_install_cask(&mut self, cask: String) {
        if !self.is_installed_cask(&cask) {
            self.planned_casks.insert(cask);
        }
    }
}

pub fn install(homebrew: Homebrew) -> Result<()> {
    if let Some(taps) = homebrew.taps {
        let mut state = State::new();

        for tap in taps {
            if tap.should_install() {
                state.plan_install_tap(tap);
            }
        }

        println!("=== Homebrew Taps ===");
        println!("Taps to install: {:?}", state.planned_taps);

        for tap in state.planned_taps {
            let output = Command::new("brew")
                .arg("tap")
                .arg(tap.name())
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                eprintln!("brew tap {} failed", tap.name());
                exit(1);
            }
        }
    }

    if let Some(formulae) = homebrew.formulae {
        let mut state = State::new();

        for formula in formulae {
            if formula.should_install() {
                state.plan_install_formula(formula);
            }
        }

        println!("=== Homebrew Formulae ===");
        println!("Formulae to install: {:?}", state.planned_formulae);

        for formula in state.planned_formulae {
            let output = Command::new("brew")
                .arg("install")
                .arg(formula.name())
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                eprintln!("brew install {} failed", formula.name());
                exit(1);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(casks) = homebrew.casks {
            let mut state = State::new();

            for cask in casks {
                state.plan_install_cask(cask);
            }

            println!("=== Homebrew Casks ===");
            println!("Casks to install: {:?}", state.planned_casks);

            for cask in state.planned_casks {
                let output = Command::new("brew")
                    .arg("install")
                    .arg("--cask")
                    .arg(cask.clone())
                    .output()
                    .expect("Failed to execute command");

                if !output.status.success() {
                    eprintln!("brew install --cask {} failed", cask);
                    exit(1);
                }
            }
        }
    }

    Ok(())
}
