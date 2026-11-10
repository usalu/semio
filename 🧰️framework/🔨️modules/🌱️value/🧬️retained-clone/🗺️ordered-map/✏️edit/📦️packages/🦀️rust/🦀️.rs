//! ✏️ Native ordered-map insertion and removal cursors above the fixed-page retained map.
use semio_framework_value_retained_map::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
pub(crate) use semio_framework_value_clone_source_testing::FixtureSourceBirth;
#[path="../../🦀️.rs"]
mod ordered_map;
pub use ordered_map::{RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapInsertProgress,RetainedOrderedMapInsertStep,RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RetainedOrderedMapRemoveProgress,RetainedOrderedMapRemoveStep};
