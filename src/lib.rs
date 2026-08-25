#[cfg(test)]
mod test;

use clap::ValueEnum;

use carapace_spec_clap::Spec;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete_nushell::Nushell;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CompletionShell {
	#[value(name = "bash")]
	Bash,
	#[value(name = "elvish")]
	Elvish,
	#[value(name = "fish")]
	Fish,
	#[value(name = "nushell")]
	Nushell,
	#[value(name = "powershell")]
	PowerShell,
	#[value(name = "zsh")]
	Zsh,
	#[value(name = "carapace-spec")]
	CarapaceSpec,
}

impl clap_complete::Generator for CompletionShell {
	fn file_name(&self, name: &str) -> String {
		match self {
			Self::Bash => Bash.file_name(name),
			Self::Elvish => Elvish.file_name(name),
			Self::Fish => Fish.file_name(name),
			Self::Nushell => Nushell.file_name(name),
			Self::PowerShell => PowerShell.file_name(name),
			Self::Zsh => Zsh.file_name(name),
			Self::CarapaceSpec => Spec.file_name(name),
		}
	}
	fn try_generate(
		&self,
		cmd: &clap::Command,
		buf: &mut dyn std::io::prelude::Write,
	) -> std::result::Result<(), std::io::Error> {
		let _: () = match self {
			Self::Bash => Bash.try_generate(cmd, buf)?,
			Self::Elvish => clap_complete::shells::Elvish.try_generate(cmd, buf)?,
			Self::Fish => Fish.try_generate(cmd, buf)?,
			Self::Nushell => Nushell.try_generate(cmd, buf)?,
			Self::PowerShell => PowerShell.try_generate(cmd, buf)?,
			Self::Zsh => Zsh.try_generate(cmd, buf)?,
			Self::CarapaceSpec => Spec.try_generate(cmd, buf)?,
		};
		Ok(())
	}

	fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::prelude::Write) {
		self.try_generate(cmd, buf)
			.expect("failed to write completion file")
	}
}

/// Logging flags to `#[command(flatten)]` into your CLI
#[derive(clap::Args, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[command(about = None, long_about = None)]
pub struct Completion {
	#[arg(long, value_enum, value_name = "Shell")]
	/// Generate shell completion script to stdout and exit
	pub generate_completion: Option<CompletionShell>,
}

impl Completion {}
