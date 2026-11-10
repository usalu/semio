//! ⚠️ Original diagnostic construction and retirement laws share a dedicated allocator.
use semio_framework_replication_retained::retained::{retained_mutation_demand_fits,diagnostic::*};
use semio_framework_replication_mutation_messages::MutationMessageLedgerRetirement;
use semio_framework_replication_mutation::MutationMessage;
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
use semio_framework_diagnostic::Severity;
use std::mem::size_of;
pub use semio_framework_replication_retained::retained::{Done,Reply};
#[path="../../🪡️field/🧪️tests/harness.rs"]
mod harness;
use harness::{Harness,Recorded,observe};
use semio_framework_value::retained_clone::RetainedCloneSource;
#[path="🦀️.rs"]
mod tests;
#[path="🔢️unsigned/🦀️.rs"]
mod unsigned_targets_tests;
