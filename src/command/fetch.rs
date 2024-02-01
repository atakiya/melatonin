use anyhow::Result;
use melatonin::pagerdata;

pub(crate) fn fetch(beta: bool) -> Result<()> {
	let version = pagerdata::latest_version(beta)?;
	log::info!("Latest version: {}", version);
	println!("{}", version);
	Ok(())
}
