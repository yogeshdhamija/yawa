use crate::lifting::{LiftAttempt, LiftAttemptResult};
use crate::services::ports::UserInputAdapter;
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
            // let mut user_input =
            // io::stdin().read_line()
        }

        Ok(results)
    }
}
