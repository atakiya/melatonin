use std::{
	env, fs,
	path::{Path, PathBuf},
};

use anyhow::Result;
use melatonin::{helpers::userstring_to_byond_version, paths::directories::Directories, versionfile};

pub(crate) fn pin(set_as_global: bool, version_string: String, directory: Option<PathBuf>) -> Result<()> {
	let byond_version = userstring_to_byond_version(&version_string)?;
	let mut dme_file: Option<PathBuf> = Option::None;
	let target_directory = if set_as_global {
		Directories::data_local_dir()
	} else if let Some(path) = directory {
		path
	} else {
		env::current_dir()?
	};

	if !set_as_global {
		dme_file = find_dme_file(&target_directory)?;
		if dme_file.is_none() {
			anyhow::bail!("Directory does not contain a DME file!");
		}
	}

	versionfile::set_directory_version(&target_directory, byond_version)?;

	log::info!(
		"Pinned version {} {}",
		byond_version,
		if set_as_global {
			String::from("as global install")
		} else {
			format!("for {}", dme_file.unwrap_or(target_directory).display())
		}
	);

	Ok(())
}

fn find_dme_file(directory: &Path) -> Result<Option<PathBuf>> {
	log::debug!("Attempting to find a .dme file under {}", directory.display());
	// Not entirely happy with the readability here..
	let option = fs::read_dir(directory)?.find_map(|dir_entry| match dir_entry {
		Err(why) => {
			log::warn!(
				"Error while reading directory contents of {}\nReason: {}",
				directory.display(),
				why
			);
			None
		}
		Ok(entry) => {
			let path = entry.path();
			if path.is_file() && path.extension().is_some_and(|ext| ext == "dme") {
				Some(path)
			} else {
				None
			}
		}
	});
	Ok(option)
}
