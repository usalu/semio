//! 🧱️ Original job ownership is independent of application requests and their nineteen field retirement.
use semio_framework_job::{StepContext,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView,InteractiveJobCloseStep};
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,arc_custody::SharedControlledRetirement}};
use std::{mem::{ManuallyDrop,size_of,size_of_val},sync::Arc};
#[path="📦️producer/🦀️.rs"]
mod producer;
pub use producer::{ArtifactReservedJob,ArtifactReservedToolJob,ArtifactReservedToolJobState};
#[path="🔗️lease/🦀️.rs"]
mod lease;
pub use lease::OriginalRequestLease;
include!("../🎟️birth/🦀️.rs");
#[cfg(any(test,feature="reserved-producer-testing"))]
pub mod original_reserved_producer_laws{include!("../🎟️birth/🧪️tests/🦀️.rs");}
#[cfg(test)]
#[path="../🎟️request/♻️retirement/🧪️tests/🦀️.rs"]
mod original_request_lease_laws;
