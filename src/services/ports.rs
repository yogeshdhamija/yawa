use anyhow::Result;

pub trait PersistanceAdapter {
    fn persist(&self, reference_weight: u64) -> Result<()>;
    fn summon(&self) -> Option<u64>;
}
