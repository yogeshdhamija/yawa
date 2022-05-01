use crate::services::ports::PersistanceAdapter;
use anyhow::{anyhow, Result};

pub fn status(persistance_adapter: &impl PersistanceAdapter) -> Result<String> {
    if let Some(reference_weight) = persistance_adapter.summon() {
        return Ok(format!("Current reference weight: {reference_weight}"));
    } else {
        return Err(anyhow!("No status. Start a program first!"));
    }
}

pub fn next_show(persistance_adapter: &impl PersistanceAdapter) -> Result<String> {
    if let Some(reference_weight) = persistance_adapter.summon() {
        return Ok(format!("Current reference weight: {reference_weight}"));
    } else {
        return Err(anyhow!(
            "Can't display next workout. Start a program first!"
        ));
    }
}
