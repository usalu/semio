//! 🎟️ Original directional native admission and retained continuation ownership.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement,factory_ticket_demands,close_factory_ticket};
pub use semio_framework_value_native_encoding::{native_encoding,NativeEncodeControl,NativeEncodeContinuation};
#[path="../../🦀️.rs"]
pub mod native_decoding;
pub use native_decoding::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
#[path="../../../🫴️receiving/🧪️tests/🦀️.rs"]
mod original_receiving_tests;
