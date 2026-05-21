use crate::{commands::ExecutableCommand, error::Result};

pub struct ListCommand;

impl ExecutableCommand for ListCommand {
    fn execute(&self) -> Result<()> {
        Ok(())
    }
}
