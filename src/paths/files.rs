use std::path::PathBuf;

use super::directories::Directories;

const INV_MANIFEST_FILENAME: &str = "inventory.json";
const VERSIONPIN_FILENAME: &str = ".byondversion";
pub enum Files {}

impl Files {
	pub fn global_versionfile() -> PathBuf {
		Directories::data_local_dir().join(Self::versionpin_filename())
	}

	pub fn inventory_manifest_file() -> PathBuf {
		Directories::data_local_dir().join(Self::inventory_manifest_filename())
	}

	pub fn versionpin_filename() -> &'static str {
		VERSIONPIN_FILENAME
	}

	pub fn inventory_manifest_filename() -> &'static str {
		INV_MANIFEST_FILENAME
	}
}
