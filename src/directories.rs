use anyhow::Result;
use directories::{BaseDirs, ProjectDirs};
use std::{fs, path::PathBuf};

const DIRECTORY_QUALIFIER: &str = "eu.takiya";
const DIRECTORY_ORG: &str = "";
const DIRECTORY_APPNAME: &str = env!("CARGO_PKG_NAME");

pub enum Directories {}

impl Directories {
	pub fn data_local_dir() -> Result<PathBuf> {
		let project_dirs = Self::project_dirs()?;
		let local_data_dir = project_dirs.data_local_dir().to_path_buf();

		if !local_data_dir.try_exists()? {
			fs::create_dir_all(&local_data_dir)?;
		};

		Ok(local_data_dir)
	}

	pub fn bin_local_dir() -> Result<PathBuf> {
		let base_dirs = BaseDirs::new().unwrap();
		let project_dirs = Self::project_dirs()?;
		let local_bin_dir = base_dirs.data_local_dir().join(project_dirs.project_path()).join("bin");

		if !local_bin_dir.try_exists()? {
			fs::create_dir_all(&local_bin_dir)?;
		};

		Ok(local_bin_dir)
	}

	fn project_dirs() -> Result<ProjectDirs> {
		ProjectDirs::from(DIRECTORY_QUALIFIER, DIRECTORY_ORG, DIRECTORY_APPNAME)
			.ok_or(anyhow::anyhow!("Couldn't set up project directories."))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_project_dirs() {
		assert!(Directories::project_dirs().is_ok())
	}

	#[test]
	fn test_data_local_dir() {
		assert!(Directories::data_local_dir().is_ok())
	}

	#[test]
	fn test_bin_local_dir() {
		assert!(Directories::bin_local_dir().is_ok())
	}
}
