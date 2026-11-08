//! 🔺️ Indexed and named collection edit plans over persistent lists — wiring only. Domain lives at the owner `🦀️.rs` files.

pub use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedAllocationEffect,OriginalRetirementCloseLoan,OriginalRetirementFailure};
pub use semio_framework_value_retirement::OriginalRetirementOutcome;
#[cfg(test)]
pub use semio_framework_value_retirement::{retirement_fields_type,retirement_fields_seed,OriginalRetirementControl,RetirementPollExit,OriginalRetirementPoll,RetainedWorkPoll};
use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_intrinsic::{DslValue, Number};
pub use semio_framework_value_native_decoding::OriginalReceivingPrimary;
use semio_framework_value_native_decoding::{NativeDecodeControl};
use semio_framework_value_native_owned::native_decoding;
use semio_framework_value_persistent::{list};
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_retirement::{artifact_retire_leaf, retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

pub use semio_framework_value_sort as sort;
pub use sort::{HeapSortCursor, HeapSortFault, HeapSortWork};
#[path = "../../🦀️.rs"]
pub mod indexed_edit;
pub(crate) use indexed_edit::{record_owed,finish,allocate_empty_vec,Settle,Polled,Exit};
pub use indexed_edit::{IndexedEditShape,IndexedEditKind,IndexedEditEntry,IndexedEditFault,IndexedEditStep,IndexedEditPlan,IndexedOutputWork,IndexedOutputCursor};
#[path = "../../🚚️commit/🦀️.rs"]
pub mod indexed_collection_commit;
pub use indexed_collection_commit::IndexedCollectionCommit;
#[path = "../../🔑️named/🦀️.rs"]
pub mod named_edit;
pub use named_edit::{NamedEditLane,NamedEditSource,NamedEditFault,NamedEditStep,NamedEditPlan,NamedRankedSource,VecRankedKeys,PersistentRankedKeys,NamedKeyIndexPreparation,AdmittedNamedKeyIndex,IndexedNamedSource,KeyPreservingPersistentSet,NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};

#[cfg(test)]
/// 🧱️ Every staged source admission turn is funded separately; its largest header placement stays far below one page.
pub(crate) const ADMISSION_STAGE_COPY_BYTES:usize=4096;
#[cfg(test)]
/// 🎟️ Funds the complete admission quote of one value, copy included, on top of the caller's authority.
pub(crate) fn admit_funded<T:Send+Sync+'static>(value:T,issuer:shared_issuance::SharedIssuer<T>,mut grant:RetainedCloneGrant)->RetainedWorkOutcome<(shared_issuance::SealedShared<T>,RetainedCloneProgress),(ValueError,T)>{let d=shared_issuance::SealedShared::<T>::admission_demand().unwrap();grant.maximum_copy_bytes=grant.maximum_copy_bytes.max(d.copy_bytes);shared_issuance::SealedShared::admit(value,issuer,grant,&mut None)}
#[cfg(test)]
#[path="../../../../♻️retirement/🧪️tests/⏱️effect/🦀️.rs"]
pub(crate) mod effect;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) mod value {
    pub(crate) use super::observe_retirement_allocations;
}
