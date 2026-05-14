use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::Command;

#[derive(Debug)]
pub enum GitOperationError {
    NoChangesToCommit,
    NotOnABranch,
    FailedToFetchChanges,
    FailedToDiscardChanges,
    FailedToCheckoutBranch,
    GitStatusError,
}

impl Error for GitOperationError {}

impl Display for GitOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitOperationError::NoChangesToCommit => write!(f, "No changes to commit."),
            GitOperationError::NotOnABranch => write!(f, "Failed to determine the current branch."),
            GitOperationError::FailedToFetchChanges => {
                write!(f, "Failed to fetch changes from origin.")
            }
            GitOperationError::FailedToDiscardChanges => {
                write!(f, "Failed to discard local changes.")
            }
            GitOperationError::FailedToCheckoutBranch => {
                write!(f, "Failed to checkout the new branch.")
            }
            GitOperationError::GitStatusError => {
                write!(f, "Failed to execute git status command.")
            }
        }
    }
}

pub fn has_git_changes() -> Result<(), GitOperationError> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output();

    match output {
        Ok(output) => {
            if output.stdout.is_empty() {
                Err(GitOperationError::NoChangesToCommit)
            } else {
                Ok(())
            }
        }
        Err(_) => Err(GitOperationError::GitStatusError),
    }
}

pub fn stash_changes() {
    let current_branch = match get_current_branch() {
        Ok(branch) => branch,
        Err(e) => {
            eprintln!("Error getting current branch: {e}");
            return;
        }
    };

    fetch_changes_from_origin().unwrap_or_else(|e| {
        eprintln!("Error fetching changes from origin: {e}");
    });

    let (random_branch_number, branch_name) = create_pairs_branch();

    checkout_branch(&branch_name).unwrap_or_else(|e| {
        eprintln!("Error checking out branch {branch_name}: {e}");
    });

    commit_and_push_changes(&branch_name).unwrap_or_else(|e| {
        eprintln!("Error committing and pushing changes: {e}");
    });

    checkout_branch(&current_branch).unwrap_or_else(|e| {
        eprintln!("Error checking out original branch {current_branch}: {e}");
    });

    println!("pairs pin: {random_branch_number}");

    discard_local_changes().unwrap_or_else(|e| {
        eprintln!("Error discarding local changes: {e}");
    });
}

fn get_current_branch() -> Result<String, GitOperationError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|_| GitOperationError::NotOnABranch)?;

    if !output.status.success() {
        return Err(GitOperationError::NotOnABranch);
    }

    let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    Ok(branch_name)
}

fn fetch_changes_from_origin() -> Result<(), GitOperationError> {
    Command::new("git")
        .arg("fetch")
        .arg("-a")
        .output()
        .map_err(|_| GitOperationError::FailedToFetchChanges)?;

    Command::new("git")
        .arg("fetch")
        .arg("-p")
        .output()
        .map_err(|_| GitOperationError::FailedToFetchChanges)?;
    Ok(())
}

fn create_pairs_branch() -> (u16, String) {
    loop {
        let random_number = 100 + rand::random::<u16>() % 900;
        let branch_name = format!("pairs/{random_number}");

        if !check_if_branch_exists(&branch_name) {
            continue;
        }

        return (random_number, branch_name);
    }
}

fn check_if_branch_exists(branch_name: &str) -> bool {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg("--exit-code")
        .arg("--heads")
        .arg("origin")
        .arg(branch_name)
        .output();

    output.map(|o| o.status.success()).unwrap_or(false)
}

fn checkout_branch(branch_name: &str) -> Result<(), GitOperationError> {
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .map_err(|_| GitOperationError::FailedToCheckoutBranch)?;
    Ok(())
}

fn commit_and_push_changes(branch_name: &str) -> Result<(), GitOperationError> {
    Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .map_err(|_| GitOperationError::NoChangesToCommit)?;

    Command::new("git")
        .arg("commit")
        .arg("--no-verify")
        .arg("-m")
        .arg("temporary pairs branch [skip ci] [ci skip] [ci-skip]")
        .output()
        .map_err(|_| GitOperationError::NoChangesToCommit)?;

    Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg(branch_name)
        .output()
        .map_err(|_| GitOperationError::FailedToCheckoutBranch)?;

    Ok(())
}

fn discard_local_changes() -> Result<(), GitOperationError> {
    Command::new("git")
        .arg("reset")
        .arg("--hard")
        .arg("HEAD")
        .output()
        .map_err(|_| GitOperationError::FailedToDiscardChanges)?;

    Command::new("git")
        .arg("clean")
        .arg("-fd")
        .output()
        .map_err(|_| GitOperationError::FailedToDiscardChanges)?;
    Ok(())
}
