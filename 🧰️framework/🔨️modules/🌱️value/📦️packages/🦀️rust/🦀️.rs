//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{CanonicalJsonTree, FromValue, NativeReceive, RetainedClone, RetainedEquality, RetireOwned, SnapshotProjection, ToValue};

pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,RetirementDemand,factory_ticket_demands,close_factory_ticket,owned_retirement_birth_bytes,admit_owned_retirement,shared_retirement_birth_bytes,shared_retirement_allocation_bytes,admit_shared_retirement,artifact_retire_leaf,artifact_retirement_sequence,artifact_retire_struct};
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;
pub use retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, RetainedOwnedProjection};

#[path = "../../🔗️read/🦀️.rs"]
pub mod read;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};

#[cfg(feature="testkit")]
#[path="../../🧪️testing/🧾️wire/🦀️.rs"]
pub mod testing;

pub use semio_framework_value_retirement::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};
pub use semio_framework_value_original_aliases::OriginalAliasBatch;

pub use semio_framework_value_factory as factory;
pub use semio_framework_value_factory::{SnapshotRetirementFactory,ArtifactOwnedValueRetirementFactory,FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryRetirementAdmissionError,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
pub use semio_framework_value_shared_issuance as shared_issuance;

pub use semio_framework_value_retirement_queue as retirement_queue;



pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
pub use semio_framework_value_factory::boxed::{FactoryBoxedValue,FactoryBoxedPublication};

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod internals {
    pub use super::types::testkit as types;
}
#[path = "../../📶️sort/🦀️.rs"]
pub mod sort;
pub use sort::{HeapSortCursor, HeapSortFault, HeapSortWork};
#[path="../../📋️list/🔺️edit/🦀️.rs"]
pub mod indexed_edit;
pub use indexed_edit::{IndexedEditShape,IndexedEditKind,IndexedEditEntry,IndexedEditFault,IndexedEditStep,IndexedEditPlan,IndexedOutputWork,IndexedOutputCursor};
#[path="../../📋️list/🔺️edit/🚚️commit/🦀️.rs"]
pub mod indexed_collection_commit;
pub use indexed_collection_commit::IndexedCollectionCommit;
#[path="../../📋️list/🪢️concat/🦀️.rs"]
pub mod original_vector_concat;
pub use original_vector_concat::OriginalVectorConcat;
#[path="../../🔑️lookup/🦀️.rs"]
pub mod exact_key_lookup;
pub use exact_key_lookup::ExactKeyCursor;
#[path="../../📋️list/🔺️edit/🔑️named/🦀️.rs"]
pub mod named_edit;
pub use named_edit::{NamedEditLane,NamedEditSource,NamedEditFault,NamedEditStep,NamedEditPlan,NamedRankedSource,VecRankedKeys,PersistentRankedKeys,NamedKeyIndexPreparation,AdmittedNamedKeyIndex,IndexedNamedSource,KeyPreservingPersistentSet,NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};
#[path="../../📋️list/🟰️equal/🦀️.rs"]
pub mod slice_equality;
pub use slice_equality::{RetainedEqualityElement,RetainedSliceEquality};
#[path="../../📝️text/🚶️utf8/🦀️.rs"]
pub mod retained_utf8;
pub use retained_utf8::{RetainedUtf8Cursor,RetainedUtf8CursorStep,RetainedUtf8Buffer,RetainedUtf8Append};
