use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum GitError {
    GitNotAvailable,
    NotAGitRepository,
    UserNameNotSet,
    UserEmailNotSet,
    RemoteOriginNotSet,
}

impl Error for GitError {}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::GitNotAvailable => {
                write!(f, "Git is not installed or not available in PATH.")
            }
            GitError::NotAGitRepository => write!(f, "Current directory is not a Git repository."),
            GitError::UserNameNotSet => write!(
                f,
                "Git user.name is not set. Please configure it using `git config --global user.name \"Your Name\"`."
            ),
            GitError::UserEmailNotSet => write!(
                f,
                "Git user.email is not set. Please configure it using `git config --global user.email \"Your Email\"`."
            ),
            GitError::RemoteOriginNotSet => write!(
                f,
                "Git remote origin is not set. Please configure it using `git remote add origin <url>`."
            ),
        }
    }
}

fn has_git_installed() -> Result<(), GitError> {
    Command::new("git")
        .arg("--version")
        .output()
        .map_err(|_| GitError::GitNotAvailable)?;
    Ok(())
}

fn is_git_repository(path: &Path) -> Result<(), GitError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .current_dir(path)
        .output()
        .map_err(|_| GitError::NotAGitRepository)?;

    if !output.status.success() {
        return Err(GitError::NotAGitRepository);
    }
    Ok(())
}

fn has_user_name_set(path: &Path) -> Result<(), GitError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .current_dir(path)
        .output()
        .map_err(|_| GitError::UserNameNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitError::UserNameNotSet);
    }
    Ok(())
}

fn has_user_email_set(path: &Path) -> Result<(), GitError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .current_dir(path)
        .output()
        .map_err(|_| GitError::UserEmailNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitError::UserEmailNotSet);
    }
    Ok(())
}

fn has_remote_set(path: &Path) -> Result<(), GitError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .current_dir(path)
        .output()
        .map_err(|_| GitError::RemoteOriginNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitError::RemoteOriginNotSet);
    }
    Ok(())
}

pub fn validate_git_setup(path: &Path) -> Result<(), GitError> {
    has_git_installed()?;
    is_git_repository(path)?;
    has_user_name_set(path)?;
    has_user_email_set(path)?;
    has_remote_set(path)?;
    Ok(())
}
