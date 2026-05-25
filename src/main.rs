mod cli;
mod commands;
mod error;
mod git;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = commands::dispatch(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
