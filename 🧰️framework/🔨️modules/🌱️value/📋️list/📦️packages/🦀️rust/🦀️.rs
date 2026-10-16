//! 📋️ Original bounded paged metadata, reservation and edit ownership.
pub use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};
pub use semio_framework_value_control_contract as retained_clone;
#[path="../../🦀️.rs"]
pub mod list;
pub use list::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[path="../../🔣️serde/🦀️.rs"]
mod serde_owners;
