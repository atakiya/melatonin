use std::fs::File;

use anyhow::Result;
use downloader::Downloader;
use melatonin::{
	helpers::{self, userstring_to_byond_version},
	manifest::inventory::{ByondArchive, ByondInstallation, InventoryManifest},
	paths::directories::Directories,
};
use zip::ZipArchive;

pub(crate) fn install(version_string: String) -> Result<()> {
	log::info!("Requested version to install: {version_string}");

	let inventory = InventoryManifest::new();
	let is_first_install = inventory.get_all()?.is_empty();

	let byond_version = userstring_to_byond_version(&version_string)?;

	if inventory.get(byond_version)?.is_some() {
		anyhow::bail!("Version {} is already installed.", byond_version);
	}

	let url = helpers::construct_download_url(&byond_version)?;

	let temp_download_dir = tempfile::tempdir()?;
	let temp_download_dir_path = temp_download_dir.path();

	let mut downloader = Downloader::builder().download_folder(temp_download_dir_path).build()?;

	let dl = downloader::Download::new(url.as_str());

	let results = downloader.download(&[dl])?;

	let downloaded = match results.last().unwrap() {
		Err(e) => anyhow::bail!("Could not download BYOND!\n{}", e.to_string()),
		Ok(s) => {
			log::info!("Downloaded!\n{}", &s);
			ByondArchive {
				path: s.file_name.to_owned(),
				version: byond_version,
			}
		}
	};

	match File::open(downloaded.path) {
		Ok(file) => {
			let destination = Directories::inventory_dir().join(downloaded.version.to_string());
			let mut zip = ZipArchive::new(file)?;
			zip.extract(&destination)?;
			inventory.add(ByondInstallation {
				path: destination,
				version: downloaded.version,
			})?;
		}
		Err(why) => anyhow::bail!("Oops, we did a fucky wucky! OwO\n{}", why),
	}
	if is_first_install {
		melatonin::versionfile::set_global_version(byond_version)?;
		log::info!("Global default version set to {byond_version}");
	}
	Ok(())
}
