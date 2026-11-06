//! 📥️ Canonical phased native defaults own their implementation and standard type instances.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
#[path="../../🦀️.rs"]mod defaults;
pub use defaults::*;

#[cfg(test)]pub use semio_framework_value_retirement::retirement::RetireOwned;
#[cfg(test)]pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

pub use semio_framework_value_native_decoding::native_decoding;
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementFailure};
#[cfg(test)]pub use semio_framework_value_testing_admission as source_fixture;
