use crate::git_client::GitClient;
use crate::prompter::Prompter;
use crate::{
    commands::{ExecutableCommand, apply::ApplyCommand},
    error::{PairsError, Result},
};
use std::io::Write;

pub struct PopCommand;

impl ExecutableCommand for PopCommand {
    fn execute(
        &self,
        prompter: &dyn Prompter,
        git_client: &dyn GitClient,
        writer: &mut dyn Write,
    ) -> Result<()> {
        git_client.validate_repository()?;

        writeln!(writer, "Attempting to pop automatically...").ok();

        git_client.fetch_all()?;

        let entries = git_client.list_stash_entries()?;

        let pin = match entries.len() {
            0 => return Err(PairsError::NoPinsFound),
            1 => entries.first().ok_or(PairsError::NoPinsFound)?.pin.clone(),
            _ => {
                let display_items: Vec<String> = entries
                    .iter()
                    .map(|e| format!("{:<8}  {:<20}  {}", e.pin.as_u16(), e.author, e.created_at))
                    .collect();

                let selection = prompter.select("Select a stash to pop", &display_items, 0)?;

                entries
                    .get(selection)
                    .ok_or(PairsError::UserAborted)?
                    .pin
                    .clone()
            }
        };

        writeln!(writer, "Trying to pop '{pin}'").ok();
        ApplyCommand::new(pin).execute(prompter, git_client, writer)
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]
mod test {
    use super::*;
    use crate::git_client::{MockGitClient, Pin, StashEntry};
    use crate::prompter::MockPrompter;

    #[test]
    fn returns_no_pins_found_when_no_stashes_exist() {
        // given
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
        let result = PopCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("No pins found"));
    }

    #[test]
    fn applies_pin_when_only_one_stash_exists() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client.expect_list_stash_entries().returning(|| {
            Ok(vec![StashEntry {
                pin: Pin::new(123),
                author: String::from("Alice"),
                created_at: String::from("2024-06-01 12:00:00"),
            }])
        });
        // expectations for ApplyCommand
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
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
            .returning(|_| Ok(()));
        mock_git_client
            .expect_delete_branch_remote()
            .returning(|_| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_confirm().returning(|_, _| Ok(true));

        let mut output = Vec::new();

        // when
        let result = PopCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Attempting to pop automatically..."));
    }

    #[test]
    fn applies_selected_pin_when_multiple_stashes_exist() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client.expect_fetch_all().returning(|| Ok(()));
        mock_git_client.expect_list_stash_entries().returning(|| {
            Ok(vec![
                StashEntry {
                    pin: Pin::new(123),
                    author: String::from("Alice"),
                    created_at: String::from("2024-06-01 12:00:00"),
                },
                StashEntry {
                    pin: Pin::new(456),
                    author: String::from("Bob"),
                    created_at: String::from("2024-06-02 12:00:00"),
                },
            ])
        });
        // expectations for ApplyCommand
        mock_git_client.expect_pull_rebase().returning(|| Ok(()));
        mock_git_client
            .expect_current_branch()
            .returning(|| Ok(String::from("main")));
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
            .returning(|_| Ok(()));
        mock_git_client
            .expect_delete_branch_remote()
            .returning(|_| Ok(()));

        let mut mock_prompter = MockPrompter::new();
        mock_prompter.expect_select().returning(|_, _, _| Ok(1));
        mock_prompter.expect_confirm().returning(|_, _| Ok(true));

        let mut output = Vec::new();

        // when
        let result = PopCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Attempting to pop automatically..."));
    }

    #[test]
    fn pop_fails_on_failed_validation() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Err(PairsError::NoRemoteOrigin));

        let mock_prompter = MockPrompter::new();

        let mut output = Vec::new();

        // when
        let result = PopCommand.execute(&mock_prompter, &mock_git_client, &mut output);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("No remote named 'origin' found."));
    }
}
