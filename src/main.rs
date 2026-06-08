mod cli;
mod commands;
mod error;
mod git;
mod prompter;

use crate::prompter::PairsPrompter;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = commands::dispatch(cli, &PairsPrompter) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
