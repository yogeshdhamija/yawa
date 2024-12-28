//! # YAWA: Yet Another Workout App
//! Keeps track of your lifts and weights.  
//! Program based on the GZCL method.  
//! Relative weights from SymmetricStrength.com.  
//!
//! The library provides useful primitives for workouts (lifts, sets, days, lift attempts, programs, etc.).
//! It also provides controllers/services/adapters to use these primitives to keep track of your workouts!
//!
//! Compile this as a binary to run in your terminal.

/// Contains the code to orchestrate how the application works (controllers, services, adapters)
pub mod application;

/// Contains domain objects and functions
pub mod domain;