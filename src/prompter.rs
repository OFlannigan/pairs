use crate::error::{PairsError, Result};
use dialoguer::{Confirm, Select};
use std::io::Error;

/// A trait to abstract user prompting, allowing for easier testing.
#[cfg_attr(test, mockall::automock)]
pub trait Prompter {
    /// Prompts the user for a yes/no confirmation.
    /// Takes a prompt message and a default value, and returns the user's response as a boolean.
    fn confirm(&self, prompt: &str, default: bool) -> Result<bool>;

    /// Prompts the user to select an item from a list.
    /// Takes a prompt message, a list of items to choose from, and a default selection index, and returns the index of the selected item.
    fn select(&self, prompt: &str, items: &[String], default: usize) -> Result<usize>;
}

/// A concrete implementation of the `Prompter` trait using the `dialoguer` crate.
/// This struct can be used in production code, while a mock implementation can be used for testing.
pub struct PairsPrompter;

impl Prompter for PairsPrompter {
    fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        Confirm::new()
            .with_prompt(prompt)
            .default(default)
            .interact()
            .map_err(|err| PairsError::Io(Error::from(err)))
    }

    fn select(&self, prompt: &str, items: &[String], default: usize) -> Result<usize> {
        Select::new()
            .with_prompt(prompt)
            .items(items)
            .default(default)
            .interact()
            .map_err(|_| PairsError::UserAborted)
    }
}
