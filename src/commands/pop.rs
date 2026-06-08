use crate::git_client::GitClient;
use crate::prompter::Prompter;
use crate::{
    commands::{ExecutableCommand, apply::ApplyCommand},
    error::{PairsError, Result},
};

pub struct PopCommand;

impl ExecutableCommand for PopCommand {
    fn execute(&self, prompter: &dyn Prompter, git_client: &dyn GitClient) -> Result<()> {
        git_client.validate_repository()?;

        println!("Attempting to pop automatically...");

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

        println!("Trying to pop '{pin}'");
        ApplyCommand::new(pin).execute(prompter, git_client)
    }
}
