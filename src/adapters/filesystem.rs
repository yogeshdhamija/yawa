use crate::programs::Program;
use crate::services::ports::PersistenceAdapter;
use anyhow::Result;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct FileSystem {}

pub fn new() -> FileSystem {
    FileSystem {}
}

impl PersistenceAdapter for FileSystem {
    fn persist(&self, program: &Program) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        let program_string = format!("{program}");
        write!(file, "{program_string}")?;
        Ok(())
    }
    fn summon(&self) -> Result<Program> {
        let mut file = File::open("/tmp/yawa/saved.json")?;
        let mut program_string = String::new();
        file.read_to_string(&mut program_string)?;
        Ok(Program::parse(&program_string)?)
    }
}
