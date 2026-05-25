use crate::error::{PairsError, Result};
use std::process::{Command, Output};
use std::string::String;

#[derive(Debug, Clone)]
/// Represents a unique identifier for a stash entry, used to create corresponding branches.
/// Leverages the newtype pattern to ensure type safety and encapsulation of the underlying value.
pub struct Pin(u16);

impl Pin {
    /// Creates a new `Pin` instance with the given numeric value.
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns a representation of the pin prefixed with "pairs/" to be used as a branch name.
    pub fn branch_name(&self) -> String {
        format!("pairs/{}", self.0)
    }
}

impl std::fmt::Display for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Checks if the working tree is clean (i.e., no uncommitted changes).
pub fn is_working_tree_clean() -> Result<bool> {
    let output = run_git_command_captured(&["status", "--porcelain"])?;
    Ok(output.is_empty())
}

/// Returns the name of the current branch.
pub fn current_branch() -> Result<String> {
    run_git_command_captured(&["rev-parse", "--abbrev-ref", "HEAD"])
}

/// Fetches all updates from the remote repository, including pruning deleted branches.
pub fn fetch_all() -> Result<()> {
    run_git_command_streaming(&["fetch", "-a"])?;
    run_git_command_streaming(&["fetch", "-p"])
}

/// Generates a random number between 100 and 999 and verifies that no branch with such a pin yet exists.
/// If a branch with the generated pin already exists, it continues generating until it finds a unique one.
pub fn generate_unique_pin() -> Result<Pin> {
    loop {
        let random_pin = Pin::new(100 + rand::random::<u16>() % 900);
        if !remote_branch_exists(&random_pin)? {
            return Ok(random_pin);
        }
    }
}

/// Checks if a remote branch corresponding to the given pin exists in the origin repository.
fn remote_branch_exists(pin: &Pin) -> Result<bool> {
    let output = Command::new("git")
        .args(["ls-remote", "--heads", "origin", &pin.branch_name()])
        .output()?;

    Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
}

/// Creates a new branch with the given name and checks it out.
pub fn checkout_new_branch(branch: &str) -> Result<()> {
    run_git_command_streaming(&["checkout", "-b", branch])
}

/// Adds all changes in the working directory to the staging area.
pub fn add_all() -> Result<()> {
    run_git_command_streaming(&["add", "."])
}

/// Commits staged changes with the provided commit message, bypassing any pre-commit hooks.
pub fn commit_no_verify(message: &str) -> Result<()> {
    run_git_command_streaming(&["commit", "--no-verify", "-m", message])
}

/// Pushes the specified branch to the origin remote and sets it as the upstream branch.
pub fn push_set_upstream(branch: &str) -> Result<()> {
    run_git_command_streaming(&["push", "-u", "origin", branch])
}

/// Checks out the specified branch, switching the working directory to that branch.
pub fn checkout(branch: &str) -> Result<()> {
    run_git_command_streaming(&["checkout", branch])
}

/// Deletes the specified branch from the local repository.
pub fn delete_branch_local(branch: &str) -> Result<()> {
    run_git_command_streaming(&["branch", "-D", branch])
}

/// Resets the current HEAD to the last commit, discarding any uncommitted changes in the working directory.
pub fn reset_hard_head() -> Result<()> {
    run_git_command_streaming(&["reset", "--hard", "HEAD"])
}

/// Removes untracked files and directories from the working directory, ensuring a clean state.
pub fn clean_fd() -> Result<()> {
    run_git_command_streaming(&["clean", "-fd"])
}

/// Performs a squash merge of the specified branch into the current branch without creating a commit.
pub fn merge_squash_no_commit(branch: &str) -> Result<()> {
    run_git_command_streaming(&["merge", "--no-commit", "--squash", branch])
}

/// Helper function to run a git command and capture its output.
/// Returning the `stdout` as a `String` if successful, or an error if the command fails.
fn run_git_command_captured(args: &[&str]) -> Result<String> {
    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git").args(args).output()?;

    if status.success() {
        Ok(String::from_utf8_lossy(&stdout).trim().to_owned())
    } else {
        Err(PairsError::GitCommandFailed(format!(
            "Command 'git {}' failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&stderr).trim()
        )))
    }
}

/// Helper function to run a git command and stream its output directly to the console.
fn run_git_command_streaming(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(PairsError::GitCommandFailed(format!(
            "Command 'git {}' exited with status {}",
            args.join(" "),
            status
        )))
    }
}
