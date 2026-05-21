use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pairs", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<PairsCommand>,

    /// Apply changes from remote stash with pin <PIN> (PIN: returned number after pushing to remote stash)
    #[arg(value_name = "PIN")]
    pub pin: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum PairsCommand {
    /// List all remotely existing stashes.
    List,

    /// Interactively attempt to pop the remote stash. If only one exists, it will be popped. Otherwise, the user will be prompted to select a pin.
    Pop,
}
