pub mod hub;
pub mod mirror;

use crate::byondversion::ByondVersion;
use anyhow::Result;

const REQUEST_TIMEOUT: u64 = 10;

pub fn latest_version(beta: bool) -> Result<ByondVersion> {
	if crate::should_use_mirror() {
		mirror::latest_version(beta)
	} else {
		hub::latest_version(beta)
	}
}
