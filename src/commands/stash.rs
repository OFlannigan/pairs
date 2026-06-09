use crate::git_client::GitClient;
use crate::prompter::Prompter;
use crate::{
    commands::ExecutableCommand,
    error::{PairsError, Result},
};
use std::io::Write;

pub struct StashCommand;

impl ExecutableCommand for StashCommand {
    fn execute(
        &self,
        prompter: &dyn Prompter,
        git_client: &dyn GitClient,
        writer: &mut dyn Write,
    ) -> Result<()> {
        git_client.validate_repository()?;

        if git_client.is_working_tree_clean()? {
            return Err(PairsError::NothingToStash);
        }

        let current_branch = git_client.current_branch()?;

        git_client.fetch_all()?;

        let pin = git_client.generate_unique_pin()?;
        let branch_name = pin.branch_name();

        git_client.checkout_new_branch(&branch_name)?;
        git_client.add_all()?;
        git_client.commit_no_verify("temporary pairs branch [ci-skip] [ci skip] [skip ci]")?;
        git_client.push_set_upstream(&branch_name)?;
        git_client.checkout(&current_branch)?;

        writeln!(writer).ok();
        writeln!(writer, "pairs pin: {pin}").ok();

        let discard = prompter.confirm("Discard changes locally?", true)?;

        if discard {
            // Guard against repos with no prior commits where HEAD is ambiguous
            if git_client.has_commits()? {
                git_client.reset_hard_head()?;
            }
            git_client.clean_fd()?;
        } else {
            git_client.merge_squash_no_commit(&branch_name)?;
        }

        git_client.delete_branch_local(&branch_name)?;

        Ok(())
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]
mod tests {
    use super::*;
    use crate::git_client::{MockGitClient, Pin};
    use crate::prompter::MockPrompter;

    #[test]
    fn cannot_stash_with_no_changes() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client
            .expect_is_working_tree_clean()
            .returning(|| Ok(true));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = StashCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("nothing to stash"));
    }

    #[test]
    fn stash_fails_on_failed_validation() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Err(PairsError::NotAGitRepository));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = StashCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Not within a git repository"));
    }

    #[test]
    fn stashes_on_changes_and_discards_locally() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client
            .expect_is_working_tree_clean()
            .returning(|| Ok(false));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_generate_unique_pin()
            .returning(|| Ok(Pin::new(123)));
        mock_git_client
            .expect_checkout_new_branch()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client.expect_add_all().returning(|| Ok(()));
        mock_git_client
            .expect_commit_no_verify()
            .returning(|_| Ok(()));
        mock_git_client
            .expect_push_set_upstream()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client
            .expect_checkout()
            .withf(|branch_name| branch_name == "main")
            .returning(|_| Ok(()));
        mock_git_client.expect_has_commits().returning(|| Ok(true));
        mock_git_client
            .expect_reset_hard_head()
            .returning(|| Ok(()));
        mock_git_client.expect_clean_fd().returning(|| Ok(()));
        mock_git_client
            .expect_delete_branch_local()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter
            .expect_confirm()
            .withf(|prompt, default| prompt == "Discard changes locally?" && *default)
            .returning(|_, _| Ok(true));

        let mut output = Vec::new();

        // when
        let result = StashCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("pairs pin: 123"));
    }

    #[test]
    fn stashes_on_changes_and_keeps_locally() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client
            .expect_is_working_tree_clean()
            .returning(|| Ok(false));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_generate_unique_pin()
            .returning(|| Ok(Pin::new(123)));
        mock_git_client
            .expect_checkout_new_branch()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client.expect_add_all().returning(|| Ok(()));
        mock_git_client
            .expect_commit_no_verify()
            .returning(|_| Ok(()));
        mock_git_client
            .expect_push_set_upstream()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client
            .expect_checkout()
            .withf(|branch_name| branch_name == "main")
            .returning(|_| Ok(()));
        mock_git_client.expect_has_commits().returning(|| Ok(true));
        mock_git_client
            .expect_merge_squash_no_commit()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));
        mock_git_client
            .expect_delete_branch_local()
            .withf(|branch_name| branch_name == "pairs/123")
            .returning(|_| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(false));

        let mut output = Vec::new();

        // when
        let result = StashCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("pairs pin: 123"));
    }
}
