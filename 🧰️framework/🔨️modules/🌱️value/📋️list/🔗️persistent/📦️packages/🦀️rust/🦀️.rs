//! 🔗️ Persistent paged lists and structural retained equality — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_derive::{RetainedEquality, RetireOwned, ToValue};
use semio_framework_value_intrinsic::{DslValue, FromValue, ToValue, ValueShape};
use semio_framework_value_native_encoding::NativeEncodeControl;
use semio_framework_value_retained_clone::retained_clone;
use semio_framework_value_snapshot_projection::snapshot_projection;
#[cfg(test)]
use semio_framework_value_retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value_retirement::{retirement, RetirementDemand};
use semio_framework_value_shared_issuance as shared_issuance;

#[path = "../../../🌳️roots/🦀️.rs"]
pub mod list;
#[path = "../../../🪢️concat/🦀️.rs"]
pub mod original_vector_concat;
pub use original_vector_concat::OriginalVectorConcat;
#[path = "../../../🟰️equal/🦀️.rs"]
pub mod slice_equality;
pub use slice_equality::{RetainedEqualityElement, RetainedSliceEquality};
#[path = "../../../../🌳️tree/🟰️equal/🦀️.rs"]
pub mod retained_equality;
pub use retained_equality::{RetainedEquality, RetainedEqualityCursor, RetainedEqualityFields, RetainedEqualityOwner, RetainedEqualityStep, RetainedRecordEquality};

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
