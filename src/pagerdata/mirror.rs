use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

use super::REQUEST_TIMEOUT;
use crate::byondversion::ByondVersion;

const MIRROR_VERSION_URL: &str = "https://spacestation13.github.io/byond-builds/version.txt";

pub(super) fn latest_version(beta: bool) -> Result<ByondVersion> {
	log::debug!("Fetching version data from {MIRROR_VERSION_URL}");
	let version_txt = minreq::get(MIRROR_VERSION_URL).with_timeout(REQUEST_TIMEOUT).send()?;
	let mut version_txt = version_txt.as_str()?.trim().lines();
	version_txt
		.nth(if beta { 1 } else { 0 })
		.ok_or(anyhow!("Could not find desired version in version.txt"))
		.and_then(|version| ByondVersion::from_str(version).context("Failed to parse BYOND version"))
}
