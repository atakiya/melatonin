use anyhow::{anyhow, Result};
use regex::Regex;

use crate::byondversion::ByondVersion;

const BYOND_PAGER_URL: &str = "https://www.byond.com/PagerHome";
const REQUEST_TIMEOUT: u64 = 10;
const REGEXP_LATEST_VERSION: &str = r#"latest_version:\s+["'](?P<version>\d+)\.(?P<build>\d+)["'],?"#;
const REGEXP_BETA_VERSION: &str = r#"beta_version:\s+["'](?P<version>\d+)\.(?P<build>\d+)["'],?"#;

pub fn latest_version(beta: bool) -> Result<ByondVersion> {
	let channel_expression = match beta {
		// Beta version requested
		true => REGEXP_BETA_VERSION,
		// Latest stable requested
		false => REGEXP_LATEST_VERSION,
	};

	// Pager data
	let data = request()?;

	let captures_version = Regex::new(channel_expression)?
		.captures(&data)
		// Required capture, error out if not found
		.ok_or(anyhow!("Version Regex Capture Error"))?;

	let latest_requested_version = ByondVersion {
		major: captures_version["version"].parse::<u32>()?,
		build: captures_version["build"].parse::<u32>()?,
	};

	Ok(latest_requested_version)
}

fn request() -> Result<String> {
	let pager_data = minreq::get(BYOND_PAGER_URL).with_timeout(REQUEST_TIMEOUT).send()?;

	Ok(String::from(pager_data.as_str()?))
}
