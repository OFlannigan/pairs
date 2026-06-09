use crate::commands::ExecutableCommand;
use crate::error::{PairsError, Result};
use crate::git_client::{GitClient, Pin};
use crate::prompter::Prompter;
use std::io::Write;

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
        _writer: &mut dyn Write,
    ) -> Result<()> {
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

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]
mod tests {
    use crate::commands::ExecutableCommand;
    use crate::commands::apply::ApplyCommand;
    use crate::git_client::{MockGitClient, Pin};
    use crate::prompter::MockPrompter;

    #[test]
    fn merges_and_deletes_branch_on_existing_pin() {
        // given
        let apply_command = ApplyCommand::new(Pin::new(123));
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_remote_branch_exists()
            .returning(|_| Ok(true));
        mock_git_client.expect_checkout().returning(|_| Ok(()));
        mock_git_client
            .expect_merge_squash_no_commit()
            .returning(|_| Ok(()));
        mock_git_client.expect_reset_mixed().returning(|| Ok(()));
        mock_git_client
            .expect_delete_branch_local()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client
            .expect_delete_branch_remote()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(true));

        let mut output = Vec::new();

        // when
        let result = apply_command.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn merges_and_keeps_branch_on_existing_pin() {
        // given
        let apply_command = ApplyCommand::new(Pin::new(123));
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_remote_branch_exists()
            .returning(|_| Ok(true));
        mock_git_client.expect_checkout().returning(|_| Ok(()));
        mock_git_client
            .expect_merge_squash_no_commit()
            .returning(|_| Ok(()));
        mock_git_client.expect_reset_mixed().returning(|| Ok(()));
        mock_git_client.expect_delete_branch_local().never();
        mock_git_client.expect_delete_branch_remote().never();

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(false));

        let mut output = Vec::new();

        // when
        let result = apply_command.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn returns_error_on_nonexistent_pin() {
        // given
        let apply_command = ApplyCommand::new(Pin::new(999));
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_remote_branch_exists()
            .withf(|pin| pin.as_u16() == 999)
            .returning(|_| Ok(false));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = apply_command.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Unknown pin: 999"));
    }

    #[test]
    fn returns_error_on_failed_validation() {
        // given
        let apply_command = ApplyCommand::new(Pin::new(123));
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Err(crate::error::PairsError::NotAGitRepository));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = apply_command.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Not within a git repository"));
    }
}
