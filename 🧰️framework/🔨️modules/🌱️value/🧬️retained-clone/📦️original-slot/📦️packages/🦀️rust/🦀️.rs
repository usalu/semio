//! 📦️ Original native slots retain one payload and its genuine captured read issuer.
pub use semio_framework_value_control_contract::{RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_factory::{ArtifactOwnedValueRetirementFactory,FactoryAuthority};
pub use semio_framework_value_factory::owned::{FactoryOwnedRetirement,FactoryOwnedSourceAuthority};
pub use semio_framework_value_retained_clone::{RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSourceTake,RetainedCloneStep,retained_clone_grant_admits};
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,factory_ticket_demands,close_factory_ticket};
#[path="../../🦀️.rs"]
pub mod original_slot;
pub use original_slot::{RetainedOriginalReadOwner,RetainedOriginalRead,RetainedOriginalReadReturn,RetainedOriginalSlot,RetainedOriginalSlotKey};
#[cfg(test)]
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
