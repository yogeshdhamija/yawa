//! # YAWA: Yet Another Workout App
//! Keeps track of your lifts and weights.  
//! Program based on the GZCL method.  
//! Relative weights from SymmetricStrength.com.  

#![warn(missing_docs)]

mod controllers;

fn main() {
    controllers::cli::parse_arguments_and_handle_help_and_version();
}
