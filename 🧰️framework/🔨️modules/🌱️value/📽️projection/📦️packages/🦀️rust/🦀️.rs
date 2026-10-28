//! 📽️ Original snapshot projection contracts and owned-field cursors share one neutral owner.
extern crate self as semio_framework_value_snapshot_projection;
pub use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
pub use semio_framework_value_retirement::{retirement, RetirementDemand};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_shared_issuance as shared_issuance;
#[path = "../../🦀️.rs"]
pub mod snapshot_projection;
#[doc(hidden)]
pub mod __private {
    pub use super::snapshot_projection;
}
pub use snapshot_projection::{ImmutableRootProjection, MaybeUninit, ProjectionOutput, SnapshotProjection, SnapshotProjectionCursor};
#[cfg(test)]
pub use semio_framework_value_derive::{RetireOwned, SnapshotProjection};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
