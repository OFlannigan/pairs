mod apply;
pub mod list;
pub mod pop;
pub mod stash;

use crate::error::PairsError;
use crate::git::Pin;
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
        (None, Some(raw_pin)) => {
            let pin = raw_pin
                .parse::<u16>()
                .map(Pin::new)
                .map_err(|_| PairsError::InvalidPin(raw_pin))?;

            apply::ApplyCommand::new(pin).execute()
        }
    }
}
