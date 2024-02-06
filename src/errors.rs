use std::{env, error::Error, fmt};

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
