pub mod apply;
pub mod list;
pub mod pop;
pub mod stash;

use crate::error::PairsError;
use crate::git_client::{GitClient, Pin};
use crate::prompter::Prompter;
use crate::{
    cli::{Cli, PairsCommand},
    error::Result,
};
use std::io::Write;

/// A trait that all command structs implement, ensuring they have an `execute` method.
pub trait ExecutableCommand {
    fn execute(
        &self,
        prompter: &dyn Prompter,
        git_client: &dyn GitClient,
        writer: &mut dyn Write,
    ) -> Result<()>;
}

/// Dispatches the appropriate command based on the provided CLI arguments.
pub fn dispatch(
    cli: Cli,
    prompter: &dyn Prompter,
    git_client: &dyn GitClient,
    writer: &mut dyn Write,
) -> Result<()> {
    match (cli.command, cli.pin) {
        (None, None) => stash::StashCommand.execute(prompter, git_client, writer),
        (Some(PairsCommand::List), _) => list::ListCommand.execute(prompter, git_client, writer),
        (Some(PairsCommand::Pop), _) => pop::PopCommand.execute(prompter, git_client, writer),
        (None, Some(raw_pin)) => {
            let pin = raw_pin
                .parse::<u16>()
                .map(Pin::new)
                .map_err(|_| PairsError::InvalidPin(raw_pin))?;

            apply::ApplyCommand::new(pin).execute(prompter, git_client, writer)
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]
mod tests {
    use crate::cli::{Cli, PairsCommand};
    use crate::commands::dispatch;
    use crate::git_client::{MockGitClient, Pin, StashEntry};
    use crate::prompter::MockPrompter;
    use rstest::rstest;

    #[rstest(
        pin_value,
        case::non_numeric("not-a-number"),
        case::negative("-5"),
        case::empty(""),
        case::too_large("99999")
    )]
    fn should_fail_on_invalid_pin_value(pin_value: &str) {
        // given
        let cli = Cli {
            command: None,
            pin: Some(pin_value.to_owned()),
        };
        let prompter = MockPrompter::new();
        let git_client = MockGitClient::new();

        let mut output = Vec::new();

        // when
        let result = dispatch(cli, &prompter, &git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Invalid pin"));
        assert!(error_message.contains(pin_value));
    }

    #[test]
    fn valid_pin_triggers_apply_command() {
        // given
        let cli = Cli {
            command: None,
            pin: Some(String::from("666")),
        };
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

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(false));

        let mut output = Vec::new();

        // when
        let result = dispatch(cli, &mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn list_argument_triggers_list_command_execution() {
        // given
        let cli = Cli {
            command: Some(PairsCommand::List),
            pin: None,
        };
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_list_stash_entries()
            .returning(|| Ok(vec![]));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = dispatch(cli, &mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn pop_fails_when_no_remote_stashes_exist() {
        // given
        let cli = Cli {
            command: Some(PairsCommand::Pop),
            pin: None,
        };
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client
            .expect_list_stash_entries()
            .returning(|| Ok(vec![]));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = dispatch(cli, &mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("No pins found"));
    }

    #[test]
    fn pop_with_one_stash_entry_pops_it() {
        // given
        let cli = Cli {
            command: Some(PairsCommand::Pop),
            pin: None,
        };
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client.expect_list_stash_entries().returning(|| {
            Ok(vec![StashEntry {
                pin: Pin::new(123),
                author: String::from("Alice"),
                created_at: String::from("2024-01-01 12:00:00"),
            }])
        });
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
        mock_git_client
            .expect_remote_branch_exists()
            .returning(|_| Ok(true));
        mock_git_client.expect_checkout().returning(|_| Ok(()));
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_merge_squash_no_commit()
            .returning(|_| Ok(()));
        mock_git_client.expect_reset_mixed().returning(|| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(false));

        let mut output = Vec::new();

        // when
        let result = dispatch(cli, &mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn clean_tree_with_no_arguments_triggers_stash_command() {
        // given
        let cli = Cli {
            command: None,
            pin: None,
        };
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
        let result = dispatch(cli, &mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Working tree is clean"));
    }
}
