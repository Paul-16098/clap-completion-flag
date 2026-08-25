# clap-completion-flag

just add

```rust
use clap::Parser;
use clap_completion_flag::Completion;
#[derive(Debug, Parser)]
struct Cli {
	#[command(flatten)]
	Completion: Completion,
	// ... other options
}

if let Some(completion_shell) = cli.completion.generate_completion {
	clap_complete::generate(completion_shell, &mut Cli::command(), env!("CARGO_BIN_NAME"), &mut io::stdout());
}

```
