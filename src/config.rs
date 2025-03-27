use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostConfig {
    pub host: String,
    pub base_dir: String,
    pub default_branch: String,
    pub ssh_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hosts: HashMap<String, HostConfig>,
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

    pub fn get_host_config(&self, alias: &str) -> Option<&HostConfig> {
        self.hosts.get(alias)
    }
}

pub fn run_config_wizard() -> Result<()> {
    println!("Configuration setup");

    // Prompt for alias
    print!("Host alias (e.g. 'git-server'): ");
    io::stdout().flush()?;
    let mut alias = String::new();
    io::stdin().read_line(&mut alias)?;
    let alias = alias.trim().to_string();

    // Load existing config or create a blank one
    let mut config = Config::load().unwrap_or_else(|_| Config {
        hosts: HashMap::new(),
    });

    let existing = config.hosts.get(&alias);

    // Prompt for host
    let current_host = existing.map(|h| h.host.clone()).unwrap_or_default();
    print!("Remote SSH address [{}]: ", current_host);
    io::stdout().flush()?;
    let mut host = String::new();
    io::stdin().read_line(&mut host)?;
    let host = if host.trim().is_empty() { current_host } else { host.trim().to_string() };

    // Prompt for base_dir
    let current_base = existing.map(|h| h.base_dir.clone()).unwrap_or("/home/git/repos".to_string());
    print!("Base repository directory [{}]: ", current_base);
    io::stdout().flush()?;
    let mut base_dir = String::new();
    io::stdin().read_line(&mut base_dir)?;
    let base_dir = if base_dir.trim().is_empty() { current_base } else { base_dir.trim().to_string() };

    // Prompt for default branch
    let current_branch = existing.map(|h| h.default_branch.clone()).unwrap_or("master".to_string());
    print!("Default branch name [{}]: ", current_branch);
    io::stdout().flush()?;
    let mut branch = String::new();
    io::stdin().read_line(&mut branch)?;
    let branch = if branch.trim().is_empty() { current_branch } else { branch.trim().to_string() };

    // Prompt for optional SSH key
    let current_key = existing.and_then(|h| h.ssh_key.clone()).unwrap_or_default();
    print!("SSH key path (optional) [{}]: ", current_key);
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let ssh_key = if key.trim().is_empty() {
        if current_key.is_empty() { None } else { Some(current_key) }
    } else {
        Some(key.trim().to_string())
    };

    // Save host config
    config.hosts.insert(
        alias.clone(),
        HostConfig {
            host,
            base_dir,
            default_branch: branch,
            ssh_key,
        },
    );

    config.save()?;
    println!("Host profile '{}' saved.", alias);
    Ok(())
}
