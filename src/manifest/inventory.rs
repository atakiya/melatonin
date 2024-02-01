use crate::{byondversion::ByondVersion, directories::Directories};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

const INV_MANIFEST_FILENAME: &str = "inventory.json";
const INV_MANIFEST_VERSION: u32 = 1;

#[derive(Clone)]
pub struct ByondArchive {
	pub path: PathBuf,
	pub version: ByondVersion,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ByondInstallation {
	pub path: PathBuf,
	pub version: ByondVersion,
}

/// Serializeable data of the install manifest
#[derive(Serialize, Deserialize)]
struct InventoryManifestData {
	pub version: u32,
	pub entries: Vec<ByondInstallation>,
}

pub struct InventoryManifest {
	pub path: PathBuf,
}

impl Default for InventoryManifest {
	fn default() -> Self {
		Self::new()
	}
}

impl InventoryManifest {
	pub fn new() -> Self {
		InventoryManifest {
			path: Directories::data_local_dir().unwrap().join(INV_MANIFEST_FILENAME),
		}
	}

	pub fn add(&self, installation: ByondInstallation) -> Result<()> {
		let mut data = self.read_inventory_manifest()?;

		if self.get(installation.version)?.is_some() {
			log::warn!(
				"Version {} already exists in the manifest, replacing!",
				installation.version
			);
			self.remove(installation.version)?
		}

		log::debug!(
			"Adding version {} at {} to manifest",
			installation.version,
			installation.path.display()
		);
		data.entries.push(installation);

		self.write_inventory_manifest(serde_json::to_string(&data)?)
	}

	pub fn remove(&self, version: ByondVersion) -> Result<()> {
		let mut data = self.read_inventory_manifest()?;

		data.entries.retain(|el| !(el.version == version));

		self.write_inventory_manifest(serde_json::to_string(&data)?)
	}

	pub fn get(&self, version: ByondVersion) -> Result<Option<ByondInstallation>> {
		let data = self.read_inventory_manifest()?;

		let found_install = data.entries.iter().find(|entry| entry.version == version).cloned();
		Ok(found_install)
	}

	pub fn get_all(&self) -> Result<Vec<ByondInstallation>> {
		let data = self.read_inventory_manifest()?;

		Ok(data.entries)
	}

	fn read_inventory_manifest(&self) -> Result<InventoryManifestData> {
		let mut manifest = match self.path.try_exists() {
			Err(why) => anyhow::bail!(why),
			Ok(true) => serde_json::from_str(fs::read_to_string(&self.path)?.as_str())?,
			Ok(false) => InventoryManifestData {
				version: INV_MANIFEST_VERSION,
				entries: Vec::new(),
			},
		};

		manifest.entries.retain(|entry| match entry.path.try_exists() {
			Err(_) => {
				log::warn!("Couldn't access {}, invalidating file.", entry.path.display());
				false
			}
			Ok(does_exist) => does_exist,
		});

		Ok(manifest)
	}

	fn write_inventory_manifest(&self, data: String) -> Result<()> {
		Ok(File::create(&self.path)?.write_all(data.as_bytes())?)
	}
}
