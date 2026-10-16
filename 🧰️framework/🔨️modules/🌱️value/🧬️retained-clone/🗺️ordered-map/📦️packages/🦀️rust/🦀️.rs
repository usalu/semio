//! 🗺️ Fixed-page retained maps own resumable comparisons, insertion, removal and retirement.
pub use semio_framework_value_retained_clone::{retained_clone};
pub use semio_framework_value_retained_clone::retained_clone::*;
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_retirement::{retirement,RetireOwned,ErasedSnapshotRetirement,owned_retirement_birth_bytes};
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
