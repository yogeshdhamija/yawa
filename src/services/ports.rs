use anyhow::Result;

pub trait PersistenceAdapter {
    fn persist(&self, reference_weight: u64) -> Result<()>;
    fn summon(&self) -> Option<u64>;
}
