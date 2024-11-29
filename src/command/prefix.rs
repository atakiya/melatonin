use anyhow::Result;
use melatonin::{helpers::userstring_to_byond_version, manifest::inventory::InventoryManifest};

pub(crate) fn prefix(version_string: String) -> Result<()> {
	let byond_version = userstring_to_byond_version(&version_string)?;
	let inventory = InventoryManifest::new();
	match inventory.get(byond_version)? {
		Some(version) => {
			print!("{}", version.path.display());
			Ok(())
		}
		None => {
			print!("{byond_version} not installed");
			std::process::exit(1);
		}
	}
}
