//! 🔐️ Native shared input slots and original field ownership.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure};
pub use semio_framework_value_shared_issuance as shared_issuance;
#[path="../../🦀️.rs"]
mod slots;
pub use slots::NativeSharedSlots;
#[path="../../../🪡️field/🦀️.rs"]
mod field;
pub use field::NativeIntrinsicSharedField;
