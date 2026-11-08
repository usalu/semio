//! 🧪️ Public ranked-update and original-read laws share their actual finite test treasury.
pub use semio_framework_value_control_contract::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetainedAllocationEffect,ValueError};
use semio_framework_value_testing_allocator as allocation_testing;
pub use allocation_testing::observe_retirement_allocations;
#[path="../../../../♻️retirement/🧪️tests/⏱️effect/🦀️.rs"]mod effect;
pub use semio_framework_value_persistent::list::persistent::*;
pub use semio_framework_value_retirement::retirement;
pub use retirement::RetireOwned;
pub use semio_framework_value_retained_clone::retained_clone;
pub use retained_clone::RetainedCloneRef;
pub use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
#[path="🦀️.rs"]mod update_work_tests;
#[path="../../🧳️read/🎟️work/🧪️tests/🦀️.rs"]mod read_work_tests;

/// 🎟️ Funds the complete admission quote of one value, copy included, on top of the caller's authority.
pub(crate) fn admit_funded<T:Send+Sync+'static>(value:T,issuer:SharedIssuer<T>,mut grant:RetainedCloneGrant)->RetainedWorkOutcome<(SealedShared<T>,semio_framework_value_control_contract::RetainedCloneProgress),(ValueError,T)>{let d=SealedShared::<T>::admission_demand().unwrap();grant.maximum_copy_bytes=grant.maximum_copy_bytes.max(d.copy_bytes);SealedShared::admit(value,issuer,grant,&mut None)}
