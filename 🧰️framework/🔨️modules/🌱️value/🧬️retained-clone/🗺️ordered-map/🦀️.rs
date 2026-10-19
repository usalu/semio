//! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, nested_retained_clone_close_demands, nested_retained_clone_demands, nested_retained_clone_grant, retained_clone_close_demands, retained_clone_grant_admits};
use crate::{retirement::RetireOwned};
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::{cmp::Ordering, mem::size_of};

#[path = "🗂️map-core/🦀️.rs"]
mod map_core;
pub use map_core::*;

#[path = "🔁️clone-cursor/🦀️.rs"]
mod clone_cursor;
pub use clone_cursor::*;

#[path = "🔢️bounded-ord/🦀️.rs"]
mod bounded_ord;
pub use bounded_ord::*;

#[path = "🔎️lookup-cursor/🦀️.rs"]
mod lookup_cursor;
pub use lookup_cursor::*;

#[path = "➕️insert-cursor/🦀️.rs"]
mod insert_cursor;
pub use insert_cursor::*;

#[path="🚦️native/🦀️.rs"]
mod native;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="➖️remove/🧪️tests/🦀️.rs"]
mod removal_tests;

#[path="➖️remove/🦀️.rs"]
mod removal;
pub use removal::{RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RetainedOrderedMapRemoveProgress,RetainedOrderedMapRemoveStep};

#[path="♻️retirement/🦀️.rs"]
mod edit_retirement;
