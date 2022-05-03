use crate::programs::Program;
use anyhow::Result;

pub trait PersistenceAdapter {
    fn persist(&self, program: &Program) -> Result<()>;
    fn summon(&self) -> Result<Program>;
}
