mod cli;
mod config;
mod init;

use cli::{Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            repo_name,
            base_dir,
            default_branch,
            host,
            ssh_key,
        } => {
            init::initialize_repo(
                &repo_name,
                base_dir,
                default_branch,
                host,
                ssh_key
            ).await?;
        },
        Commands::Configure => {
            config::run_config_wizard()?;
        },
    }

    Ok(())
}
