use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(super) struct Cli {
	#[command(flatten)]
	pub(super) verbose: clap_verbosity_flag::Verbosity,

	#[command(subcommand)]
	pub(super) command: Commands,
}

#[derive(Subcommand)]
pub(super) enum Commands {
	/// Get info about the latest version
	Fetch {
		#[arg(long, short)]
		beta: bool,
	},
	/// Install a new version
	Install { version: String },
	/// List all installed versions
	List {},
	/// Pin a specific version to the current project
	Pin { version: String },
	/// Setup shims and other environmental settings
	Setup {},
	/// Uninstall a previously installed version
	Uninstall { version: String },
}
