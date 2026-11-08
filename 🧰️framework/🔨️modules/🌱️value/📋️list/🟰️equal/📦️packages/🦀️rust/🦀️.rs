//! 🟰️ Actual bounded scalar equality owner.
pub use semio_framework_value_control_contract::{RetirementDemand,ValueError,ValueRefusalKind,RetainedWorkOutcome,RetainedWorkPoll,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,OriginalEffectReply};
#[path="../../🦀️.rs"] mod slice;
pub use slice::{RetainedEqualityElement,RetainedSliceEquality,RetainedSliceInspection,RetainedStagedSliceEquality};
#[cfg(test)] pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

#[path="../../⚙️sequence/🦀️.rs"]mod native_sequence_engine;
