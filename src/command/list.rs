use std::{env, fmt::Write};

use anyhow::Result;
use colored::{ColoredString, Colorize};
use melatonin::{manifest::inventory::InventoryManifest, versionfile};

const GLOBAL_VERSION_SUFFIX: &str = " (global)";
const ACTIVE_VERSION_SUFFIX: &str = " (active)";

pub(crate) fn list() -> Result<()> {
	let manifest = InventoryManifest::new();
	let active_version = versionfile::get_currently_used_byondversion()?;
	let global_version = versionfile::get_global_version()?;
	let project_version = versionfile::get_directory_version(&env::current_dir()?)?;

	if let Some(version) = project_version {
		log::debug!("Version {} is pinned in this directory", version);
	}
	if let Some(version) = global_version {
		log::debug!("Version {} is pinned as global", version);
	}
	if let Some(version) = active_version {
		log::debug!("Version {} is currently active", version);
	}

	let all_versions_string = manifest.get_all()?.iter().fold(String::new(), |mut output, entry| {
		let mut in_use_suffix = String::from("");

		if global_version.is_some_and(|version| version == entry.version) {
			in_use_suffix.push_str(GLOBAL_VERSION_SUFFIX);
		}
		if active_version.is_some_and(|version| version == entry.version) {
			in_use_suffix.push_str(ACTIVE_VERSION_SUFFIX);
		}

		let mut version_line: ColoredString = format!("{}{in_use_suffix}", entry.version).into();

		if active_version.is_some_and(|version| version == entry.version) {
			version_line = version_line.bold().green();
		}

		let _ = writeln!(output, "\t{}", version_line);
		output
	});

	println!("Currently installed versions:\n{}", all_versions_string);
	Ok(())
}
