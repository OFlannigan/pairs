use crate::commands::ExecutableCommand;
use crate::error::PairsError;
use crate::git;
use crate::git::Pin;
use dialoguer::Confirm;
use std::io::Error;

pub struct ApplyCommand {
    pin: Pin,
}

impl ApplyCommand {
    pub fn new(pin: Pin) -> Self {
        Self { pin }
    }
}

impl ExecutableCommand for ApplyCommand {
    fn execute(&self) -> crate::error::Result<()> {
        git::validate_repository()?;

        let branch_name = self.pin.branch_name();

        git::pull_rebase()?;

        let current_branch = git::current_branch()?;

        git::fetch_all()?;

        if !git::remote_branch_exists(&self.pin)? {
            return Err(PairsError::UnknownPin(self.pin.as_u16()));
        }

        git::checkout(&branch_name)?;
        git::pull_rebase()?;
        git::checkout(&current_branch)?;
        git::merge_squash_no_commit(&branch_name)?;
        git::reset_mixed()?;

        let delete = Confirm::new()
            .with_prompt("Delete temporary pairs branch locally and remotely?")
            .default(true)
            .interact()
            .map_err(|err| PairsError::Io(Error::from(err)))?;

        if delete {
            git::delete_branch_local(&branch_name)?;
            git::delete_branch_remote(&branch_name)?;
        }

        Ok(())
    }
}
