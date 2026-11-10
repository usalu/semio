//! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, nested_retained_clone_close_demands, nested_retained_clone_take_demands, nested_retained_clone_demands, nested_retained_clone_grant, retained_clone_close_demands, retained_clone_grant_admits, settle_child_close, spent, take_child_output};
use crate::{retirement::RetireOwned,ValueError};
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::{cmp::Ordering, mem::size_of};

#[path = "🗂️map-core/🦀️.rs"]
mod map_core;
pub use map_core::*;

#[path = "🔁️clone-cursor/🦀️.rs"]
mod clone_cursor;
pub use clone_cursor::*;

pub use semio_framework_value_native_bounded_ordering::{BoundedOrdGrant,BoundedOrdProgress,BoundedOrdStep,BoundedOrd,BoundedOrdCursor,NativeBoundedOrdering,NativeBoundedOrdCursor,ScalarBoundedOrdCursor,StringBoundedOrdCursor};
#[cfg(test)]
#[path="🔢️bounded-ord/🧪️tests/driver.rs"]
pub(crate) mod bounded_test_support;

#[path = "🔎️lookup-cursor/🦀️.rs"]
mod lookup_cursor;
pub use lookup_cursor::*;

#[path="🚦️native/🦀️.rs"]
mod native;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
