use crate::programs::Gzcl4Day;
use anyhow::Result;

pub trait PersistenceAdapter {
    fn persist(&self, program: &Gzcl4Day) -> Result<()>;
    fn summon(&self) -> Result<Gzcl4Day>;
}
