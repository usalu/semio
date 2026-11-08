//! 🪡️ Canonical native shared field owns the original source, child and output.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure,OriginalRetirementCloseLoan};
pub use semio_framework_value_shared_issuance as shared_issuance;
pub use semio_framework_value_native_shared_slots::NativeSharedSlots;
#[path="../../🧳️owned/🪡️field/🦀️.rs"]
mod field;
pub use field::NativeIntrinsicSharedField;
