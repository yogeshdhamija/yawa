use crate::lifting::{LiftAttempt, LiftAttemptResult, Set};
use crate::services::ports::UserInputAdapter;
use anyhow::Result;
use dialoguer::Confirm;
use std::io;
use std::io::{Read, Write};

pub struct Tui {}
pub fn new() -> Tui {
    Tui {}
}

impl UserInputAdapter for Tui {
    fn check_complete(&self, attempts: &[LiftAttempt]) -> Result<Vec<LiftAttemptResult>> {
        let mut results = Vec::new();

        for attempt in attempts {
            if attempt.has_rep_range() {
                if get_user_confirmation(&format!("Did you complete: {}?", attempt))? {
                    results.push(LiftAttemptResult::Completed {
                        completed_maximum_reps: get_user_confirmation(
                            &"        ... were you able to achieve the maximum rep range?",
                        )?,
                    })
                } else {
                    results.push(LiftAttemptResult::NotCompleted)
                }
            } else {
                if get_user_confirmation(&format!("Did you complete: {}?", attempt))? {
                    results.push(LiftAttemptResult::Completed {
                        completed_maximum_reps: true,
                    })
                } else {
                    results.push(LiftAttemptResult::NotCompleted)
                }
            }
        }

        Ok(results)
    }
}

fn get_user_confirmation(prompt: &str) -> Result<bool> {
    loop {
        print!("{} [y/n] ", prompt);
        io::stdout().flush()?;
        let mut string = String::new();
        io::stdin().read_line(&mut string)?;
        if string.trim() == "y" {
            return Ok(true);
        } else if string.trim() == "n" {
            return Ok(false);
        }
    }
}

impl LiftAttempt {
    fn has_rep_range(&self) -> bool {
        self.lift
            .sets
            .iter()
            .any(|it| matches!(it, Set::Range { .. }))
    }
}
