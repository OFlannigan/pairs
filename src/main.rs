mod cli;
mod commands;
mod error;
mod git_client;
mod prompter;

use crate::git_client::PairsGitClient;
use crate::prompter::PairsPrompter;
use clap::Parser;
use cli::Cli;
use std::io;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = commands::dispatch(cli, &PairsPrompter, &PairsGitClient, &mut io::stdout()) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
