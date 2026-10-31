//! 🔗️ Genuine typed read ownership composes only lower neutral control, clone, Factory and retirement owners.
pub use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement};
pub use semio_framework_value_factory::{FactoryAuthority,FactoryRetirement,FactoryPayloadRetirement};
pub use semio_framework_value_retained_clone::retained_clone;
#[path="../../🦀️.rs"]
mod definition;
pub use definition::{ReadAuthority,ReadLease,ReadLeaseId,ReadOwnershipRegistry,ErasedReadLease,OriginalReadSource,OriginalErasedReadSource,READ_LEASE_CAPACITY};
