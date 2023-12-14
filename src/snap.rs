use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::process::{exit, Command};
use std::str;

use anyhow::Result;
use os_type::{current_platform, OSType};
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum Snap {
    Simple(String),
    Detailed { name: String, classic: bool },
}

impl Snap {
    fn name(&self) -> &str {
        match self {
            Snap::Simple(name) => name,
            Snap::Detailed { name, .. } => name,
        }
    }

    fn is_classic(&self) -> bool {
        match self {
            Snap::Simple(_) => false,
            Snap::Detailed { classic, .. } => *classic,
        }
    }
}

impl Hash for Snap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for Snap {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Snap {}

impl Debug for Snap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize)]
pub(crate) struct Snaps {
    snaps: Option<Vec<Snap>>,
}

fn get_installed_snaps() -> HashSet<String> {
    let output = Command::new("snap")
        .arg("list")
        .arg("--color=never")
        .arg("--unicode=never")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("snap list failed");
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).expect("Not UTF8");
    let lines: Vec<&str> = output_str.lines().collect();

    let mut snaps = HashSet::new();

    for line in lines {
        let snap = line.split_whitespace().next().unwrap();
        snaps.insert(snap.to_string());
    }

    snaps
}

struct State {
    installed: HashSet<String>,
    planned: HashSet<Snap>,
}

impl State {
    fn new() -> Self {
        Self {
            installed: get_installed_snaps(),
            planned: HashSet::new(),
        }
    }

    fn is_installed(&self, snap: &str) -> bool {
        self.installed.contains(snap)
    }

    fn plan_install(&mut self, snap: Snap) {
        if !self.is_installed(snap.name()) {
            self.planned.insert(snap);
        }
    }
}

fn install_snap(snap: &Snap) -> Result<()> {
    let mut snap_cmd = Command::new("sudo");
    snap_cmd.arg("snap").arg("install").arg(snap.name());

    if snap.is_classic() {
        snap_cmd.arg("--classic");
    }

    let output = snap_cmd.output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to install snap"))
    }
}

pub fn install(snaps: Snaps) -> Result<()> {
    if current_platform().os_type != OSType::Ubuntu {
        return Ok(());
    }

    if let Some(snaps) = snaps.snaps {
        let mut state = State::new();

        for snap in snaps {
            state.plan_install(snap);
        }

        println!("=== Snap ===");
        println!("Snaps to install: {:?}", state.planned);

        for snap in state.planned {
            install_snap(&snap)?;
        }
    }

    Ok(())
}
