use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn initialize_repo(
    name: &str,
    override_base_dir: Option<String>,
    default_branch: String,
) -> Result<()> {
    let config = Config::load()?;
    let base_dir = override_base_dir.unwrap_or(config.base_dir);
    let repo_path = Path::new(&base_dir).join(format!("{}.git", name));

    if repo_path.exists() {
        anyhow::bail!("Repository already exists at {}", repo_path.display());
    }

    println!("Creating bare Git repository at {}", repo_path.display());
    fs::create_dir_all(&repo_path)?;

    let output = Command::new("git")
        .arg("init")
        .arg("--bare")
        .arg(&repo_path)
        .output()
        .context("Failed to run `git init`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command `git init` failed: {}", stderr);
    }

    let head_path = repo_path.join("HEAD");
    fs::write(head_path, format!("ref: refs/heads/{}\n", default_branch))?;

    println!("Repo '{}' initialized with default branch '{}'", name, default_branch);
    Ok(())
}
