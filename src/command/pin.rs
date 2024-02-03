use std::{env, fs, io::Write};

use anyhow::Result;
use melatonin::helpers::userstring_to_byond_version;

const VERSION_FILE_NAME: &str = ".byondversion";

pub(crate) fn pin(version_string: String) -> Result<()> {
	let byond_version = userstring_to_byond_version(&version_string)?;
	let current_dir = env::current_dir()?;
	let version_file_path = current_dir.join(VERSION_FILE_NAME);

	if !fs::read_dir(&current_dir)?.any(|entry| {
		let path = entry.unwrap().path();
		path.is_file() && path.extension().is_some_and(|ext| ext == "dme")
	}) {
		anyhow::bail!("Directory does not contain a DME file!");
	};

	let mut version_file = fs::File::create(&version_file_path)
		.map_err(|why| anyhow::anyhow!("Couldn't create the version file\n{}", why))?;

	match version_file.write_all(byond_version.to_string().as_bytes()) {
		Err(why) => anyhow::bail!("Couldn't write to {}: {}", &version_file_path.display(), why),
		Ok(_) => log::debug!("Wrote to {}", &version_file_path.display()),
	}

	Ok(log::info!(
		"Pinned version {} for {:?}",
		version_string,
		current_dir.file_name().unwrap_or_default()
	))
}
