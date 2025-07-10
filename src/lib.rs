pub mod byondversion;
pub mod errors;
pub mod helpers;
pub mod manifest;
pub mod pagerdata;
pub mod paths;
pub mod versionfile;

use std::sync::atomic::{AtomicBool, Ordering};

// yeah yeah yeah it's a global flag, idc, bite me, i don't feel like forwarding a single arg through so many layers.
static USE_MIRROR: AtomicBool = AtomicBool::new(false);

pub fn set_mirror(should_use_mirror: bool) {
	USE_MIRROR.store(should_use_mirror, Ordering::Relaxed);
}

pub fn should_use_mirror() -> bool {
	USE_MIRROR.load(Ordering::Relaxed)
}
