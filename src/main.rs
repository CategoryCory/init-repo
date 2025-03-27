mod cli;
mod config;
mod init;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { repo_name, base_dir, default_branch } => {
            init::initialize_repo(&repo_name, base_dir, default_branch)?;
        },
        Commands::Configure => {
            config::run_config_wizard()?;
        },
    }

    Ok(())
}
