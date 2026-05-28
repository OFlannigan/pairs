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

    /// Returns the string representation of the pin, which is the numeric value as a `String`.
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    /// Returns the numeric value of the pin as a `u16`.
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
/// Represents an entry in the stash, containing the pin, author, and creation time.
pub struct StashEntry {
    pub pin: Pin,
    pub author: String,
    pub created_at: String,
}

/// Checks if the working tree is clean (i.e., no uncommitted changes).
pub fn is_working_tree_clean() -> Result<bool> {
    let output = run_git_command_captured(&["status", "--porcelain"])?;
    Ok(output.is_empty())
}

/// Checks if the repository has any commits yet.
/// This is important for operations that rely on HEAD, which can be ambiguous in a repository with no commits.
pub fn has_commits() -> Result<bool> {
    run_git_check(&["rev-parse", "--verify", "HEAD"])
}

/// Returns the name of the current branch.
pub fn current_branch() -> Result<String> {
    // Guard against repos with no prior commits where HEAD is ambiguous
    if !has_commits()? {
        return Err(PairsError::NoCommitsYet);
    }
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
pub fn remote_branch_exists(pin: &Pin) -> Result<bool> {
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

/// Deletes the specified branch from the remote repository (origin).
pub fn delete_branch_remote(branch: &str) -> Result<()> {
    run_git_command_streaming(&["push", "origin", "--delete", branch])
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

/// Lists all stash entries by querying remote branches that follow the "origin/pairs/*" pattern.
/// Parses the output to extract the pin, author, and creation time for each entry, returning a vector of `StashEntry` instances.
pub fn list_stash_entries() -> Result<Vec<StashEntry>> {
    let format = "%(refname:short)\x1d%(authorname)\x1d%(committerdate:relative)";
    let output = run_git_command_captured(&[
        "for-each-ref",
        "--sort=committerdate",
        "refs/remotes/origin/pairs/*",
        &format!("--format={format}"),
    ])?;

    if output.is_empty() {
        return Ok(vec![]);
    }

    let entries = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, '\x1d').collect();
            if parts.len() == 3 {
                let pin_str = parts.first()?.trim_start_matches("origin/pairs/");
                let pin = pin_str.parse::<u16>().ok()?;
                Some(StashEntry {
                    pin: Pin::new(pin),
                    author: parts.get(1)?.to_string(),
                    created_at: parts.get(2)?.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(entries)
}

/// Performs a git pull with rebase to update the current branch with the latest changes from the remote repository.
pub fn pull_rebase() -> Result<()> {
    run_git_command_streaming(&["pull", "-r"])
}

/// Resets the current branch to the last commit, but keeps changes in the working directory (i.e., unstaged changes).
pub fn reset_mixed() -> Result<()> {
    run_git_command_streaming(&["reset", "--mixed"])
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

/// Helper function to run a git command and check if it executed successfully without caring about its output.
fn run_git_check(args: &[&str]) -> Result<bool> {
    let output = Command::new("git")
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    Ok(output.success())
}
