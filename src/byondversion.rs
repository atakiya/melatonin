use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, str::FromStr};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct ByondVersion {
	pub major: u32,
	pub build: u32,
}

/* ParseByondVersionError stuff */
#[derive(Debug)]
pub struct ParseByondVersionError;

impl Error for ParseByondVersionError {
	fn description(&self) -> &str {
		"failed to parse byondversion"
	}
}

impl fmt::Display for ParseByondVersionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		"provided string could not be parsed into a valid BYOND version".fmt(f)
	}
}

impl fmt::Display for ByondVersion {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.{}", self.major, self.build)
	}
}

impl FromStr for ByondVersion {
	type Err = ParseByondVersionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (version, build) = s.trim().split_once(['.', ' ']).ok_or(ParseByondVersionError)?;

		let version_fromstr = version.parse::<u32>().map_err(|_| ParseByondVersionError)?;
		let build_fromstr = build.parse::<u32>().map_err(|_| ParseByondVersionError)?;

		Ok(ByondVersion {
			major: version_fromstr,
			build: build_fromstr,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_VERSIONSTRING_PERIOD: &str = "515.1630";
	const TEST_VERSIONSTRING_SPACE: &str = "515 1630";
	const TEST_BYONDVERSION_STRUCT: ByondVersion = ByondVersion {
		major: 515,
		build: 1630,
	};

	#[test]
	fn test_string_to_version_period() {
		assert_eq!(
			TEST_VERSIONSTRING_PERIOD.parse::<ByondVersion>().unwrap(),
			TEST_BYONDVERSION_STRUCT
		)
	}

	#[test]
	fn test_string_to_version_space() {
		assert_eq!(
			TEST_VERSIONSTRING_SPACE.parse::<ByondVersion>().unwrap(),
			TEST_BYONDVERSION_STRUCT
		)
	}
}
