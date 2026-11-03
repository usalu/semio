//! 🔗️ Persistent paged lists — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
pub use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value_derive::{RetainedEquality, RetireOwned, ToValue};
use semio_framework_value_intrinsic::{DslValue, FromValue, ToValue, ValueShape};
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_snapshot_projection::snapshot_projection;
use semio_framework_value_retirement::{retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../../🌳️roots/🦀️.rs"]
pub mod list;
#[path = "../../../🪢️concat/🦀️.rs"]
pub mod original_vector_concat;
pub use original_vector_concat::OriginalVectorConcat;
pub use semio_framework_value_equality::{slice_equality,retained_equality,RetainedEqualityElement,RetainedSliceEquality,RetainedEquality,RetainedEqualityCursor,RetainedEqualityFields,RetainedEqualityOwner,RetainedEqualityStep,RetainedRecordEquality};

#[path = "../../🔁️codec/🦀️.rs"]
mod value_codec;

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) mod value {
    pub(crate) use super::observe_retirement_allocations;
}

#[path = "../../🧬️clone/🦀️.rs"]
pub mod immutable_clone;

pub use semio_framework_value_retained_work::work;
