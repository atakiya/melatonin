use std::fs;

use anyhow::Result;
use melatonin::{byondversion::ByondVersion, manifest::inventory::InventoryManifest};

pub(crate) fn uninstall(version_string: String) -> Result<()> {
	log::info!("Requested version to uninstall: {version_string}");

	let inventory = InventoryManifest::new();

	let byond_version: ByondVersion = version_string.parse::<ByondVersion>()?;

	log::debug!("Parsed BYOND version to uninstall: {}", byond_version);

	let install = inventory
		.get(byond_version)?
		.unwrap_or_else(|| panic!("Version {} is not installed!", byond_version));

	log::debug!("Removing binaries at {}", install.path.display());
	fs::remove_dir_all(install.path)?;
	log::debug!("Removed binaries");

	log::debug!("Removing from manifest");
	inventory.remove(byond_version)?;
	log::debug!("Removed from manifest");

	log::info!("Successfully uninstalled version {}", byond_version);
	Ok(())
}
