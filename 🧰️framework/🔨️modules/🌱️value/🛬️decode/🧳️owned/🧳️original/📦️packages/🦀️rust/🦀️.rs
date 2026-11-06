//! 🧳️ Original retained source and native receiver lifecycle.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding::NativeDecodeControl,native_decoding::OriginalReceivingPrimary};
pub use semio_framework_value_native_owned::{owned,native_decoding,NativeReceive,NativeReceiver,DslValue,RetireOwned};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome};
pub use semio_framework_value_shared_issuance as shared_issuance;
#[cfg(test)]use semio_framework_value_testing_admission as source_fixture;
#[path="../../🦀️.rs"]
mod original;
pub use original::NativeReceivingOriginal;
#[doc(hidden)]
pub use original::NativeReceivingOriginalSeed;
