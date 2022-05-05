use crate::lifting::LiftAttempt;
use crate::services::ports::{PersistenceAdapter, UserInputAdapter};
use crate::services::service;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a new weightlifting routine! Let's GOOOoOoOo!!!!!1
    Start {
        /// The reference weight to start with (in lbs). 45 is a good number to start with if it's
        /// your first time.
        #[clap(short)]
        reference_weight: usize,
    },

    /// Display current status of your lifting lifting.
    Status {},

    /// Show the next workout in your program.
    Next {},

    /// Complete the next workout of your program! (Run 'next' to see it first!)
    Complete {},
}

pub fn execute_based_on_args(
    persistence_adapter: &impl PersistenceAdapter,
    tui_adapter: &impl UserInputAdapter,
) -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Status {} => status(persistence_adapter)?,
        Commands::Start { reference_weight } => start(persistence_adapter, reference_weight)?,
        Commands::Next {} => next(persistence_adapter)?,
        Commands::Complete {} => complete(persistence_adapter, tui_adapter)?,
    };
    Ok(())
}

fn complete(
    persistence_adapter: &impl PersistenceAdapter,
    tui_adapter: &impl UserInputAdapter,
) -> Result<()> {
    service::complete_workout(persistence_adapter, tui_adapter)?;
    println!("Well done!");
    Ok(())
}

fn next(persistence_adapter: &impl PersistenceAdapter) -> Result<()> {
    let (day_name, lift_attempts) = service::next_workout(persistence_adapter)?;
    println!(
        "{}",
        format!("=== Day: {} ===\n{}", day_name, to_string(&lift_attempts))
    );
    Ok(())
}

fn to_string(lift_attempts: &Vec<LiftAttempt>) -> String {
    let lifts = lift_attempts
        .iter()
        .map(|lift| lift.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    lifts
}

fn start(persistence_adapter: &impl PersistenceAdapter, reference_weight: &usize) -> Result<()> {
    let program = service::start_and_save_new_program(persistence_adapter, *reference_weight)?;
    println!("Started program: {}", program.name);
    Ok(())
}

fn status(persistence_adapter: &impl PersistenceAdapter) -> Result<()> {
    let program = service::get_program(persistence_adapter)?;
    println!(
        "Current program: {}\nCurrent reference weight: {}\nStarting reference weight: {}\nWorkouts completed: {}",
        program.name, program.reference_weight, program.starting_reference_weight, program.workouts_completed
    );
    Ok(())
}
