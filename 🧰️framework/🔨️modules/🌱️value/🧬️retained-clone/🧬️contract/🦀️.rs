//! 🧬️ Original retained-clone traits and standard concrete cursor/field owners.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_clone_source::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneBorrowAuthority,RetainedCloneSource,RetainedCloneSourceTake,RetainedOwnedProjection};
use crate::{ErasedSnapshotRetirement,retirement::RetireOwned};
use std::mem::{size_of,ManuallyDrop};
#[path="../🌱️dynamic/🦀️.rs"]
pub mod dynamic;
#[path="../📋️paged-list/🦀️.rs"]
pub mod paged_list;
#[path="../🧺️std-collections/🦀️.rs"]
mod std_collections;
pub use std_collections::{RetainedCloneEntries,RetainedCloneEntriesCursor};
#[path="../🔗️shared/🦀️.rs"]
pub mod shared;
#[path="../📋️field/🦀️.rs"]
mod field;
pub use field::RetainedFieldCursor;
#[path="../🔮️demands/🦀️.rs"]
mod normal_demands;
pub use normal_demands::RetainedCloneDemandCursor;
#[path="../🧳️owner/🦀️.rs"]
pub mod owner;
pub use owner::RetainedCloneOwner;

#[path = "📜️clone-and-cursor-traits/🦀️.rs"]
mod clone_and_cursor_traits;
pub use clone_and_cursor_traits::*;

#[path = "🔢️scalar-cursor/🦀️.rs"]
mod scalar_cursor;
pub use scalar_cursor::*;

#[path = "🧮️array-cursor/🦀️.rs"]
mod array_cursor;
pub use array_cursor::*;

#[path = "🔤️string-cursor/🦀️.rs"]
mod string_cursor;
pub use string_cursor::*;

#[path = "📚️vec-cursor/🦀️.rs"]
mod vec_cursor;
pub use vec_cursor::*;

#[path = "🎁️option-cursor/🦀️.rs"]
mod option_cursor;
pub use option_cursor::*;

#[path = "🗃️box-cursor/🦀️.rs"]
mod box_cursor;
pub use box_cursor::*;

#[path = "🧱️tuple-cursors/🦀️.rs"]
mod tuple_cursors;
pub use tuple_cursors::*;

#[cfg(test)]
#[path="../🧮️scalar/🧪️tests/🦀️.rs"]
mod scalar_authority_tests;
#[cfg(test)]
#[path="../🚚️bulk-vec/🧪️tests/🦀️.rs"]
mod bulk_vec_tests;
#[cfg(test)]
pub(crate) use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};

#[cfg(test)]
#[path="🧪️tests/♻️close/🦀️.rs"]
mod original_close_frame_tests;
