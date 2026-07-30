# rsomics-help

`rsomics-help` is the Layer-A command-line UX adapter for rsomics products.
It applies one visual and interaction grammar to the real Clap command tree:
top-level and nested help, version output, help navigation, suggestions, and
argument errors.

There is no parallel help specification. Names, flags, defaults, constraints,
value hints, subcommands, descriptions, and usage all come from the same
command tree that performs parsing.

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rsomics-example", version, about = "Example product")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect a sequence file.
    Inspect { input: String },
}

fn main() {
    let _cli = rsomics_help::parse::<Cli>();
}
```

Products may use normal Clap `long_about`, `after_help`, help headings, and
value metadata for domain-specific explanation and examples. `rsomics-help`
does not ask them to repeat a flag table.

## Output policy

- A terminal receives the shared rsomics color palette.
- Redirected output remains plain.
- `NO_COLOR` disables ANSI presentation.
- Help and version use Clap's successful exits.
- Invalid command lines keep Clap's status-2 behavior and contextual
  suggestions.
- Runtime result envelopes, domain errors, and their exit-code mapping belong
  to `rsomics-common`.

The old `HelpSpec` renderer was removed in 0.4 because it duplicated the
parser, drifted on nested commands, and made Layer-B integration harder.

License: MIT OR Apache-2.0.
