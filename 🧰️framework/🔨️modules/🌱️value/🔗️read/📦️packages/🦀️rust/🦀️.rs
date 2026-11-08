//! 🔗️ Genuine typed read ownership composes only lower neutral control, clone, Factory and retirement owners.
pub use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedWorkPoll,OriginalRetirementFailure,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement,OriginalRetirementPoll};
pub use semio_framework_value_factory::{FactoryAuthority,FactoryRetirement,FactoryPayloadRetirement};
pub use semio_framework_value_retained_clone::retained_clone;
#[path="../../🦀️.rs"]
mod definition;
pub use definition::{IssuedReadRegistry,ReadAuthority,ReadLease,ReadLeaseId,ReadLeaseIssue,ReadVisit,ReadOwnershipRegistry,ReadRegistryBirth,ErasedReadLease,OriginalReadSource,OriginalErasedReadSource,ReadSourceCursor,READ_LEASE_CAPACITY};
