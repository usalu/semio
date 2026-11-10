//! 🧬️ Original retained-clone traits and standard concrete cursor/field owners.
pub use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementCloseLoan};
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_clone_source::{RetainedCloneClose,RetainedEmptyOwner,RetainedCloseSelection,close_retained_binding,RetainedCloneRef,RetainedCloneBinding,RetainedCloneBorrowAuthority,RetainedCloneSourceAdmission,StagedSourceAdmission,StagedProjectionSelection,OriginalSourceOwner,OriginalSourceMode,RetainedCloneSource,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedNativeFieldProjection,RetainedNativeArrayIndexProjection,OriginalProjectionContext,OriginalProjectionFrame,OriginalProjectionFramePhase,OriginalProjectionProducer,OriginalProjectionReply,OriginalSourceProjection};
use crate::{ErasedSnapshotRetirement,retirement,retirement::RetireOwned};
use std::mem::{size_of,ManuallyDrop};
#[path="🔗️alias/🦀️.rs"]mod alias;
pub use alias::{RetainedAlias,RetainedAliasCursor};
#[path="../🌱️dynamic/🦀️.rs"]
pub mod dynamic;
#[path="../📋️paged-list/🦀️.rs"]
pub mod paged_list;
#[path="../🧺️collection/🦀️.rs"]
mod native_collections;
pub use native_collections::{NativeCollectionClone,NativeCollectionCursor,NativeSortedKey,NativeSortedMap,NativeSortedSet,NativeMapAdmission,NativeSetAdmission};
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
#[path="🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
#[cfg(test)]
#[path="🧪️tests/♻️close/🦀️.rs"]
mod original_close_frame_tests;


/// 🎟️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn retained_clone_close_demands<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError> {
    cursor.close_demands(maximum_body_bytes)
}

/// 🪆️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_grant(grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError> {
    Ok(RetainedCloneGrant {maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone requires enclosing depth"))?,..grant})
}

/// 🌳️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,source:RetainedCloneRef<'_,T>,maximum_copy_bytes:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError> {
    let mut demand=child.advance_demands(source,maximum_copy_bytes,maximum_work_units)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone depth overflow"))?;
    Ok(demand)
}

/// 🎬️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_close_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError> {
    let mut demand=if child.close_started(){child.close_demands(maximum_body_bytes)?}else{child.begin_close_demand()?};
    demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone close depth overflow"))?;
    Ok(demand)
}

/// 📤️ Quotes the actual child output move under one enclosing cursor frame.
pub fn nested_retained_clone_take_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>)->Result<RetirementDemand,ValueError>{let mut demand=child.take_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone take depth overflow"))?;Ok(demand)}

use semio_framework_value_native_clone_engine::allocate_clone_storage;

/// 🧷️ An advance outcome cannot carry an unresolved producer reply, so a child scaffold close that left its reply pending refuses typed.
/// 🧮️ Narrows the u128 work already spent in a step to the u64 limit arithmetic of its original quote.
pub fn spent(work:u128)->u64{u64::try_from(work).unwrap_or(u64::MAX)}
pub fn settle_child_close(poll:crate::OriginalRetirementPoll)->crate::OriginalRetirementOutcome{match poll{crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original child close left its producer reply pending inside advance").into()),work}}}
/// 📤️ Moves a child's ready output into its parent slot under one nested effect; the child's own take records its work in that effect.
pub fn take_child_output<T:RetainedClone>(child:&mut impl RetainedCloneCursor<T>,slot:&mut Option<T>,grant:RetainedCloneGrant,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{
 let mut reply_slot=None;{let reply=crate::OriginalEffectReply::borrow(&mut reply_slot)?;child.take(slot,&mut effect.nested(grant),reply)?;}
 let step=reply_slot.take().ok_or_else(||ValueError::invariant("child take left no reply"))??;
 Ok(if slot.is_some(){Some(step.progress())}else{None})
}
fn child_close_start_demand<T:RetainedClone>(child:&impl RetainedCloneCursor<T>)->Result<RetirementDemand,ValueError>{if child.close_started(){Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}else{child.begin_close_demand()}}
fn child_close_start_work<T:RetainedClone>(child:&impl RetainedCloneCursor<T>)->Result<u64,ValueError>{if child.close_started(){Ok(1)}else{child.begin_close_work_demand()}}

pub use semio_framework_value_clone_selection::{ArrayPrefixShape,ArrayPrefixSelection,select_array_prefix};

#[cfg(test)]
pub(crate) mod selection_observation{
 use std::cell::Cell;
 thread_local!{static ACTIVE:Cell<bool>=const{Cell::new(false)};static VISITS:Cell<usize>=const{Cell::new(0)};}
 pub(crate) fn begin(){VISITS.with(|visits|visits.set(0));ACTIVE.with(|active|active.set(true));}
 pub(crate) fn record(){ACTIVE.with(|active|{if active.get(){VISITS.with(|visits|visits.set(visits.get()+1))}});}
 pub(crate) fn finish()->usize{ACTIVE.with(|active|active.set(false));VISITS.with(Cell::get)}
}
