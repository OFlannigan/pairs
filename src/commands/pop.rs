use crate::{commands::ExecutableCommand, error::Result};

pub struct PopCommand;

impl ExecutableCommand for PopCommand {
    fn execute(&self) -> Result<()> {
        Ok(())
    }
}
