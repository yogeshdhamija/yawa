use anyhow::Result;
use std::path::Path;

use crate::lifting::{LiftAttempt, LiftAttemptResult};
use crate::programs::Program;

pub trait PersistenceAdapter {
    fn set_save_dir(self, dir: &Path) -> Self;
    fn persist(&self, program: &Program) -> Result<()>;
    fn save_history(&self, attempt: &LiftAttempt, result: &LiftAttemptResult) -> Result<()>;
    fn summon(&self) -> Result<Program>;
}

pub trait UserInputAdapter {
    fn check_complete(&self, attempts: &[LiftAttempt]) -> Result<Vec<LiftAttemptResult>>;
}
