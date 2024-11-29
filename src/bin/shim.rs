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

	let current_args = env::args_os().skip(1).collect::<Vec<OsString>>();
	let current_exe = env::current_exe()?;

	let project_version = versionfile::get_currently_used_byondversion()?
		.expect("Could not determine what version to use, no global or local version set!");

	//TODO: Fix this shitcode
	let install = inventory.get(project_version)?.ok_or_else(|| {
		log::error!("Project has version {} pinned, but is not installed.", project_version);
		//TODO: Ask user if they would like to install the missing version(?) or just abort.
		// Abort for now.
		anyhow::anyhow!("Missing version for project.")
	})?;

	let called_executable_name = current_exe.file_name().expect("Could not get executable name!");
	let shimmed_bin_path = install.path.join("byond").join("bin");
	let shimmed_exe_path = shimmed_bin_path.join(called_executable_name);

	log::debug!("Shimmed exe path: {}", shimmed_exe_path.display());

	match shimmed_exe_path.try_exists() {
		Err(why) => anyhow::bail!("Could not check if the executable exists!\n\tReason: {}", why),
		Ok(false) => anyhow::bail!(
			"Could not find the executable - does BYOND version {} have it?",
			project_version
		),
		Ok(true) => {
			let mut exe: Command;
			#[allow(unused_mut)]
			let mut args: Vec<OsString> = current_args;

			#[cfg(target_os = "linux")]
			{
				args.insert(0, "--library-path".into());
				args.insert(1, shimmed_bin_path.into());
				args.insert(2, shimmed_exe_path.into());
				exe = Command::new("/lib/ld-linux.so.2");
			}
			#[cfg(target_os = "windows")]
			{
				exe = Command::new(&shimmed_exe_path);
			}

			exe.args(args);
			log::debug!("Running {}", exe.get_program().to_string_lossy());
			log::debug!(
				"With arguments\n{}",
				exe.get_args().fold(String::new(), |mut out, arg| {
					let _ = writeln!(out, "\t{}", arg.to_string_lossy());
					out
				})
			);
			exe.status().expect("Could not execute process");

			Ok(())
		}
	}
}

fn print_debuginfo() -> Result<()> {
	log::debug!(
		"ARGS:\n{}",
		env::args_os().fold(String::new(), |mut output, arg| {
			let _ = writeln!(output, "\t{}", arg.to_string_lossy());
			output
		})
	);
	log::debug!("DIR: {}", env::current_dir()?.to_string_lossy());
	log::debug!("EXE: {}", env::current_exe()?.to_string_lossy());
	log::debug!("CANON: {}", env::current_exe()?.canonicalize()?.display());
	Ok(())
}
