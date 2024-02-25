use directories::{BaseDirs, ProjectDirs};
use std::{fs, path::PathBuf};

const DIRECTORY_QUALIFIER: &str = "eu.takiya";
const DIRECTORY_ORG: &str = "";
const DIRECTORY_APPNAME: &str = env!("CARGO_PKG_NAME");
const INVENTORY_PATH_SUFFIX: &str = "inventory";

pub enum Directories {}

impl Directories {
	pub fn inventory_dir() -> PathBuf {
		Self::data_local_dir().join(INVENTORY_PATH_SUFFIX)
	}

	pub fn data_local_dir() -> PathBuf {
		let project_dirs = Self::project_dirs();
		let local_data_dir = project_dirs.data_local_dir().to_path_buf();

		if !local_data_dir
			.try_exists()
			.expect("Could not check if the local data directory exists.")
		{
			fs::create_dir_all(&local_data_dir).unwrap_or_else(|why| {
				panic!(
					"Could not create the directory at {}\nReason: {}",
					local_data_dir.display(),
					why
				)
			});
		};

		local_data_dir
	}

	pub fn bin_local_dir() -> PathBuf {
		let base_dirs = Self::base_dirs();

		let mut local_bin_dir: Option<PathBuf> = None;

		// There's probably a better way to do this...
		if cfg!(target_os = "windows") {
			let project_dirs = Self::project_dirs();
			local_bin_dir = Some(base_dirs.data_local_dir().join(project_dirs.project_path()).join("bin"));
		} else if cfg!(target_os = "linux") {
			local_bin_dir = Some(base_dirs.executable_dir().unwrap().to_path_buf());
		}

		if let Some(dir) = local_bin_dir {
			if !dir
				.try_exists()
				.expect("Could not check if the local data directory exists.")
			{
				fs::create_dir_all(&dir).unwrap_or_else(|why| {
					panic!("Could not create the directory at {}\nReason: {}", dir.display(), why)
				});
			};

			dir
		} else {
			panic!("Unsupported target platform!")
		}
	}

	fn base_dirs() -> BaseDirs {
		BaseDirs::new().expect("Could not set up base directories!")
	}

	fn project_dirs() -> ProjectDirs {
		ProjectDirs::from(DIRECTORY_QUALIFIER, DIRECTORY_ORG, DIRECTORY_APPNAME)
			.expect("Could not set up project directories!")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_base_dirs() {
		let _ = Directories::base_dirs();
	}

	#[test]
	fn test_project_dirs() {
		let _ = Directories::project_dirs();
	}

	#[test]
	fn test_data_local_dir() {
		let _ = Directories::data_local_dir();
	}

	#[test]
	fn test_bin_local_dir() {
		let _ = Directories::bin_local_dir();
	}
}
