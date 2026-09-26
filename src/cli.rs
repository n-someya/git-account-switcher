use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "git-account-switcher",
    about = "Declarative Git & GitHub CLI multi-account manager",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new manifest (accounts.toml) by inspecting existing git/gh setup
    Init {
        #[arg(short, long)]
        force: bool,
    },
    /// Generate git & ssh configs and shell integrations from the manifest
    Apply {
        /// Path to custom manifest file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    /// Resolve account for current directory and emit shell exports (for direnv / wrapper)
    Env {
        #[arg(short, long, value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },
    /// Inspect environment, permissions, and detect configuration mismatches
    Doctor,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}
