use crate::lifting::LiftAttempt;
use crate::services::ports::{PersistenceAdapter, UserInputAdapter};
use crate::services::service;
use crate::services::service::apply_save_dir;
use crate::user_input::Action;
use anyhow::Result;

pub fn start_ephemeral_interface(
    persistence_adapter: impl PersistenceAdapter,
    user_input_adapter: &impl UserInputAdapter,
) -> Result<()> {
    let (action, save_path) = user_input_adapter.ask_what_to_do()?;
    let persistence_adapter = apply_save_dir(persistence_adapter, save_path);
    match action {
        Action::SeeStatus => status(&persistence_adapter)?,
        Action::StartProgram { reference_weight } => start(&persistence_adapter, &reference_weight)?,
        Action::SeeNextDay => next(&persistence_adapter)?,
        Action::CompleteDay => complete(&persistence_adapter, user_input_adapter)?,
    };
    Ok(())
}

fn complete(
    persistence_adapter: &impl PersistenceAdapter,
    user_input_adapter: &impl UserInputAdapter,
) -> Result<()> {
    service::complete_workout(persistence_adapter, user_input_adapter)?;
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
