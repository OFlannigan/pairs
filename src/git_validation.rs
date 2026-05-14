use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum GitValidationError {
    GitNotAvailable,
    NotAGitRepository,
    UserNameNotSet,
    UserEmailNotSet,
    RemoteOriginNotSet,
}

impl Error for GitValidationError {}

impl Display for GitValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitValidationError::GitNotAvailable => {
                write!(f, "Git is not installed or not available in PATH.")
            }
            GitValidationError::NotAGitRepository => {
                write!(f, "Current directory is not a Git repository.")
            }
            GitValidationError::UserNameNotSet => write!(
                f,
                "Git user.name is not set. Please configure it using `git config --global user.name \"Your Name\"`."
            ),
            GitValidationError::UserEmailNotSet => write!(
                f,
                "Git user.email is not set. Please configure it using `git config --global user.email \"Your Email\"`."
            ),
            GitValidationError::RemoteOriginNotSet => write!(
                f,
                "Git remote origin is not set. Please configure it using `git remote add origin <url>`."
            ),
        }
    }
}

fn has_git_installed() -> Result<(), GitValidationError> {
    Command::new("git")
        .arg("--version")
        .output()
        .map_err(|_| GitValidationError::GitNotAvailable)?;
    Ok(())
}

fn is_git_repository(path: &Path) -> Result<(), GitValidationError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .current_dir(path)
        .output()
        .map_err(|_| GitValidationError::NotAGitRepository)?;

    if !output.status.success() {
        return Err(GitValidationError::NotAGitRepository);
    }
    Ok(())
}

fn has_user_name_set(path: &Path) -> Result<(), GitValidationError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .current_dir(path)
        .output()
        .map_err(|_| GitValidationError::UserNameNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitValidationError::UserNameNotSet);
    }
    Ok(())
}

fn has_user_email_set(path: &Path) -> Result<(), GitValidationError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .current_dir(path)
        .output()
        .map_err(|_| GitValidationError::UserEmailNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitValidationError::UserEmailNotSet);
    }
    Ok(())
}

fn has_remote_set(path: &Path) -> Result<(), GitValidationError> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .current_dir(path)
        .output()
        .map_err(|_| GitValidationError::RemoteOriginNotSet)?;

    if output.stdout.is_empty() {
        return Err(GitValidationError::RemoteOriginNotSet);
    }
    Ok(())
}

pub fn validate_git_setup(path: &Path) -> Result<(), GitValidationError> {
    has_git_installed()?;
    is_git_repository(path)?;
    has_user_name_set(path)?;
    has_user_email_set(path)?;
    has_remote_set(path)?;
    Ok(())
}
