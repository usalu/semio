//! 🧳️ Native receiving of retained owners — wiring only. Domain lives at the owner `🦀️.rs` files.

pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue, Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeOwnedDecodeControl, OriginalReceivingPrimary};
pub use semio_framework_value_persistent::list;
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::OriginalRetirementOutcome;
pub use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../🦀️.rs"]
pub mod owned;
pub use owned::{NativeReceive, NativeReceiver, NativeSharedSlots};
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct RetainedCloneGrant{maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct RetainedCloneProgress{copied_items:usize,copied_bytes:usize,retained_capacity_bytes:usize,released_bytes:usize}}

pub mod native_decoding {
    pub use semio_framework_value_native_decoding::native_decoding::*;
    pub use super::owned;
    pub use super::owned::{NativeReceive, NativeReceiver, NativeSharedSlots};
}

#[cfg(test)]
pub use semio_framework_value_derive::{NativeReceive, RetireOwned, ToValue};
#[cfg(test)]
pub use semio_framework_value_intrinsic::{DecodedValue, FromValue, NativeEncodeControl, ToValue, ValueEdit, ValueShape};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[cfg(test)]pub(crate) use semio_framework_value_testing_admission as source_fixture;
