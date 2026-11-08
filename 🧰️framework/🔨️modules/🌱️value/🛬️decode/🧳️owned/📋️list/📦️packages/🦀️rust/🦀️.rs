//! 📋️ Canonical persistent-list native receiver engine.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue,Number};
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
pub use semio_framework_value_native_receiver_protocol::{NativeReceiver,NativeReceiverSeed,NativeReceivingBatch,receiving_child_body_budget,native_receiver_backing_demands,close_native_receiver_backing};
pub use semio_framework_value_native_shared_engine::NativeSharedSlots;
pub use semio_framework_value_persistent::list;
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure,OriginalRetirementCloseLoan};
pub use semio_framework_value_shared_issuance as shared_issuance;
#[path="../../🦀️.rs"]mod list_receiver;
pub use list_receiver::{NativeListSource,NativePersistentListReceiver};
#[cfg(test)]pub use semio_framework_value_native_owned::owned::{NativeReceive,NativeRecordFields,NativeRecordSchema};
#[cfg(test)]pub use semio_framework_value_derive::{NativeReceive,RetireOwned};
#[cfg(test)]pub use semio_framework_value_retirement::retirement::RetireOwned;
#[cfg(test)]pub(crate) use semio_framework_value_testing_admission as source_fixture;
#[cfg(test)]pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
