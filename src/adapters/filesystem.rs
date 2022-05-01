use crate::services::ports::PersistanceAdapter;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

pub struct FileSystem {}

pub fn new() -> FileSystem {
    FileSystem {}
}

#[derive(Serialize, Deserialize)]
struct Saveable {
    reference_weight: u64,
}

impl PersistanceAdapter for FileSystem {
    fn persist(&self, reference_weight: u64) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        let string = serde_json::to_string_pretty(&Saveable { reference_weight })?;
        write!(file, "{string}")?;
        Ok(())
    }
    fn summon(&self) -> Option<u64> {
        let file = File::open("/tmp/yawa/saved.json").ok()?;
        let reader = BufReader::new(file);
        let saved: Saveable = serde_json::from_reader(reader).ok()?;
        Some(saved.reference_weight)
    }
}
