use crate::git_client::GitClient;
use crate::prompter::Prompter;
use crate::{commands::ExecutableCommand, error::Result};

pub struct ListCommand;

impl ExecutableCommand for ListCommand {
    fn execute(&self, _prompter: &dyn Prompter, git_client: &dyn GitClient) -> Result<()> {
        git_client.validate_repository()?;

        git_client.fetch_all()?;

        let entries = git_client.list_stash_entries()?;

        if entries.is_empty() {
            println!("No remote stashes found.");
            return Ok(());
        };

        // Dynamic column widths based on content
        let pin_width = entries
            .iter()
            .map(|entry| entry.pin.as_str().chars().count())
            .max()
            .unwrap_or(3)
            .max(3);
        let author_width = entries
            .iter()
            .map(|entry| entry.author.chars().count())
            .max()
            .unwrap_or(6)
            .max(6);

        println!(
            "{:<pin_w$}  {:<author_w$}  CREATED AT",
            "PIN",
            "AUTHOR",
            pin_w = pin_width,
            author_w = author_width,
        );
        println!("{}", "-".repeat(pin_width + author_width + 20));

        for entry in &entries {
            println!(
                "{:<pin_w$}  {:<author_w$}  {}",
                entry.pin,
                entry.author,
                entry.created_at,
                pin_w = pin_width,
                author_w = author_width,
            );
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
    use crate::commands::list::ListCommand;
    use crate::error::PairsError;
    use crate::git_client::{MockGitClient, Pin, StashEntry};
    use crate::prompter::MockPrompter;

    #[test]
    fn no_stashes_returns_empty_list() {
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

        // when
        let result = ListCommand.execute(&mock_prompter, &mock_git_client);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn list_stashes_returns_entries() {
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
                    created_at: String::from("2024-01-01 12:00:00"),
                },
                StashEntry {
                    pin: Pin::new(456),
                    author: String::from("Bob"),
                    created_at: String::from("2024-01-02 13:30:00"),
                },
            ])
        });

        let mock_prompter = MockPrompter::new();

        // when
        let result = ListCommand.execute(&mock_prompter, &mock_git_client);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn list_returns_error_on_failed_validation() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Err(PairsError::NotAGitRepository));

        let mock_prompter = MockPrompter::new();

        // when
        let result = ListCommand.execute(&mock_prompter, &mock_git_client);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Not within a git repository"));
    }

    #[test]
    fn list_returns_error_on_failed_fetch() {
        // given
        let mut mock_git_client = MockGitClient::new();
        mock_git_client
            .expect_validate_repository()
            .returning(|| Ok(()));
        mock_git_client
            .expect_fetch_all()
            .returning(|| Err(PairsError::GitCommandFailed(String::from("network error"))));

        let mock_prompter = MockPrompter::new();

        // when
        let result = ListCommand.execute(&mock_prompter, &mock_git_client);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Git command failed: network error"));
    }
}
