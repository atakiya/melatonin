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

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use command::*;
use melatonin::helpers;
use simple_logger::SimpleLogger;

mod cli;
mod command;

fn main() -> Result<()> {
	let cli = Cli::parse();

	SimpleLogger::new()
		.with_level(cli.verbose.log_level_filter())
		.env()
		.init()?;

	match cli.command {
		Commands::Fetch { beta } => fetch::fetch(beta),
		Commands::Install { version } => install::install(version),
		Commands::List {} => list::list(),
		Commands::Pin { version } => pin::pin(version),
		Commands::Setup {} => setup::setup(),
		Commands::Uninstall { version } => uninstall::uninstall(version),
	}
}
