//! 📥️ Items of `member_group` that belong to `os-store-one-item-batch` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs`.

use std::sync::Arc;
use semio_framework_os_store_ledger::lifted_snapshot_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreStringRetirement};
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator};
use semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement;
use semio_framework_os_store_replay::lifted_snapshot_read::ArtifactStoreStringVectorRetirement;
use semio_framework_value::ValueError;
/// 📚️ Private commit metadata is prepared one original row and paid backing at a time.
pub struct BatchCommitMetadata {
    pub edit_id_digest:Option<[u8;32]>,
    pub cursor_redo:Option<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub log_redo:Option<semio_framework_os_vcs::HistoryPageStack<String>>,
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

/// 🤝️ A member commit binds its private metadata to the genuine common decision.
pub struct MemberGroupPreparation{
    pub visibility:Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,
    pub metadata:BatchCommitMetadata,
}
impl BatchCommitMetadata{
    pub fn new(history:Option<ArtifactStoreHistoryCommitReservation>,displaced:Option<ArtifactStoreDisplacedOwnerReservation>)->Self{Self{edit_id_digest:None,cursor_redo:None,log_redo:None,history,displaced,applied:semio_framework_os_vcs::HistoryPageStack::empty(),cursor:semio_framework_os_vcs::HistoryPageStack::empty(),revision:None,checkpoint:None,phase:0,ordinal:0,operation:0,operation_indexed:false,demand:1,completed:0}}
    pub fn retirement_demands(&self)->Option<semio_framework_value::RetirementDemand>{
        use semio_framework_value::RetirementDemand;
        if self.history.is_some(){return Some(RetirementDemand{depth:1,..Default::default()})}
        self.displaced.as_ref().map(|_|{let capacity_bytes=if self.applied.capacity()!=0||self.cursor.capacity()!=0||self.cursor_redo.is_some()||self.log_redo.is_some(){size_of::<ArtifactStoreStringVectorRetirement>()}else if self.revision.is_some(){size_of::<ArtifactStoreRevisionAccumulatorRetirement>()}else if self.checkpoint.is_some(){size_of::<ArtifactStoreStringRetirement>()}else{0};RetirementDemand{capacity_bytes,depth:1,..Default::default()}})
    }
}
impl MemberGroupPreparation {
    pub fn new(visibility:&Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,history:Option<ArtifactStoreHistoryCommitReservation>,displaced:Option<ArtifactStoreDisplacedOwnerReservation>)->Self{Self{visibility:Arc::clone(visibility),metadata:BatchCommitMetadata::new(history,displaced)}}
    pub(super) fn next_byte_demand(&self)->usize{self.metadata.demand}
    pub fn retirement_demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if let Some(demand)=self.metadata.retirement_demands(){return Ok(demand)}
        let release_bytes=if Arc::strong_count(&self.visibility)==1{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<semio_framework_os_vcs::ArtifactGroupVisibility>()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"group visibility frame layout overflow"))?.0.pad_to_align().size()}else{0};
        Ok(semio_framework_value::RetirementDemand{release_bytes,depth:1,..Default::default()})
    }
    pub fn visibility(&self)->Arc<semio_framework_os_vcs::ArtifactGroupVisibility>{Arc::clone(&self.visibility)}
    pub fn completed_items(&self)->u32{self.metadata.completed}
}
