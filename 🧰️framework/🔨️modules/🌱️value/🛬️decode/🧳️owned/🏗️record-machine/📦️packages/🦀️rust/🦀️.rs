//! 🏗️ The native record receiving machine and its field plumbing, below the receive trait. Moved here from `semio-framework-value-native-owned` because that crate exceeded its 262144 byte source boundary; edit `../../🦀️.rs` (machine), `../../🪡️field/🦀️.rs` and the slot adapters in `../../🧩️slot/🦀️.rs`, which stay in the owner.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue,Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,NativeOwnedDecodeControl,OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use retirement::RetireOwned;
pub use retained_clone::RetainedCloneRef;
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure,OriginalRetirementControl,retirement_fields_type,retirement_fields_seed,OriginalRetirementCloseLoan};
pub use semio_framework_value_native_schema_engine::record;
pub use record::{answer_schema_key,NativeRecordKey,NativeRecordField,NativeSourceKeyReply,NativeSourceKeyReplyComplete,NativeSourceKeyValue,NativeSchemaKeySource,NativeRecordStorage};
pub use semio_framework_value_native_schema::{NATIVE_SCHEMA_METADATA_ITEMS,NativeSchemaLeaf,NativeSchemaFields,NativeSchemaKeyPoll,NativeSchemaKeyMetadata,NativeSchemaMatchSelection,NativeRecordSchema,NativeFieldMatch,NativeRecordMatcher};
pub use semio_framework_value_native_default::{native_admit_default,NativeDefault,NativeDefaultCursor,NativeDefaultValue,NativeDefaultValueCursor,NativeDefaultSlot,NativeArrayDefaultCursor,NativePairDefaultCursor};
pub use semio_framework_value_native_receiver_protocol::{receiving_capacity,RetainedCursorRelease,NativeAdvanceSelection,NativeReceiver,native_receiver_close_start,native_close_settle,native_relay,native_close_begun,native_close_denied,native_reply_poll,native_output_demand,native_output_take,native_record_pure_take,SlotFrame,OwnedFrame,SharedFrame,NativeReceivingChild,NativeLocalAuthority,NativeLocalProducer,NativeLocalOperation,native_poll_local,native_receive_local,NativeReceivingBatch};
pub use semio_framework_value_native_receiving_contract::{NativeClosePermit,native_work_limit,work_sum,selected_native_work};
pub use semio_framework_value_native_storage::NativeSeedTemplate;

pub mod native_decoding{
    pub use semio_framework_value_native_decoding::native_decoding::*;
    pub use semio_framework_value_native_storage::{NativeOwnedStorage,NativeSeedTemplate};
}

#[path="../../🪡️field/🦀️.rs"]
pub mod tree;
#[path="../../🦀️.rs"]
pub mod machine;
#[path="../../🎯️selector/🦀️.rs"]
pub mod selector;
#[path="../../🔀️enum/🦀️.rs"]
pub mod enum_machine;
