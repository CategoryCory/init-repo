use crate::config::Config;
use anyhow::{bail, Context, Result};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub async fn initialize_repo(
    name: &str,
    override_base_dir: Option<String>,
    override_branch: Option<String>,
    override_host: Option<String>,
    override_ssh_key: Option<String>,
) -> Result<()> {
    let config = Config::load()?;

    let host_key = override_host.clone().unwrap_or_else(|| {
        eprintln!("You must specify '--host' or configure a host profile.");
        std::process::exit(1);
    });

    let host_config = config
        .get_host_config(&host_key)
        .with_context(|| format!("No config found for host '{}'", host_key))?;

    let host = override_host.unwrap_or_else(|| host_config.host.clone());
    let base_dir = override_base_dir.unwrap_or_else(|| host_config.base_dir.clone());
    let default_branch = override_branch.unwrap_or_else(|| host_config.default_branch.clone());
    let ssh_key = override_ssh_key.or_else(|| host_config.ssh_key.clone());

    let remote_repo_path = format!("{}/{}.git", base_dir, name);

    println!("Connecting to remote host: {}", host);
    println!("Creating repository: {}", remote_repo_path);

    let remote_command = format!(
        "mkdir -p '{0}' && git init --bare '{0}' && echo 'ref: refs/heads/{1}' > '{0}/HEAD'",
        remote_repo_path, default_branch
    );

    let mut ssh = Command::new("ssh");

    if let Some(key_path) = ssh_key.as_deref() {
        ssh.arg("-i").arg(key_path);
    }

    ssh.arg(&host).arg(&remote_command);

    let result = timeout(Duration::from_secs(10), ssh.output()).await;

    match result {
        Ok(Ok(output)) if output.status.success() => {
            println!("Repo '{}' created on host '{}'", name, host_key);
            Ok(())
        },
        Ok(Ok(output)) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("SSH command failed:\n{}", stderr);
        },
        Ok(Err(e)) => Err(e.into()),
        Err(_) => bail!("Operation timed out after 10 seconds"),
    }
}
