use std::{
	env, fs,
	io::Write,
	path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{byondversion::ByondVersion, paths::files::Files};

pub fn get_currently_used_byondversion() -> Result<Option<ByondVersion>> {
	match get_directory_version(&env::current_dir()?)? {
		None => Ok(get_global_version()?),
		Some(version) => Ok(Some(version)),
	}
}

pub fn get_directory_version(path: &Path) -> Result<Option<ByondVersion>> {
	read_versionfile(path)
}

pub fn get_global_version() -> Result<Option<ByondVersion>> {
	read_versionfile(&Files::global_versionfile())
}

pub fn set_directory_version(directory: &Path, version: ByondVersion) -> Result<()> {
	write_versionfile(directory, version)
}

pub fn set_global_version(version: ByondVersion) -> Result<()> {
	write_versionfile(&Files::global_versionfile(), version)
}

/// Read a pinned version file
///
/// Can either point to a directory or directly to the file if the filename is valid
fn read_versionfile(path: &Path) -> Result<Option<ByondVersion>> {
	log::debug!("Attempting to read byond version file at {}", path.display());
	let versionfile_path: PathBuf = if path
		.file_name()
		.is_some_and(|filename| filename == Files::versionpin_filename())
	{
		path.into()
	} else {
		path.join(Files::versionpin_filename())
	};

	let parsed_version = match versionfile_path.try_exists() {
		Err(why) => anyhow::bail!("{}", why),
		Ok(true) => match fs::read_to_string(versionfile_path) {
			Err(why) => {
				anyhow::bail!(
					"Could not read the {} file at {}\n\tReason: {}",
					Files::versionpin_filename(),
					path.display(),
					why
				)
			}
			Ok(data) => Some(data.parse::<ByondVersion>()?),
		},
		Ok(false) => None,
	};

	Ok(parsed_version)
}

fn write_versionfile(directory: &Path, version: ByondVersion) -> Result<()> {
	let versionfile_path: PathBuf = if directory
		.file_name()
		.is_some_and(|filename| filename == Files::versionpin_filename())
	{
		directory.into()
	} else {
		directory.join(Files::versionpin_filename())
	};
	let mut versionfile = fs::File::create(&versionfile_path).map_err(|why| {
		anyhow::anyhow!(
			"Could not create the {} file at {}\n\tReason: {}",
			Files::versionpin_filename(),
			directory.display(),
			why
		)
	})?;

	match versionfile.write_all(version.to_string().as_bytes()) {
		// Sounds more concise to say "we couldn't write to -this- path than doing the same as above"
		Err(why) => anyhow::bail!("Could not write to {}\n\tReason: {}", &versionfile_path.display(), why),
		Ok(_) => {
			log::debug!("Wrote to {}", &versionfile_path.display());
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	use std::io::Write;

	use super::*;

	const TEST_BYONDVERSION_STRUCT: ByondVersion = ByondVersion {
		major: 515,
		build: 1630,
	};

	#[test]
	fn test_projectdir_version() {
		let path = env::current_dir().unwrap();
		let _ = fs::File::create(path.join(Files::versionpin_filename()))
			.unwrap()
			.write_all(TEST_BYONDVERSION_STRUCT.to_string().as_bytes());
		assert_eq!(get_directory_version(&path).unwrap().unwrap(), TEST_BYONDVERSION_STRUCT);
		let _ = fs::remove_file(path.join(Files::versionpin_filename()));
	}
}
