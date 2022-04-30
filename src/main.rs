//! # YAWA: Yet Another Workout App
//! Keeps track of your lifts and weights.  
//! Program based on the GZCL method.  
//! Relative weights from SymmetricStrength.com.  

use anyhow::Result;

mod adapters;
mod controllers;

fn main() -> Result<()> {
    let file_system_adapter = adapters::filesystem::new();
    controllers::cli::start_program_with_args(&file_system_adapter)?;
    Ok(())
}

pub trait PersistanceAdapter {
    fn persist(&self) -> Result<()>;
    fn summon(&self) -> Result<bool>;
}
