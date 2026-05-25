use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[expect(
    dead_code,
    reason = "currently not implemented, but will be used in the future"
)] // TODO: remove this once operations are implemented
pub enum PairsError {
    GitCommandFailed(String),
    UnknownPin(u16),
    InvalidPin(String),
    NoPinsFound,
    NothingToStash,
    Io(std::io::Error),
}

impl Error for PairsError {}

impl Display for PairsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PairsError::GitCommandFailed(msg) => write!(f, "Git command failed: {msg}"),
            PairsError::UnknownPin(pin) => write!(f, "Unknown pin: {pin}"),
            PairsError::InvalidPin(pin) => write!(f, "Invalid pin: {pin}, must be a number"),
            PairsError::NoPinsFound => write!(f, "No pins found."),
            PairsError::NothingToStash => write!(f, "Working tree is clean, nothing to stash."),
            PairsError::Io(err) => write!(f, "IO error: {err}"),
        }
    }
}

impl From<std::io::Error> for PairsError {
    fn from(source: std::io::Error) -> Self {
        PairsError::Io(source)
    }
}

pub type Result<T> = std::result::Result<T, PairsError>;
