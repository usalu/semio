//! 🧳️ Native receiving of retained owners — wiring only. Domain lives at the owner `🦀️.rs` files.

pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue, Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeOwnedDecodeControl, OriginalReceivingPrimary};
pub use semio_framework_value_native_decoding::native_decoding::NativeDecodeAllocationOutcome;
pub use semio_framework_value_native_decoding::native_encoding::NativeEncodeAllocationOutcome;
pub use semio_framework_value_native_decoding::native_encoding::NativeEncodeEffectFailure;
pub use semio_framework_value_persistent::list;
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure};
pub use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../🦀️.rs"]
pub mod owned;
pub use owned::{NativeReceive, NativeReceiver, NativeSharedSlots};
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct NativeDecodeAllocationOutcome{#[value(required)]request:Option<NativeAllocationRequest>,admitted:bool,#[value(required)]primary:Option<ValueError>,#[value(required)]quote_refusal:Option<NativeAllocationRefusal>,#[value(required)]extent_refusal:Option<ValueError>,#[value(required)]settlement_refusal:Option<NativeAllocationRefusal>,#[value(required)]actual:Option<usize>}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct NativeEncodeAllocationOutcome{#[value(required)]request:Option<NativeAllocationRequest>,admitted:bool,#[value(required)]primary:Option<ValueError>,#[value(required)]forwarded_quote:Option<NativeAllocationRefusal>,#[value(required)]scoped_quote:Option<NativeAllocationRefusal>,#[value(required)]extent_refusal:Option<ValueError>,#[value(required)]forwarded_refusal:Option<NativeAllocationRefusal>,#[value(required)]scoped_refusal:Option<NativeAllocationRefusal>,#[value(required)]actual:Option<usize>}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct RetainedCloneGrant{maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct NativeAllocationFailure{#[value(required)]primary:Option<ValueError>,#[value(required)]admission:Option<ValueError>,#[value(required)]forwarded_admission:Option<NativeAllocationRefusal>,#[value(required)]scoped_admission:Option<NativeAllocationRefusal>,#[value(required)]forwarded_quote:Option<NativeAllocationRefusal>,#[value(required)]scoped_quote:Option<NativeAllocationRefusal>,#[value(required)]forwarded_post:Option<NativeAllocationRefusal>,#[value(required)]scoped_post:Option<NativeAllocationRefusal>,#[value(required)]forwarded_cancel:Option<NativeAllocationRefusal>,#[value(required)]scoped_cancel:Option<NativeAllocationRefusal>,#[value(required)]receipt:Option<ValueError>,#[value(required)]extent:Option<ValueError>,progress:NativeAllocationProgress}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct NativeAllocationRequest{bytes:usize,owned_bytes:usize,next_owned_bytes:usize,maximum_bytes:usize}}
semio_framework_value_derive::value_codec!{#[derive(NativeReceive)]#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]pub struct RetainedCloneProgress{copied_items:usize,copied_bytes:usize,retained_capacity_bytes:usize,released_bytes:usize}}

pub mod native_decoding {
    pub use semio_framework_value_native_decoding::native_decoding::*;
    pub use super::owned;
    pub use super::owned::{NativeReceive, NativeReceiver, NativeSharedSlots};
}

pub use semio_framework_value_derive::{NativeReceive, RetireOwned};
#[cfg(test)]
pub use semio_framework_value_derive::ToValue;
#[cfg(test)]
pub use semio_framework_value_intrinsic::{DecodedValue, FromValue, NativeEncodeControl, ToValue, ValueEdit, ValueShape};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[cfg(test)]pub(crate) use semio_framework_value_testing_admission as source_fixture;
