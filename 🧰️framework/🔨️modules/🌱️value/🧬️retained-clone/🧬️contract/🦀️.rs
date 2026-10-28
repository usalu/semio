//! 🧬️ Original retained-clone traits and standard concrete cursor/field owners.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_clone_source::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneBorrowAuthority,RetainedCloneSource,RetainedCloneSourceTake,RetainedOwnedProjection};
use crate::{ErasedSnapshotRetirement,retirement::RetireOwned};
use std::mem::{size_of,ManuallyDrop};
#[path="🔗️alias/🦀️.rs"]mod alias;
pub use alias::{RetainedAlias,RetainedAliasCursor};
#[path="../🌱️dynamic/🦀️.rs"]
pub mod dynamic;
#[path="../📋️paged-list/🦀️.rs"]
pub mod paged_list;
#[path="../🧺️std-collections/🦀️.rs"]
mod std_collections;
pub use std_collections::{RetainedCloneEntries,RetainedCloneEntriesCursor};
#[path="../🔗️issuance-clone/🦀️.rs"]
pub mod issuance_clone;
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
/// ⏳️ Bounds logical rows even when each physical element occupies zero bytes.
pub const RETAINED_CLONE_BULK_PAGE_ITEMS: usize = 1024;

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


/// 🎟️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn retained_clone_close_demands<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<crate::RetirementDemand,crate::ValueError> {
    cursor.close_demands(maximum_body_bytes)
}

/// 🪆️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_grant(grant:RetainedCloneGrant)->Result<RetainedCloneGrant,crate::ValueError> {
    Ok(RetainedCloneGrant {maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"nested clone requires enclosing depth"))?,..grant})
}

/// 🌳️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,source:RetainedCloneRef<'_,T>,maximum_copy_bytes:usize)->Result<crate::RetirementDemand,crate::ValueError> {
    let mut demand=child.advance_demands(source,maximum_copy_bytes)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"nested clone depth overflow"))?;
    Ok(demand)
}

/// 🎬️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_close_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<crate::RetirementDemand,crate::ValueError> {
    let mut demand=child.close_demands(maximum_body_bytes)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"nested clone close depth overflow"))?;
    Ok(demand)
}
