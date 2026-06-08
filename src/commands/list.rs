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
            .map(|entry| entry.pin.as_str().len())
            .max()
            .unwrap_or(3)
            .max(3);
        let author_width = entries
            .iter()
            .map(|entry| entry.author.len())
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
