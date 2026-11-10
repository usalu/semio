//! 🧳️ Native receiving constructs retained owners from genuine original input under explicit physical grants.
//! 🗒️ See [ownership and unsafe contracts](README.md).
use crate::{RetainedWorkOutcome,OriginalReceivingPrimary,NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
pub use semio_framework_value_control_contract::{OriginalSourceTurn,OriginalSourceTurnIssuer,OriginalSourceTurnWitness};
pub use semio_framework_value_native_shared_field::NativeIntrinsicSharedField;
pub use semio_framework_value_native_schema_engine::record;
pub use record::{NativeRecordKey,NativeRecordField,NativeSourceKeyReply,NativeSourceKeyReplyComplete,NativeSourceKeyValue,NativeSchemaLeaf,NativeSchemaFields,NativeSchemaKeyPoll,NativeSchemaKeyMetadata,NativeSchemaMatchSelection,NativeSchemaKeySource,NativeRecordSchema,NativeFieldMatch,NativeRecordMatcher,NativeRecordStorage};
#[path="📋️list-impl/🦀️.rs"]mod list_impl;
pub use semio_framework_value_native_list_receiver::{NativeListSource,NativePersistentListReceiver};

#[path="🔗️sealed-shared-impl/🦀️.rs"]mod shared_impl;
#[doc(hidden)]pub use shared_impl::NativeReceiverSeedFor;
pub use semio_framework_value_native_shared_engine::{NativeSharedReceiver,NativeSharedRoot};
pub use semio_framework_value_native_default::{native_admit_default,NativeDefault,NativeDefaultCursor,NativeDefaultValue,NativeDefaultValueCursor,NativeDefaultSlot,NativeArrayDefaultCursor,NativePairDefaultCursor};

#[path="🌳️value/🦀️.rs"]pub mod tree;
pub use tree::NativeReceiveSlot;
#[path="🗂️map/🦀️.rs"]pub mod map;
#[path="🧺️set/🦀️.rs"]pub mod set;

use semio_framework_value_native_receiver_protocol::receiving_child_body_budget;
pub use semio_framework_value_native_receiver_protocol::receiving_capacity;
pub use semio_framework_value_native_receiver_protocol::RetainedCursorRelease;

pub trait NativeReceive<I:?Sized>:RetireOwned+Sized{
 type Receiver:NativeReceiver<I,Self>;
 /// 🌱️ Produces only the inline original seed without allocating, releasing, calling out or unwinding.
 fn receiver()->Self::Receiver;
 /// 🌱️ Produces the optional inline original subset seed with the same pure contract.
 fn record_subset_receiver()->Option<Self::Receiver>{None}
 /// 🧬️ Returns only the matching factory-owned immutable seed template, when available.
 fn receiver_template(_subset:bool)->Option<semio_framework_value_native_storage::NativeSeedTemplate<Self::Receiver>>{None}
}

pub use semio_framework_value_native_receiver_protocol::NativeReceivingBatch;

/// 🧾️ Declares a native fixed record descriptor table.
///
/// # Safety
/// The count and every reachable descriptor table must be bounded, deterministic, allocation-free, callback-free and non-unwinding.
pub unsafe trait NativeRecordFields{
 fn native_schema_count()->usize;
 fn native_schema_field(index:usize)->Option<NativeRecordSchema>;
 fn native_schema_fields()->NativeSchemaFields{unsafe{NativeSchemaFields::native(Self::native_schema_field)}}
}

pub use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeReceiver,native_receiver_close_start,native_close_settle,native_relay,native_close_begun,native_close_denied,native_reply_poll};
pub use crate::native_decoding::construction::NativeRecoveryLoan;
pub use semio_framework_value_native_receiving_contract::{NativeClosePermit,native_work_limit,work_sum,selected_native_work};




pub use semio_framework_value_native_shared_slots::NativeSharedSlots;

#[path="🔄️map/🦀️.rs"]
pub mod mapped;
pub use mapped::{NativeReceiveMap,NativeMappedReceiver,NativeReceiveTryMap,NativeTryMappedReceiver};





pub use semio_framework_value_native_receiver_protocol::{native_output_demand,native_output_take,native_record_pure_take};
pub use semio_framework_value_native_receiver_protocol::{NativeReceivingChild,NativeLocalAuthority,NativeLocalProducer,NativeLocalOperation,native_poll_local,native_receive_local};
