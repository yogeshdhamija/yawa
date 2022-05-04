use crate::lifting::LiftAttempt;
use crate::programs::{start_gzcl_4day, Program};
use crate::services::ports::PersistenceAdapter;
use crate::UserInputAdapter;
use anyhow::{anyhow, Result};

pub fn get_current_program(persistence_adapter: &impl PersistenceAdapter) -> Result<Program> {
    with_program(persistence_adapter, |p| p)
}

pub fn complete_workout(
    persistence_adapter: &impl PersistenceAdapter,
    tui_adapter: &impl UserInputAdapter,
) -> Result<()> {
    with_program(persistence_adapter, |program| {
        let results = tui_adapter.check_complete(&program.next_workout())?;
        persistence_adapter.persist(&program.complete_workout(&results))
    })??;
    Ok(())
}

pub fn next_workout(
    persistence_adapter: &impl PersistenceAdapter,
) -> Result<(String, Vec<LiftAttempt>)> {
    with_program(persistence_adapter, |program| {
        (
            program.days[program.current_day as usize].name.clone(),
            program.next_workout(),
        )
    })
}

fn start_program(r: u64) -> Program {
    start_gzcl_4day(r)
}

pub fn start_and_save_new_program(
    persistence_adapter: &impl PersistenceAdapter,
    reference_weight: u64,
) -> Result<Program> {
    let program = start_program(reference_weight);
    persistence_adapter.persist(&program)?;
    return Ok(program);
}

fn with_program<F, R>(persistence_adapter: &impl PersistenceAdapter, closure: F) -> Result<R>
where
    F: FnOnce(Program) -> R,
{
    return if let Ok(program) = persistence_adapter.summon() {
        Ok(closure(program))
    } else {
        Err(anyhow!("Start a lifting first!"))
    };
}
