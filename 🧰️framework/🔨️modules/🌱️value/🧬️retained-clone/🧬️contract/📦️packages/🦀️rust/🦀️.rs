//! 🧬️ Concrete standard clone cursors own exact payload copies and original alias custody.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue,Number,dsl_value};
pub use semio_framework_value_retirement::{retirement,RetirementSelection,select_factory_ticket,ErasedSnapshotRetirement,factory_ticket_demands,close_factory_ticket,owned_retirement_birth_bytes,admit_owned_retirement};
pub use semio_framework_value_shared_issuance as shared_issuance;
#[path="../../🦀️.rs"]
pub mod retained_clone;
pub use retained_clone::*;
#[cfg(test)]
pub use semio_framework_value_derive::{RetainedClone,RetireOwned};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
