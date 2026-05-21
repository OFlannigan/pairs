use crate::{commands::ExecutableCommand, error::Result};

pub struct StashCommand;

impl ExecutableCommand for StashCommand {
    fn execute(&self) -> Result<()> {
        Ok(())
    }
}
