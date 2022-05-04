use anyhow::Result;

use crate::lifting::{LiftAttempt, LiftAttemptResult};
use crate::programs::Program;

pub trait PersistenceAdapter {
    fn persist(&self, program: &Program) -> Result<()>;
    fn summon(&self) -> Result<Program>;
}

pub trait UserInputAdapter {
    fn check_complete(&self, attempts: &[LiftAttempt]) -> Result<Vec<LiftAttemptResult>>;
}
