//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{native_json_codec, CanonicalJsonTree, FromValue, NativeJsonBorrow, RetainedClone, RetainedEquality, RetireOwned, SnapshotProjection, ToValue};

pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::retirement_contract::RetirementPollExit;
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_native_owned::native_decoding;
pub use native_decoding::OriginalReceivingPrimary;
pub use semio_framework_value_native_owned::{NativeReceive,NativeReceiver};
pub use semio_framework_value_retirement::{retirement_fields_type,retirement_fields_seed,RetainedWorkOutcome,ErasedSnapshotRetirement,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,settle_original_retirement_work,run_original_retirement_work,run_original_retirement_loan,RetirementDemand,factory_ticket_demands,factory_ticket_controlled_demands,factory_ticket_original_demands,close_factory_ticket,close_factory_ticket_original,record_retirement_work,owned_retirement_birth_bytes,admit_owned_retirement,shared_retirement_birth_bytes,shared_retirement_allocation_bytes,admit_shared_retirement,artifact_retire_leaf,artifact_retirement_sequence,artifact_retire_struct,OriginalRetirementControl,OriginalFailureFunding,refill_original_failure_storage,original_failure_storage_birth_demand,original_failure_storage_birth_work_demand,original_failure_storage_release_demand,original_failure_storage_release_work_demand,RetirementSelection,select_retirement_cursor,select_factory_ticket,close_factory_ticket_controlled};
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;
pub use retained_clone::{RetainedAllocationEffect,RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, RetainedOwnedProjection};

pub use semio_framework_value_read as read;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};

#[cfg(feature="testkit")]
#[path="../../🧪️testing/🧾️wire/🦀️.rs"]
pub mod testing;

pub use semio_framework_value_retirement::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};

pub use semio_framework_value_factory as factory;
pub use semio_framework_value_factory::{factory_retire_owned,factory_retire_snapshot,factory_retire_framed,SnapshotRetirementFactory,ArtifactOwnedValueRetirementFactory,OwnedValueRetirementFactory,SharedValueRetirementFactory,FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryProducerReply,FactoryProducerReplyComplete,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
pub use semio_framework_value_shared_issuance as shared_issuance;
/// 🔒️ Producer paths emitted by the derive macros; never a consumer API.
#[doc(hidden)]
pub mod __private {
    pub use semio_framework_value_snapshot_projection::snapshot_projection;
}




pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
pub use semio_framework_value_factory::boxed::{FactoryBoxedValue,FactoryBoxedPublication};

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod internals {
    pub use super::types::testkit as types;
}
#[path="../../📝️text/🚶️utf8/🦀️.rs"]
pub mod retained_utf8;
pub use retained_utf8::{RetainedUtf8Cursor,RetainedUtf8CursorStep,RetainedUtf8Buffer,RetainedUtf8Append,RetainedUtf8BytesCursor,RetainedUtf8BytesStep};

pub use semio_framework_value_sort as sort;

pub use semio_framework_value_control_contract::{AuthoringWallet,AuthoredEffect,RetainedWorkPoll};
pub use semio_framework_value_retirement_contract::{OriginalRetirementCloseLoan,OriginalFailureStorage,OriginalFailureStoragePool,run_with_failure_storage};

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
#[cfg(test)]
#[path="../../♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect_fixture;

pub use semio_framework_value_persistent::work;
pub use semio_framework_value_persistent::list::persistent;
pub use semio_framework_value_persistent::list::{PagedListReleaseFailure,PagedListReleaseOutcome};
