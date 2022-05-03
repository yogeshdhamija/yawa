use crate::programs::Gzcl4Day;
use crate::services::ports::PersistenceAdapter;
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
    program: String,
}

impl PersistenceAdapter for FileSystem {
    fn persist(&self, program: &Gzcl4Day) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        let program_string = format!("{program}");
        let json = serde_json::to_string_pretty(&Saveable {
            program: program_string,
        })?;
        write!(file, "{json}")?;
        Ok(())
    }
    fn summon(&self) -> Result<Gzcl4Day> {
        let file = File::open("/tmp/yawa/saved.json")?;
        let reader = BufReader::new(file);
        let saved: Saveable = serde_json::from_reader(reader)?;
        Ok(Gzcl4Day::parse(&saved.program)?)
    }
}
