use std::env;

use anyhow::Result;
use url::Url;

use crate::{byondversion::ByondVersion, errors, pagerdata};

const BYOND_DOWNLOAD_BASEURL: &str = "https://www.byond.com/download/build/";
const BYOND_DOWNLOAD_FILENAME_SUFFIX_WINDOWS: &str = "_byond.zip";
const BYOND_DOWNLOAD_FILENAME_SUFFIX_LINUX: &str = "_byond_linux.zip";

pub fn construct_download_url(byond_version: &ByondVersion) -> Result<Url> {
	let url = Url::parse(BYOND_DOWNLOAD_BASEURL)?
		.join(format!("{}/", byond_version.major).as_str())?
		.join(format!("{}{}", byond_version, downloadurl_platform_suffix()?).as_str())?;
	Ok(url)
}

fn downloadurl_platform_suffix() -> Result<String, errors::UnsupportedOSError> {
	let platform_suffix = match env::consts::OS {
		"linux" => BYOND_DOWNLOAD_FILENAME_SUFFIX_LINUX,
		"windows" => BYOND_DOWNLOAD_FILENAME_SUFFIX_WINDOWS,
		_ => return Err(errors::UnsupportedOSError),
	};

	Ok(platform_suffix.to_owned())
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

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_BYONDVERSION_STRUCT: ByondVersion = ByondVersion {
		major: 515,
		build: 1630,
	};

	#[test]
	fn test_construct_url() {
		assert!(construct_download_url(&TEST_BYONDVERSION_STRUCT).is_ok())
	}
}
