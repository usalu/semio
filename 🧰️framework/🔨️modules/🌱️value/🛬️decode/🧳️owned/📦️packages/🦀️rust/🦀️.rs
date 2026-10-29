//! 🧳️ Native receiving of retained owners — wiring only. Domain lives at the owner `🦀️.rs` files.

pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue, Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeOwnedDecodeControl};
pub use semio_framework_value_persistent::list;
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../🦀️.rs"]
pub mod owned;
pub use owned::{blocking, NativeReceive, NativeReceiver};

/// 🎟️ The native decoding vocabulary composed with its receivers: the decoding core and the owned receiving tree under one module.
pub mod native_decoding {
    pub use semio_framework_value_native_decoding::native_decoding::*;
    pub use super::owned;
    pub use super::owned::{NativeReceive, NativeReceiver};
}

#[cfg(test)]
pub use semio_framework_value_derive::{NativeReceive, RetireOwned, ToValue};
#[cfg(test)]
pub use semio_framework_value_intrinsic::{DecodedValue, FromValue, NativeEncodeControl, ToValue, ValueEdit, ValueShape};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
