//! ✏️ Native insertion and removal retain the original lookup, shifted pages and edited row until paid handoff.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, nested_retained_clone_close_demands, nested_retained_clone_take_demands, nested_retained_clone_demands, nested_retained_clone_grant, retained_clone_close_demands, retained_clone_grant_admits, settle_child_close, spent, take_child_output};
use crate::{retirement::RetireOwned,ValueError};
use semio_framework_value_retained_map::ordered_map::{RETAINED_ORDERED_MAP_PAGE_CAPACITY,RetainedOrderedMap,RetainedOrderedMapLookup,RetainedOrderedMapLookupCursor};
use semio_framework_value_native_bounded_ordering::{BoundedOrdGrant,BoundedOrdProgress,BoundedOrdStep,BoundedOrd,BoundedOrdCursor,NativeBoundedOrdering,NativeBoundedOrdCursor,ScalarBoundedOrdCursor,StringBoundedOrdCursor};
use std::{cmp::Ordering, mem::size_of};

#[path="➕️insert-cursor/🦀️.rs"]
mod insert_cursor;
pub use insert_cursor::*;

#[path="➖️remove/🦀️.rs"]
mod removal;
pub use removal::{RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RetainedOrderedMapRemoveProgress,RetainedOrderedMapRemoveStep};

#[path="🎟️lease/🦀️.rs"]
mod staged_lease;

#[path="📥️workspace/🦀️.rs"]
mod edit_input;

#[path="♻️retirement/🦀️.rs"]
mod edit_retirement;

#[cfg(test)]
#[path="../🔢️bounded-ord/🧪️tests/driver.rs"]
pub(crate) mod bounded_test_support;

#[cfg(test)]
#[path="../🧪️tests/bank-support.rs"]
pub(crate) mod bank_support;

#[cfg(test)]
#[path="🧪️tests/edit-support.rs"]
pub(crate) mod edit_test_support;

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="➖️remove/🧪️tests/🦀️.rs"]
mod removal_tests;

#[cfg(test)]
#[path="🧪️tests/edit-native.rs"]
mod native_edit_tests;
