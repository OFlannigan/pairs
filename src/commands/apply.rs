use crate::commands::ExecutableCommand;
use crate::error::PairsError;
use crate::git_client::{GitClient, Pin};
use crate::prompter::Prompter;

pub struct ApplyCommand {
    pin: Pin,
}

impl ApplyCommand {
    pub fn new(pin: Pin) -> Self {
        Self { pin }
    }
}

impl ExecutableCommand for ApplyCommand {
    fn execute(
        &self,
        prompter: &dyn Prompter,
        git_client: &dyn GitClient,
    ) -> crate::error::Result<()> {
        git_client.validate_repository()?;

        let branch_name = self.pin.branch_name();

        git_client.pull_rebase()?;

        let current_branch = git_client.current_branch()?;

        git_client.fetch_all()?;

        if !git_client.remote_branch_exists(&self.pin)? {
            return Err(PairsError::UnknownPin(self.pin.as_u16()));
        }

        git_client.checkout(&branch_name)?;
        git_client.pull_rebase()?;
        git_client.checkout(&current_branch)?;
        git_client.merge_squash_no_commit(&branch_name)?;
        git_client.reset_mixed()?;

        let delete =
            prompter.confirm("Delete temporary pairs branch locally and remotely?", true)?;

        if delete {
            git_client.delete_branch_local(&branch_name)?;
            git_client.delete_branch_remote(&branch_name)?;
        }

        Ok(())
    }
}
