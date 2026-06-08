use crate::git_client::GitClient;
use crate::prompter::Prompter;
use crate::{
    commands::ExecutableCommand,
    error::{PairsError, Result},
};

pub struct StashCommand;

impl ExecutableCommand for StashCommand {
    fn execute(&self, prompter: &dyn Prompter, git_client: &dyn GitClient) -> Result<()> {
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

        println!();
        println!("pairs pin: {pin}");

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
