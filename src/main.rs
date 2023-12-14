mod cargo;
mod config;
mod homebrew;
mod snap;

use anyhow::Result;

use config::Config;

fn main() -> Result<()> {
    let config = Config::load()?;

    if let Some(brew) = config.homebrew {
        homebrew::install(brew)?;
    }
    if let Some(cargo) = config.cargo {
        cargo::install(cargo)?;
    }
    if let Some(snap) = config.snap {
        snap::install(snap)?;
    }

    Ok(())
}
