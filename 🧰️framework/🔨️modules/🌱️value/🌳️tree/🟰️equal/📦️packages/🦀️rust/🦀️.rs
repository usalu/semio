//! 🟰️ Structural equality over retained original values and scalar slices.
use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};
pub use semio_framework_value_control_contract::{RetainedAllocationEffect,OriginalRetirementFailure,RetainedWorkOutcome,RetainedWorkPoll,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub use semio_framework_value_control_contract as retained_clone;

use semio_framework_value_retirement::{retirement,RetirementDemand};
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll};
#[path = "../../../../📋️list/🟰️equal/🦀️.rs"]
pub mod slice_equality;
pub use slice_equality::{RetainedEqualityElement,RetainedSliceEquality};
#[path = "../../🦀️.rs"]
pub mod retained_equality;
pub use retained_equality::{RetainedEquality,RetainedEqualityCursor,RetainedEqualityFields,RetainedEqualityOwner,RetainedEqualityStep,RetainedRecordEquality};
#[cfg(test)]
#[path="../../🧪️tests/effect.rs"]
pub(crate) mod eff;
#[cfg(test)]
pub use semio_framework_value_retirement::{retirement_fields_type,retirement_fields_seed,OriginalRetirementControl};
#[cfg(test)]
pub use semio_framework_value_control_contract::OriginalRetirementCloseLoan;
#[cfg(test)]
use semio_framework_value_derive::{RetainedEquality,RetireOwned,ToValue};
#[cfg(test)]
use semio_framework_value_intrinsic::{DslValue,FromValue,ToValue,ValueShape};
#[cfg(test)]
use semio_framework_value_native_encoding::NativeEncodeControl;
#[cfg(test)]
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

#[path="../../🌱️intrinsic/🦀️.rs"]
mod original_value_equality;
pub use original_value_equality::{OriginalValueEquality,ValueEqualityStep};
