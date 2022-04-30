//! # YAWA: Yet Another Workout App
//! Keeps track of your lifts and weights.  
//! Program based on the GZCL method.  
//! Relative weights from SymmetricStrength.com.  

#![warn(missing_docs)]

use anyhow::Result;

mod controllers;

fn main() -> Result<()> {
    controllers::cli::start_program_with_args()?;
    Ok(())
}
