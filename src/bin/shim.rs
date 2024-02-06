/*
	melatonin / bvm - BYOND version manager
	Copyright (C) 2024 Avunia Takiya <avunia@takiya.eu>

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ffi::OsString;
use std::process::Command;
use std::{env, fmt::Write};

use anyhow::Result;
use melatonin::manifest::inventory::InventoryManifest;
use melatonin::versionfile;
use simple_logger::SimpleLogger;

fn main() -> Result<()> {
	SimpleLogger::new().with_level(log::LevelFilter::Warn).env().init()?;
	print_debuginfo()?;

	let inventory = InventoryManifest::new();

	let current_exe = env::current_exe()?;
	let current_args = env::args_os().skip(1).collect::<Vec<OsString>>();

	let project_version = versionfile::get_currently_used_byondversion()?
		.expect("Could not determine what version to use, no global or local version set!");

	//TODO: Fix this shitcode
	let install = match inventory.get(project_version)?.ok_or("") {
		Err(_) => {
			log::error!("Project has version {} pinned, but is not installed.", project_version);
			//TODO: Ask user if they would like to install the missing version(?) or just abort.
			// Abort for now.
			anyhow::bail!("Missing version for project.")
		}
		Ok(install) => install,
	};

	let called_executable_name = current_exe.file_name().expect("Could not get executable name!");
	let redirected_executable_path = install.path.join("byond/bin").join(called_executable_name);

	match redirected_executable_path.try_exists() {
		Err(why) => anyhow::bail!("Could not check if the executable exists!\n\tReason: {}", why),
		Ok(false) => anyhow::bail!(
			"Could not find the executable - does BYOND version {} have it?",
			project_version
		),
		Ok(true) => {
			let mut exe = Command::new(redirected_executable_path);
			exe.args(current_args).status()?;

			Ok(())
		}
	}
}

fn print_debuginfo() -> Result<()> {
	log::debug!(
		"ARG: {}",
		env::args_os().fold(String::new(), |mut output, arg| {
			let _ = write!(output, "{},", arg.to_string_lossy());
			output
		})
	);
	log::debug!("DIR: {}", env::current_dir()?.to_string_lossy());
	log::debug!("EXE: {}", env::current_exe()?.to_string_lossy());
	log::debug!("CANON: {}", env::current_exe()?.canonicalize()?.display());
	Ok(())
}
