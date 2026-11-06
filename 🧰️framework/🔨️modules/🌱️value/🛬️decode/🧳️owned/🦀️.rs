//! 🧳️ Native receiving constructs retained owners from genuine original input under explicit physical grants.
//! 🗒️ See [ownership and unsafe contracts](README.md).
use crate::{RetainedWorkOutcome,OriginalReceivingPrimary,NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
pub use semio_framework_value_native_shared_engine::NativeIntrinsicSharedField;
pub use semio_framework_value_native_schema_engine::record;
pub use record::{NativeRecordKey,NativeRecordField,NativeSchemaLeaf,NativeSchemaKeySource,NativeRecordSchema,NativeFieldMatch,NativeRecordMatcher,NativeRecordStorage};
#[path="📋️list/🦀️.rs"]mod list;
pub use list::{NativeListSource,NativePersistentListReceiver};

#[path="🔗️sealed-shared-receiver/🦀️.rs"]mod shared;
pub use shared::{NativeSharedReceiver,NativeSharedRoot};
pub use semio_framework_value_native_default::{NativeDefault,NativeDefaultCursor,NativeDefaultValue,NativeDefaultValueCursor,NativeDefaultSlot,NativeArrayDefaultCursor,NativePairDefaultCursor};

#[path="🌳️value/🦀️.rs"]pub mod tree;
#[path="🗂️map/🦀️.rs"]pub mod map;
#[path="🧺️set/🦀️.rs"]pub mod set;

use semio_framework_value_native_receiver_protocol::receiving_child_body_budget;
pub use semio_framework_value_native_receiver_protocol::receiving_capacity;

pub trait NativeReceive<I:?Sized>:RetireOwned+Sized{
 type Receiver:NativeReceiver<I,Self>;
 /// 🌱️ Produces only the inline original seed without allocating, releasing, calling out or unwinding.
 fn receiver()->Self::Receiver;
 /// 🌱️ Produces the optional inline original subset seed with the same pure contract.
 fn record_subset_receiver()->Option<Self::Receiver>{None}
}

pub use semio_framework_value_native_receiver_protocol::NativeReceivingBatch;

pub trait NativeRecordFields{
 fn native_schema_count()->usize;
 fn native_schema_field(index:usize)->Option<NativeRecordSchema>;
}

pub use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeReceiver};




pub use semio_framework_value_native_shared_engine::NativeSharedSlots;

#[path="🔄️map/🦀️.rs"]
pub mod mapped;
pub use mapped::{NativeReceiveMap,NativeMappedReceiver,NativeReceiveTryMap,NativeTryMappedReceiver};



pub use semio_framework_value_native_receiver_protocol::{native_receiver_backing_demands,close_native_receiver_backing};

#[cfg(test)]#[path="♻️header/🎟️work/🧪️tests/🦀️.rs"]mod header_retirement_tests;

pub use semio_framework_value_native_receiver_protocol::{native_output_demand,native_output_take,native_record_pure_take};
