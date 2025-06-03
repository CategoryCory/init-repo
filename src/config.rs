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
            .join(".repo-man/config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Err(anyhow::anyhow!("Config not found. Please run `repo-man configure`."))
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
    println!("Configuration setup -- create or modify a host");

    // Prompt for alias
    print!("Enter config profile name (will create new profile if not found): ");
    io::stdout().flush()?;
    let mut alias = String::new();
    io::stdin().read_line(&mut alias)?;
    let alias = alias.trim().to_string();

    // Load the existing config or create a blank one
    let mut config = Config::load().unwrap_or_else(|_| Config {
        hosts: HashMap::new(),
    });

    let existing = config.hosts.get(&alias);

    let host = prompt_with_default(
        "Host domain name or IP address",
        &existing.map(|h| h.host.clone()).unwrap_or_default(),
    )?;

    let base_dir = prompt_with_default(
        "Repository base directory",
        &existing
            .map(|h| h.base_dir.clone())
            .unwrap_or("/home/git/repos".to_string()),
    )?;

    let branch = prompt_with_default(
        "Default Git branch",
        &existing
            .map(|h| h.default_branch.clone())
            .unwrap_or("master".to_string()),
    )?;

    let ssh_key = prompt_with_default(
        "SSH key path (optional)",
        &existing
            .and_then(|h| h.ssh_key.clone())
            .unwrap_or_default(),   
    )?;
    let ssh_key = if ssh_key.is_empty() { None } else { Some(ssh_key) };

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

pub fn list_hosts() -> Result<()> {
    let config = Config::load().context("Failed to load configuration file")?;

    if config.hosts.is_empty() {
        println!("No host profiles found. Run `repo-man configure` to add one.");
        return Ok(())
    }

    println!("Configured hosts:\n");

    for (alias, host) in &config.hosts {
        println!("{}", alias);
        println!("    Host:           {}", host.host);
        println!("    Base directory: {}", host.base_dir);
        println!("    Default branch: {}", host.default_branch);
        println!(
            "    SSH key path:   {}",
            host.ssh_key.as_deref().unwrap_or("None")
        );
        println!();
    }

    Ok(())
}

pub fn delete_host(profile_name: &str) -> Result<()> {
    let mut config = Config::load().context("Failed to load configuration")?;

    if config.hosts.remove(profile_name).is_some() {
        config.save()?;
        println!("Host profile {} has been deleted.", profile_name);
    } else {
        println!("Host profile {} not found.", profile_name);
    }

    Ok(())
}

fn prompt_with_default(prompt: &str, default: &str) -> Result<String> {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(input.to_string())
    }
}
