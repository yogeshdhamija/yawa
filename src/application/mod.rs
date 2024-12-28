/// Interact with outside systems, like the filesystem or a TUI.
pub mod adapters;
/// Parse user input and ask a service to perform a relevant task.
pub mod controllers;
/// Coordinate multiple adapters to achieve a feature of the app.
pub mod services;
