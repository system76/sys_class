use std::path::{Path, PathBuf};

use SysClass;

pub struct Block {
    path: PathBuf
}

impl SysClass for Block {
    fn class() -> &'static str {
        "block"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}
