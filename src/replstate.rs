use std::path::PathBuf;
use crate::{vfs::VirtualFs};

pub struct ReplState {
    pub cwd: PathBuf,
    pub vfs: VirtualFs,
}