use clap::{CommandFactory, Parser, ValueEnum};

use crate::{Completion, CompletionShell};

fn get_all_target() -> Vec<String> {
	CompletionShell::value_variants()
		.to_vec()
		.into_iter()
		.map(|t| {
			let t = t
				.to_possible_value()
				.expect("to_possible_value should be Some");
			t.get_name().to_owned()
		})
		.collect()
}

#[test]
fn test_base() -> Result<(), Box<dyn std::error::Error>> {
	#[derive(clap::Parser, Debug, Clone, Default)]
	struct Cli {
		#[command(flatten)]
		completion: Completion,

		#[arg(long, short, default_value_t = false)]
		flag_a: bool,
	}
	for t in get_all_target() {
		let cli = Cli::try_parse_from(["-a", "--generate-completion", t.as_str()]).unwrap();
		let mut f: Vec<u8> = Vec::new();

		if let Some(completion_shell) = cli.completion.generate_completion {
			clap_complete::generate(completion_shell, &mut Cli::command(), "test", &mut f);
		}

		insta::assert_snapshot!(
			t,
			String::from_utf8(f).expect("failed to convert bytes to string")
		);
	}
	Ok(())
}
