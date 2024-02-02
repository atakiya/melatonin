use std::{env, fmt::Write};

use anyhow::Result;
use melatonin::manifest::inventory::InventoryManifest;

use crate::helpers::get_projectdir_version;

pub(crate) fn list() -> Result<()> {
	let manifest = InventoryManifest::new();
	let current_version = get_projectdir_version(&env::current_dir()?)?;

	if current_version.is_some() {
		log::debug!("Current directory has version {} pinned", current_version.unwrap());
	}

	let all_versions_string = manifest.get_all()?.iter().fold(String::new(), |mut output, entry| {
		let in_use_string = if current_version.is_some_and(|version| version == entry.version) {
			" (active)"
		} else {
			""
		};
		let _ = writeln!(output, "\t{}{}", entry.version, in_use_string);
		output
	});

	println!("Currently installed versions:\n{}", all_versions_string);
	Ok(())
}
