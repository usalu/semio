//! ♻️ Neutral retained-owner retirement and typed factories.
use std::sync::Arc;
use crate::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetirementDemand {
    pub copy_bytes: usize,
    pub capacity_bytes: usize,
    pub release_bytes: usize,
    pub depth: usize,
}
pub trait ErasedSnapshotRetirement: Send {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_depth_demand(&self) -> Result<usize, ValueError>;
    /// 📏️ Quotes all four axes of the next turn in one pass. A nesting owner overrides it to ask its child once: the four axis methods each re-derive the child's quote, so a chain of `d` wrappers would otherwise cost `4^d` quotes per turn.
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? })
    }
}

pub trait SnapshotRetirementFactory<P>: crate::FactoryRetirement + Send + Sync {
    /// 📦️ Publishes the retained cursor birth allocation before snapshot ownership transfers.
    fn retirement_birth_bytes(&self, snapshot: &Arc<P>) -> usize;
    fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)>;
}

pub trait ArtifactOwnedValueRetirementFactory<T>: crate::FactoryRetirement + Send + Sync {
    fn retirement_birth_bytes(&self, value: &T) -> usize;
    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)>;
}
