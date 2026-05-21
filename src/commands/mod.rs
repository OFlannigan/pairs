pub mod list;
pub mod pop;
pub mod stash;

use crate::{
    cli::{Cli, PairsCommand},
    error::Result,
};

pub trait ExecutableCommand {
    fn execute(&self) -> Result<()>;
}

pub fn dispatch(cli: Cli) -> Result<()> {
    match (cli.command, cli.pin) {
        (None, None) => stash::StashCommand.execute(),
        (Some(PairsCommand::List), _) => list::ListCommand.execute(),
        (Some(PairsCommand::Pop), _) => pop::PopCommand.execute(),
        (None, Some(_pin)) => Ok(()),
    }
}
