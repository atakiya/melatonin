use std::{env, error::Error, fmt, fs, path::PathBuf};

use anyhow::Result;
use url::Url;

use crate::{byondversion::ByondVersion, pagerdata};

const BYOND_DOWNLOAD_BASEURL: &str = "https://www.byond.com/download/build/";
const BYOND_DOWNLOAD_FILENAME_SUFFIX_WINDOWS: &str = "_byond.zip";
const BYOND_DOWNLOAD_FILENAME_SUFFIX_LINUX: &str = "_byond_linux.zip";

#[derive(Debug)]
pub struct UnsupportedOSError;

impl Error for UnsupportedOSError {
	fn description(&self) -> &str {
		"unsupported OS configuration"
	}
}

impl fmt::Display for UnsupportedOSError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"could not determine a supported OS configuration ({})",
			env::consts::OS
		)
	}
}

pub fn construct_download_url(byond_version: &ByondVersion) -> Result<Url> {
	let url = Url::parse(BYOND_DOWNLOAD_BASEURL)?
		.join(format!("{}/", byond_version.major).as_str())?
		.join(format!("{}{}", byond_version, downloadurl_platform_suffix()?).as_str())?;
	Ok(url)
}

pub fn downloadurl_platform_suffix() -> Result<String, UnsupportedOSError> {
	let platform_suffix = match env::consts::OS {
		"linux" => BYOND_DOWNLOAD_FILENAME_SUFFIX_LINUX,
		"windows" => BYOND_DOWNLOAD_FILENAME_SUFFIX_WINDOWS,
		_ => return Err(UnsupportedOSError),
	};

	Ok(platform_suffix.to_owned())
}

pub fn get_projectdir_version(path: PathBuf) -> Result<Option<ByondVersion>> {
	let version_file = path.join(".byondversion");
	let file = match version_file.try_exists() {
		Err(why) => anyhow::bail!("Couldn't read version file:\n{}", why),
		Ok(true) => Some(fs::read_to_string(version_file)?.parse::<ByondVersion>()?),
		Ok(false) => None,
	};
	Ok(file)
}

pub fn userstring_to_byond_version(version_string: &String) -> Result<ByondVersion> {
	let parsed_version = match version_string.as_str() {
		"latest" | "current" | "stable" => pagerdata::latest_version(false)?,
		"beta" => pagerdata::latest_version(true)?,
		versionstring => versionstring.parse::<ByondVersion>()?,
	};

	log::debug!(
		"Requested version '{}' resolves to BYOND version '{}'",
		version_string,
		parsed_version
	);

	Ok(parsed_version)
}
