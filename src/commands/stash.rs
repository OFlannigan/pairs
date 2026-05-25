use crate::{
    commands::ExecutableCommand,
    error::{PairsError, Result},
    git,
};
use dialoguer::Confirm;
use std::io::Error;

pub struct StashCommand;

impl ExecutableCommand for StashCommand {
    fn execute(&self) -> Result<()> {
        if git::is_working_tree_clean()? {
            return Err(PairsError::NothingToStash);
        }

        let current_branch = git::current_branch()?;

        git::fetch_all()?;

        let pin = git::generate_unique_pin()?;
        let branch_name = pin.branch_name();

        git::checkout_new_branch(&branch_name)?;
        git::add_all()?;
        git::commit_no_verify("temporary pairs branch [ci-skip] [ci skip] [skip ci]")?;
        git::push_set_upstream(&branch_name)?;
        git::checkout(&current_branch)?;

        println!();
        println!("pairs pin: {pin}");

        let discard = Confirm::new()
            .with_prompt("Discard changes locally?")
            .default(true)
            .interact()
            .map_err(|err| PairsError::Io(Error::from(err)))?;

        if discard {
            // Guard against repos with no prior commits where HEAD is ambiguous
            if git::has_commits()? {
                git::reset_hard_head()?;
            }
            git::clean_fd()?;
        } else {
            git::merge_squash_no_commit(&branch_name)?;
        }

        git::delete_branch_local(&branch_name)?;

        Ok(())
    }
}
