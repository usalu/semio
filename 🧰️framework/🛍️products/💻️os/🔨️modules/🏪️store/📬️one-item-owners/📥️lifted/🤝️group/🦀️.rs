//! 📥️ Items of `member_group` that belong to `os-store-one-item-batch` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs`.

use std::sync::Arc;
use semio_framework_os_store_ledger::lifted_snapshot_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreStringRetirement};
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator};
use semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement;
use semio_framework_os_store_replay::lifted_snapshot_read::ArtifactStoreStringVectorRetirement;
use semio_framework_value::ValueError;
pub(super) struct MemberGroupPreparation {
    pub visibility: Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,
    pub history: Option<ArtifactStoreHistoryCommitReservation>,
    pub displaced: Option<ArtifactStoreDisplacedOwnerReservation>,
    pub applied: semio_framework_os_vcs::HistoryPageStack<String>,
    pub cursor: semio_framework_os_vcs::HistoryPageStack<String>,
    pub revision: Option<CursorRevisionAccumulator>,
    pub checkpoint: Option<String>,
    pub phase: u8,
    pub ordinal: usize,
    pub operation: usize,
    pub operation_indexed: bool,
    pub demand: usize,
    pub completed: u32,
}

impl MemberGroupPreparation {
    pub(super) fn new(visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, history: Option<ArtifactStoreHistoryCommitReservation>, displaced: Option<ArtifactStoreDisplacedOwnerReservation>) -> Self {
        Self { visibility: Arc::clone(visibility), history, displaced, applied: semio_framework_os_vcs::HistoryPageStack::empty(), cursor: semio_framework_os_vcs::HistoryPageStack::empty(), revision: None, checkpoint: None, phase: 0, ordinal: 0, operation: 0, operation_indexed: false, demand: 1, completed: 0 }
    }

    pub(super) fn next_byte_demand(&self) -> usize { self.demand }
    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.history.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.displaced.is_some() {
            let capacity_bytes = if self.applied.capacity() != 0 || self.cursor.capacity() != 0 { size_of::<ArtifactStoreStringVectorRetirement>() } else if self.revision.is_some() { size_of::<ArtifactStoreRevisionAccumulatorRetirement>() } else if self.checkpoint.is_some() { size_of::<ArtifactStoreStringRetirement>() } else { 0 };
            return Ok(RetirementDemand { capacity_bytes, depth: 1, ..Default::default() });
        }
        let release_bytes = if Arc::strong_count(&self.visibility) == 1 { std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<semio_framework_os_vcs::ArtifactGroupVisibility>()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "group visibility frame layout overflow"))?.0.pad_to_align().size() } else { 0 };
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }

    pub fn visibility(&self) -> Arc<semio_framework_os_vcs::ArtifactGroupVisibility> { Arc::clone(&self.visibility) }

    pub fn completed_items(&self) -> u32 { self.completed }
}
