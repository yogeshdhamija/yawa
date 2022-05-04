use crate::lifting::{LiftAttempt, LiftAttemptResult, WeightScheme};
use crate::services::ports::UserInputAdapter;
use dialoguer::Confirm;
use std::io;

pub struct Tui {}
pub fn new() -> Tui {
    Tui {}
}

impl UserInputAdapter for Tui {
    fn check_complete(
        &self,
        attempts: &[LiftAttempt],
    ) -> Result<Vec<LiftAttemptResult>, anyhow::Error> {
        let mut results = Vec::new();

        for attempt in attempts {
            match attempt.lift.weight {
                WeightScheme::BasedOnReference { .. } => {
                    Confirm::new()
                        .with_prompt(format!("Did you complete:\n{}?", attempt))
                        .interact()?;
                }
                WeightScheme::LinearBasedOnPrevious { .. } => {}
                WeightScheme::Any => {}
                WeightScheme::None => {}
            }
        }

        Ok(results)
    }
}
