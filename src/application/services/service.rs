use crate::application::services::ports::{PersistenceAdapter, UserInputAdapter};
use crate::domain::lifting::LiftAttempt;
use crate::domain::programs::{start_gzcl_4day, Program};
use anyhow::{anyhow, Error, Result};
use std::path::PathBuf;

const LIFTING_PROGRAM_NOT_STARTED_ERROR_MESSAGE: &'static str = "Start a lifting program first!";

fn not_started_error() -> Result<Program, Error> {
    Err(anyhow!(LIFTING_PROGRAM_NOT_STARTED_ERROR_MESSAGE))
}

pub fn apply_save_dir(
    persistence_adapter: impl PersistenceAdapter,
    maybe_dir: Option<PathBuf>,
) -> impl PersistenceAdapter {
    return if let Some(dir) = maybe_dir {
        let new_adapter = persistence_adapter.set_save_dir(&dir);
        new_adapter
    } else {
        persistence_adapter
    };
}

pub fn complete_workout(
    persistence_adapter: &impl PersistenceAdapter,
    user_input_adapter: &impl UserInputAdapter,
) -> Result<()> {
    let program = get_program(persistence_adapter)?;
    let lift_attempts = program.next_workout();
    let lift_results = user_input_adapter.check_complete(&lift_attempts)?;
    persistence_adapter.persist(&program.complete_workout(&lift_results))?;
    lift_attempts
        .iter()
        .enumerate()
        .try_for_each(|(index, attempt)| {
            persistence_adapter.save_history(&attempt, &lift_results[index])
        })?;
    Ok(())
}

pub fn next_workout(
    persistence_adapter: &impl PersistenceAdapter,
) -> Result<(String, Vec<LiftAttempt>)> {
    let program = get_program(persistence_adapter)?;
    Ok((
        program.days[program.current_day].name.clone(),
        program.next_workout(),
    ))
}

pub fn get_program(persistence_adapter: &impl PersistenceAdapter) -> Result<Program> {
    persistence_adapter.summon().or(not_started_error())
}

fn start_program(r: usize) -> Program {
    start_gzcl_4day(r)
}

pub fn start_and_save_new_program(
    persistence_adapter: &impl PersistenceAdapter,
    reference_weight: usize,
) -> Result<Program> {
    let program = start_program(reference_weight);
    persistence_adapter.persist(&program)?;
    return Ok(program);
}
