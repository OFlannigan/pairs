pub mod apply;
pub mod list;
pub mod pop;
pub mod stash;

use crate::error::PairsError;
use crate::git::Pin;
use crate::prompter::Prompter;
use crate::{
    cli::{Cli, PairsCommand},
    error::Result,
};

/// A trait that all command structs implement, ensuring they have an `execute` method.
pub trait ExecutableCommand {
    fn execute(&self, prompter: &dyn Prompter) -> Result<()>;
}

/// Dispatches the appropriate command based on the provided CLI arguments.
pub fn dispatch(cli: Cli, prompter: &dyn Prompter) -> Result<()> {
    match (cli.command, cli.pin) {
        (None, None) => stash::StashCommand.execute(prompter),
        (Some(PairsCommand::List), _) => list::ListCommand.execute(prompter),
        (Some(PairsCommand::Pop), _) => pop::PopCommand.execute(prompter),
        (None, Some(raw_pin)) => {
            let pin = raw_pin
                .parse::<u16>()
                .map(Pin::new)
                .map_err(|_| PairsError::InvalidPin(raw_pin))?;

            apply::ApplyCommand::new(pin).execute(prompter)
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]
mod tests {
    use crate::cli::Cli;
    use crate::commands::dispatch;
    use crate::prompter::MockPrompter;
    use rstest::rstest;

    #[rstest(
        pin_value,
        case::non_numeric("not-a-number"),
        case::negative("-5"),
        case::empty(""),
        case::too_large("99999")
    )]
    fn should_fail_on_invalid_pin_value(pin_value: &str) {
        // given
        let cli = Cli {
            command: None,
            pin: Some(pin_value.to_owned()),
        };
        let prompter = MockPrompter::new();

        // when
        let result = dispatch(cli, &prompter);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Invalid pin"));
        assert!(error_message.contains(pin_value));
    }
}
