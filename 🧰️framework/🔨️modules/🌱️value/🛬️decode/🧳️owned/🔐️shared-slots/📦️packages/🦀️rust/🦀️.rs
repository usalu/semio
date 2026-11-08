//! 🔐️ Native shared input slots and original field ownership.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure,OriginalRetirementCloseLoan};
pub use semio_framework_value_shared_issuance as shared_issuance;
pub use semio_framework_value_native_shared_slots::NativeSharedSlots;
pub use semio_framework_value_native_shared_field::NativeIntrinsicSharedField;

pub use semio_framework_value_native_receiver_protocol::{NativeReceiver,NativeReceivingBatch,native_receiver_backing_demands,NativeReceiverBackingClose};
#[path="../../../🔗️sealed-shared-receiver/🦀️.rs"]mod shared_receiver;
pub use shared_receiver::{NativeSharedReceiver,NativeSharedRoot};
pub use semio_framework_value_native_receiver_protocol::NativeReceiverSeed;

#[cfg(test)]pub(crate) use semio_framework_value_intrinsic::DslValue;
#[cfg(test)]pub(crate) use semio_framework_value_testing_admission as source_fixture;
#[cfg(test)]pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
