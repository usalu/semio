//! 🎟️ Original directional native admission and retained continuation ownership.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement,factory_ticket_demands,close_factory_ticket};
#[path="../../../🫴️receiving/🦀️.rs"]
pub mod native_receiving;
#[path="../../🦀️.rs"]
pub mod native_encoding;
pub use native_encoding::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
