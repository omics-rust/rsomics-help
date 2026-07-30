use std::ffi::OsString;

use clap::builder::styling::{AnsiColor, Styles};
use clap::{ColorChoice, Command, CommandFactory, Error, FromArgMatches};

const FAMILY_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Cyan.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default().bold())
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Yellow.on_default().bold())
    .context(AnsiColor::BrightBlack.on_default())
    .context_value(AnsiColor::Yellow.on_default());

const MAX_HELP_WIDTH: usize = 100;

/// Builds the product command tree with the family UX applied recursively.
#[must_use]
pub fn command<P>() -> Command
where
    P: CommandFactory,
{
    configure(P::command(), color_choice())
}

/// Parses process arguments, exiting with Clap's standard help and error codes.
pub fn parse<P>() -> P
where
    P: CommandFactory + FromArgMatches,
{
    try_parse().unwrap_or_else(|error| error.exit())
}

/// Parses process arguments without exiting.
pub fn try_parse<P>() -> Result<P, Error>
where
    P: CommandFactory + FromArgMatches,
{
    try_parse_from(std::env::args_os())
}

/// Parses an explicit argument iterator without exiting.
pub fn try_parse_from<P, I, T>(arguments: I) -> Result<P, Error>
where
    P: CommandFactory + FromArgMatches,
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let mut command = command::<P>();
    let mut matches = command.try_get_matches_from_mut(arguments)?;
    P::from_arg_matches_mut(&mut matches).map_err(|error| error.format(&mut command))
}

fn color_choice() -> ColorChoice {
    if std::env::var_os("NO_COLOR").is_some() {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    }
}

fn configure(command: Command, color: ColorChoice) -> Command {
    command
        .styles(FAMILY_STYLES)
        .color(color)
        .max_term_width(MAX_HELP_WIDTH)
        .disable_help_subcommand(false)
        .mut_subcommands(|subcommand| configure(subcommand, color))
}

#[cfg(test)]
mod tests {
    use clap::{Args, Parser, Subcommand, error::ErrorKind};

    use super::*;

    #[derive(Debug, Parser)]
    #[command(
        name = "rsomics-example",
        version = "1.2.3",
        about = "Example product",
        subcommand_required = true
    )]
    struct ExampleCli {
        #[command(subcommand)]
        command: ExampleCommand,
    }

    #[derive(Debug, Subcommand)]
    enum ExampleCommand {
        /// Inspect an input.
        Inspect(InspectArgs),
    }

    #[derive(Debug, Args)]
    struct InspectArgs {
        /// Input sequence file.
        #[arg(value_name = "FASTA")]
        input: String,

        /// Emit all records.
        #[arg(short, long)]
        all: bool,
    }

    #[derive(Debug, Parser)]
    #[command(name = "value-example")]
    struct ValueCli {
        value: String,
    }

    #[test]
    fn decorated_command_tree_is_valid() {
        command::<ExampleCli>().debug_assert();
    }

    #[test]
    fn nested_help_is_derived_from_the_real_command_tree() {
        let error = try_parse_from::<ExampleCli, _, _>(["rsomics-example", "inspect", "--help"])
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
        let help = error.to_string();
        assert!(help.contains("Inspect an input"), "{help}");
        assert!(help.contains("<FASTA>"), "{help}");
        assert!(help.contains("--all"), "{help}");
    }

    #[test]
    fn clap_help_subcommand_navigates_the_same_tree() {
        let error =
            try_parse_from::<ExampleCli, _, _>(["rsomics-example", "help", "inspect"]).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
        assert!(error.to_string().contains("--all"));
    }

    #[test]
    fn parse_errors_keep_clap_context_and_suggestions() {
        let error =
            try_parse_from::<ExampleCli, _, _>(["rsomics-example", "inspect", "--al", "input.fa"])
                .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownArgument);
        let message = error.to_string();
        assert!(message.contains("--all"), "{message}");
        assert!(message.contains("Usage:"), "{message}");
    }

    #[test]
    fn ordinary_help_value_is_not_intercepted() {
        let parsed = try_parse_from::<ValueCli, _, _>(["value-example", "help"]).unwrap();
        assert_eq!(parsed.value, "help");
    }

    #[test]
    fn explicit_iterator_builds_the_derived_type() {
        let parsed =
            try_parse_from::<ExampleCli, _, _>(["rsomics-example", "inspect", "--all", "reads.fa"])
                .unwrap();
        let ExampleCommand::Inspect(args) = parsed.command;
        assert_eq!(args.input, "reads.fa");
        assert!(args.all);
    }
}
