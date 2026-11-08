//! 🗺️ Fixed-page retained maps own resumable comparisons, insertion, removal and retirement.
#[cfg(not(test))]
pub use semio_framework_value_retained_clone::{retained_clone};
#[cfg(test)]
#[path="../../../🧬️contract/🧪️tests/⏱️effect/🦀️.rs"]
mod effect_fixture;
#[cfg(test)]
pub mod retained_clone{pub use semio_framework_value_retained_clone::retained_clone::*;pub(crate) use super::effect_fixture as effect;}
pub use semio_framework_value_retained_clone::retained_clone::*;
pub use semio_framework_value_retirement::{retirement,RetireOwned,ErasedSnapshotRetirement,owned_retirement_birth_bytes,OriginalRetirementControl,OriginalRetirementOutcome,OriginalRetirementPoll,RetirementPollExit,retirement_fields_type,retirement_fields_seed};
pub use semio_framework_value_derive::RetireOwned;
pub use semio_framework_value_intrinsic::{DslValue,Number,DecodedValue,FromValue,ToValue,ValueShape,ValueEdit,NativeDecodeControl,NativeEncodeControl};
pub use serde;
#[path="../../🦀️.rs"]
pub mod ordered_map;
pub use ordered_map::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) use semio_framework_value_clone_source_testing::FixtureSourceBirth;
