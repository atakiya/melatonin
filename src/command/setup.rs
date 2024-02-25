use std::{env, fs};

use anyhow::Result;
use melatonin::{errors::UnsupportedOSError, paths::directories::Directories};

const BINARY_TYPES_LINUX: &[&str] = &["DreamDaemon", "DreamDownload", "DreamMaker"];
const BINARY_TYPES_WINDOWS: &[&str] = &[
	"byond",
	"dd",
	"dm",
	"dreamdaemon",
	"dreamdeploy",
	"dreammaker",
	"dreamseeker",
];

pub(crate) fn setup() -> Result<()> {
	setup_shims()?;
	Ok(())
}

fn setup_shims() -> Result<()> {
	let local_binary_path = Directories::bin_local_dir();
	let shim_binary_name = format!("{}-shim", env!("CARGO_BIN_NAME"));

	let binaries_to_shim = match env::consts::OS {
		"linux" => BINARY_TYPES_LINUX,
		"windows" => BINARY_TYPES_WINDOWS,
		_ => anyhow::bail!(UnsupportedOSError),
	};

	let mut current_exe = env::current_exe()?.canonicalize()?;
	current_exe.set_file_name(&shim_binary_name);
	current_exe.set_extension(env::consts::EXE_EXTENSION);

	for binary in binaries_to_shim {
		log::debug!(
			"Setting up hardlink for {}\n\tTarget: {}",
			binary,
			&current_exe.display()
		);
		let mut binary_path = local_binary_path.join(binary);
		binary_path.set_extension(env::consts::EXE_EXTENSION);

		if binary_path.try_exists()? {
			log::debug!("Hardlink already exists, removing");
			fs::remove_file(&binary_path)?;
		}

		fs::hard_link(&current_exe, &binary_path).map_err(|op| {
			log::error!("Could not set up hardlink for {}", binary_path.display());
			op
		})?
	}

	// Linux uses $HOME/.local/bin which should be in PATH on systemd systems
	// So we only really need to set up windows
	#[cfg(target_os = "windows")]
	{
		use winreg::{enums::HKEY_CURRENT_USER, RegKey};

		log::info!("Reading Windows Registry for PATH setup...");
		let hkcu = RegKey::predef(HKEY_CURRENT_USER);
		let user_environment = hkcu.open_subkey("Environment")?;
		log::debug!("Getting current PATH from hive");
		let path: String = user_environment.get_value("path")?;

		let local_binary_path_str = local_binary_path
			.to_str()
			.expect("Path to binary directory contains invalid unicode");

		//TODO: Non-Windows code to set envvars
		if !path.contains(local_binary_path_str) {
			log::debug!("PATH does not yet contain binary directory");
			log::debug!("Setting new PATH value");
			let mut command = std::process::Command::new("setx");
			command.arg("Path");
			command.arg(format!("{};{}", local_binary_path_str, path));

			let output = command.output()?;

			if !output.status.success() {
				log::error!("setx stderr: {}", String::from_utf8_lossy(&output.stderr));
				log::error!("setx stdout: {}", String::from_utf8_lossy(&output.stdout));
			}
		} else {
			log::debug!("PATH already contains the binary directory");
		}
	}

	Ok(())
}
