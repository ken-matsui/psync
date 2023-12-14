use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};
use serde_derive::Deserialize;

use crate::cargo::Cargo;
use crate::homebrew::Homebrew;
use crate::snap::Snaps;

#[derive(Deserialize)]
pub struct Config {
    pub homebrew: Option<Homebrew>,
    pub cargo: Option<Cargo>,
    pub snap: Option<Snaps>,
}

impl Config {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn load() -> Result<Self> {
        let config_dir = xdg::BaseDirectories::with_prefix("psync")?;
        config_dir.create_config_directory("")?;
        let config_file = config_dir
            .find_config_file("config.toml")
            .context("Failed to find config file")?;

        let config = Self::from_file(config_file)?;
        Ok(config)
    }
}
