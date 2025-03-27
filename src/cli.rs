use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "init-repo")]
#[command(about = "Initialize and manage remote Git repositories")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new bare repo
    New {
        /// The name of the repo
        repo_name: String,

        /// Specify the base directory; optional
        #[arg(long)]
        base_dir: Option<String>,

        /// Specify the default branch
        #[arg(long, default_value = "master")]
        default_branch: String,
    },

    /// Run configuration setup
    Configure,

    /// Show usage help information
    Help,
}