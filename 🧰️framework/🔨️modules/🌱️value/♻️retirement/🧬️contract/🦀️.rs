//! ♻️ Neutral retained-owner retirement and typed factories.
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotRetirementStep {
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

pub trait ErasedSnapshotRetirement: Send {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError>;
    fn terminal_is_empty(&self) -> bool;

    /// 📏️ The SMALLEST byte grant this retirement's next `close_step` can spend to make physical
    /// progress. A heap allocation is freed whole or not at all, so a retirement whose next unit is
    /// one owned buffer legitimately answers that buffer's size and answers `Blocked` (or
    /// `Pending { 0, 0 }`) below it — which leaves a NESTED driver, holding only a
    /// `Box<dyn ErasedSnapshotRetirement>`, unable to tell "I under-granted" from "I am waiting on
    /// someone else". A driver reads this BEFORE it grants, and pays the demand out of its own
    /// allocation admission rather than out of the caller's payload page.
    ///
    /// 🧱️ Defaulted to 1 — the byte-free answer of every retirement that releases owners rather than
    /// buffers, which is most of them — so no existing implementor changes shape. The inherent
    /// `next_close_byte_demand` methods that already exist on concrete retirements
    /// (`🌱️value/🗂️ordered`, `🌊️flow/🧵️retained`) are the same quantity; this is the erased view of
    /// it, the one a `Box<dyn ErasedSnapshotRetirement>` holder can actually reach.
    fn next_close_byte_demand(&self) -> usize {
        1
    }
}

pub trait SnapshotRetirementFactory<P>: crate::FactoryRetirement + Send + Sync {
    /// 📦️ Publishes the retained cursor birth allocation before snapshot ownership transfers.
    fn retirement_birth_bytes(&self, snapshot: &Arc<P>) -> usize;
    fn retire(&self, snapshot: Arc<P>) -> Box<dyn ErasedSnapshotRetirement>;
}

pub trait ArtifactOwnedValueRetirementFactory<T>: crate::FactoryRetirement + Send + Sync {
    fn retire_owned(&self, value: T) -> Box<dyn ErasedSnapshotRetirement>;
}
