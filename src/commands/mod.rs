pub mod apply;
pub mod list;
pub mod pop;
pub mod stash;

use crate::error::PairsError;
use crate::git::Pin;
use crate::{
    cli::{Cli, PairsCommand},
    error::Result,
};

/// A trait that all command structs implement, ensuring they have an `execute` method.
pub trait ExecutableCommand {
    fn execute(&self) -> Result<()>;
}

/// Dispatches the appropriate command based on the provided CLI arguments.
pub fn dispatch(cli: Cli) -> Result<()> {
    match (cli.command, cli.pin) {
        (None, None) => stash::StashCommand.execute(),
        (Some(PairsCommand::List), _) => list::ListCommand.execute(),
        (Some(PairsCommand::Pop), _) => pop::PopCommand.execute(),
        (None, Some(raw_pin)) => {
            let pin = raw_pin
                .parse::<u16>()
                .map(Pin::new)
                .map_err(|_| PairsError::InvalidPin(raw_pin))?;

            apply::ApplyCommand::new(pin).execute()
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

        // when
        let result = dispatch(cli);

        // then
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(error_message.contains("Invalid pin"));
        assert!(error_message.contains(pin_value));
    }
}
