use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "init-repo",
    about = "Initialize and manage remote Git repositories",
    version,
    help_template = "\
{name} v{version}
{about}

USAGE:
    init-repo <COMMAND> [OPTIONS]

COMMANDS:
    new         Create a new bare Git repository
    configure   Run the interactive configuration wizard
    help        Show help information

Run 'init-repo help <COMMAND>' for more information on a command.
"
)]
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
}