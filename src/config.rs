use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub base_dir: String,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        Ok(home_dir()
            .context("Could not find home directory")?
            .join(".init-repo/config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Err(anyhow::anyhow!("Config not found. Please run `init-repo configure`."))
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml = toml::to_string_pretty(self)?;
        fs::write(path, toml)?;
        Ok(())
    }
}

pub fn run_config_wizard() -> Result<()> {
    println!("Initial Configuration Setup Wizard");

    let existing = Config::load().ok();
    let current_base = existing
        .as_ref()
        .map(|c| c.base_dir.clone())
        .unwrap_or_else(|| "/home/git/repos".to_string());

    print!("Base directory for repositories [{}]: ", current_base);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    let base_dir = if input.is_empty() {
        current_base
    } else {
        input.to_string()
    };

    let config = Config { base_dir };
    config.save()?;
    println!("Configuration saved.");
    Ok(())
}
