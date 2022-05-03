use crate::lifting::{LiftAttempt, Program};
use crate::services::ports::PersistenceAdapter;
use anyhow::{anyhow, Result};

pub fn status(persistence_adapter: &impl PersistenceAdapter) -> Result<String> {
    with_program(persistence_adapter, |r| {
        format!("Current reference weight: {r}")
    })
}

pub fn next_show(
    persistence_adapter: &impl PersistenceAdapter,
) -> Result<(String, Vec<LiftAttempt>)> {
    with_program(persistence_adapter, |r| {
        let program = crate::programs::Gzcl4Day::start(r);
        (
            program.days().first().unwrap().name.clone(),
            program.next_workout(),
        )
    })
}

pub fn new_program(
    persistence_adapter: &impl PersistenceAdapter,
    reference_weight: u64,
) -> Result<()> {
    persistence_adapter.persist(reference_weight)?;
    Ok(())
}

fn with_program<F, R>(persistence_adapter: &impl PersistenceAdapter, closure: F) -> Result<R>
where
    F: FnOnce(u64) -> R,
{
    return if let Some(reference_weight) = persistence_adapter.summon() {
        Ok(closure(reference_weight))
    } else {
        Err(anyhow!("Start a lifting first!"))
    };
}
