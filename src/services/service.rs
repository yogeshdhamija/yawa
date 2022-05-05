use crate::lifting::LiftAttempt;
use crate::programs::{start_gzcl_4day, Program};
use crate::services::ports::{PersistenceAdapter, UserInputAdapter};
use anyhow::{anyhow, Error, Result};

const LIFTING_PROGRAM_NOT_STARTED_ERROR_MESSAGE: &'static str = "Start a lifting program first!";

fn not_started_error() -> Result<Program, Error> {
    Err(anyhow!(LIFTING_PROGRAM_NOT_STARTED_ERROR_MESSAGE))
}

pub fn complete_workout(
    persistence_adapter: &impl PersistenceAdapter,
    user_input_adapter: &impl UserInputAdapter,
) -> Result<()> {
    let program = get_program(persistence_adapter)?;
    let results = user_input_adapter.check_complete(&program.next_workout())?;
    persistence_adapter.persist(&program.complete_workout(&results))?;
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
