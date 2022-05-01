use crate::services::ports::PersistanceAdapter;
use anyhow::{anyhow, Result};

pub fn status(persistance_adapter: &impl PersistanceAdapter) -> Result<String> {
    with_program(persistance_adapter, |r| {
        format!("Current reference weight: {r}")
    })
}

pub fn next_show(persistance_adapter: &impl PersistanceAdapter) -> Result<String> {
    with_program(persistance_adapter, |r| {
        format!("Current reference weight: {r}")
    })
}

pub fn new_program(
    persistance_adapter: &impl PersistanceAdapter,
    reference_weight: u64,
) -> Result<()> {
    persistance_adapter.persist(reference_weight)?;
    Ok(())
}

fn with_program<F, R>(persistance_adapter: &impl PersistanceAdapter, closure: F) -> Result<R>
where
    F: FnOnce(u64) -> R,
{
    if let Some(reference_weight) = persistance_adapter.summon() {
        return Ok(closure(reference_weight));
    } else {
        return Err(anyhow!("Start a program first!"));
    }
}
