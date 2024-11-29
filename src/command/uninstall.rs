use std::fs;

use anyhow::Result;
use melatonin::{helpers::userstring_to_byond_version, manifest::inventory::InventoryManifest};

pub(crate) fn uninstall(version_string: String) -> Result<()> {
	log::info!("Requested version to uninstall: {version_string}");

	let inventory = InventoryManifest::new();

	let byond_version = userstring_to_byond_version(&version_string)?;

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
