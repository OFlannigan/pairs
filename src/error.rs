use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[expect(
    dead_code,
    reason = "currently not implemented, but will be used in the future"
)] // TODO: remove this once operations are implemented
pub enum PairsError {
    GitCommandFailed(String),
    UnknownPin(String),
    NoPinsFound,
    NothingToStash,
}

impl Error for PairsError {}

impl Display for PairsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PairsError::GitCommandFailed(msg) => write!(f, "Git command failed: {msg}"),
            PairsError::UnknownPin(pin) => write!(f, "Unknown pin: {pin}"),
            PairsError::NoPinsFound => write!(f, "No pins found."),
            PairsError::NothingToStash => write!(f, "Working tree is clean, nothing to stash."),
        }
    }
}

pub type Result<T> = std::result::Result<T, PairsError>;
