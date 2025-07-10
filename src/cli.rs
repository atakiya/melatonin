use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(super) struct Cli {
	/// Use spacestation13.github.io/byond-builds instead of byond.com.
	#[arg(global = true, long, short)]
	pub(super) mirror: bool,

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
	/// List all installed versions
	List,
	/// Prints the install directory the given version is installed in.
	Prefix { version: String },
	/// Pin a specific version to the current project
	Pin {
		#[arg(long, short)]
		global: bool,
		version: String,
		directory: Option<PathBuf>,
	},
	/// Setup shims and other environmental settings
	Setup,
	/// Install a new version
	Install { version: String },
	/// Uninstall a previously installed version
	Uninstall { version: String },
}
