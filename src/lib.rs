//! # YAWA: Yet Another Workout App
//! Keeps track of your lifts and weights.  
//! Program based on the GZCL method.  
//! Relative weights from SymmetricStrength.com.  
//!
//! The library provides useful primitives for workouts (lifts, sets, days, lift attempts, programs, etc.).
//! It also provides controllers/services/adapters to use these primitives to keep track of your workouts!
//!
//! Compile this as a binary to run in your terminal.

/// Interact with outside systems, like the filesystem or a TUI.
pub mod adapters;
/// Parse user input and ask a service to perform a relevant task.
pub mod controllers;
/// Coordinate multiple adapters to achieve a feature of the app.
pub mod services;

/// Useful primitives like Sets/Weights
pub mod lifting;
/// Useful primitives: the program
pub mod programs;
