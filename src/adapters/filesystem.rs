use crate::PersistanceAdapter;
use anyhow::Result;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct FileSystem {}

pub fn new() -> FileSystem {
    FileSystem {}
}

impl PersistanceAdapter for FileSystem {
    fn persist(&self) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        file.write_all(b"Hello, world!")?;
        Ok(())
    }
    fn summon(&self) -> Result<bool> {
        return Ok(!Path::new("/tmp/yawa/saved.json").is_file());
    }
}
