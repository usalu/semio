//! 📋️ The retained paged clone laws.
pub use semio_framework_value::retained_clone::paged::*;
use crate::list::PagedList;
use crate::paged::{PagedBytes, PagedMap, PagedUtf8};
use crate::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneStep};
use crate::retirement::RetireOwned;
use std::mem::size_of;

#[path = "../../../../🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
