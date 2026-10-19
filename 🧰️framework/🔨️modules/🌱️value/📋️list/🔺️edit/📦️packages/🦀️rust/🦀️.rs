//! 🔺️ Indexed and named collection edit plans over persistent lists — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_intrinsic::{DslValue, Number};
use semio_framework_value_native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl};
use semio_framework_value_native_owned::native_decoding;
use semio_framework_value_persistent::{list, original_vector_concat::OriginalVectorConcat};
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_retirement::{artifact_retire_leaf, retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../../../📶️sort/🦀️.rs"]
pub mod sort;
pub use sort::{HeapSortCursor, HeapSortFault, HeapSortWork};
#[path = "../../🦀️.rs"]
pub mod indexed_edit;
pub use indexed_edit::{IndexedEditShape,IndexedEditKind,IndexedEditEntry,IndexedEditFault,IndexedEditStep,IndexedEditPlan,IndexedOutputWork,IndexedOutputCursor};
#[path = "../../🚚️commit/🦀️.rs"]
pub mod indexed_collection_commit;
pub use indexed_collection_commit::IndexedCollectionCommit;
#[path = "../../🔑️named/🦀️.rs"]
pub mod named_edit;
pub use named_edit::{NamedEditLane,NamedEditSource,NamedEditFault,NamedEditStep,NamedEditPlan,NamedRankedSource,VecRankedKeys,PersistentRankedKeys,NamedKeyIndexPreparation,AdmittedNamedKeyIndex,IndexedNamedSource,KeyPreservingPersistentSet,NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) mod value {
    pub(crate) use super::observe_retirement_allocations;
}
