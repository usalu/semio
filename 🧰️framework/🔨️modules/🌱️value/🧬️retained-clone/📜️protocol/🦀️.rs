//! 📜️ Clone and cursor traits of the retained clone contract: the retained clone trait with its bitwise hooks, the bulk page bytes, the bulk run helpers, the clone cursor trait, the close record and the binding close helper. See [`super`].

pub use semio_framework_value_control_contract::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone_grant_admits};
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,retirement::RetireOwned};
pub use semio_framework_value_clone_source::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneBorrowAuthority,RetainedCloneBorrowAuthorityAdmission,RetainedCloneSourceAdmission,StagedSourceAdmission,RetainedCloneSource,RetainedCloneSourceTake,RetainedOwnedProjection};
/// ⏳️ Bounds logical rows even when each physical element occupies zero bytes.
pub const RETAINED_CLONE_BULK_PAGE_ITEMS:usize=1024;

pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    /// 🏗️ Constructs this cursor without allocation, source access, traversal or panic within its declared birth work.
    fn retained_clone_cursor() -> Self::Cursor;
    /// ⛽️ Quotes the fixed source-free construction work of this actual cursor.
    fn cursor_birth_work()->Result<u64,ValueError>;
    /// 🧬️ True for plain scalar values whose arrays clone in one funded bitwise copy turn.
    const BITWISE: bool = false;
    /// 📋️ Copies a whole array bitwise when [`Self::BITWISE`] is set; every other element clones one by one.
    fn bitwise_array<const N: usize>(_source: &[Self; N]) -> Option<[Self; N]> { None }
    /// 🚚️ A bitwise implementation appends exactly the supplied run without allocation, traversal or panic and within the quoted per-byte work; a false result leaves the target unchanged.
    fn bitwise_extend(_target: &mut Vec<Self>, _source: &[Self]) -> bool { false }
}

pub use semio_framework_value_control_contract::RETAINED_CLONE_BULK_PAGE_BYTES;

/// 📏️ The number of `width`-byte elements one bulk turn may process under `grant`: one item and at most one page of payload, zero when the grant admits neither one item nor one element.
pub fn bulk_run_elements(grant: RetainedCloneGrant, width: usize) -> usize {
    if grant.maximum_items == 0 { return 0; }
    if width == 0 { return RETAINED_CLONE_BULK_PAGE_ITEMS; }
    grant.maximum_copy_bytes.min(RETAINED_CLONE_BULK_PAGE_BYTES) / width
}

/// 🧾️ The honest receipt of one bulk turn that processed `elements` elements of `width` bytes: one item and exactly those payload bytes.
pub fn bulk_run_progress(elements: usize, width: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: elements.saturating_mul(width), ..Default::default() }
}

/// 🧷️ Transfers one native owner once; after `take`, callers must close the spent cursor before dropping it.
/// 🎟️ Borrows the actual selected child closure quote without advancing its typed owner.
pub fn clone_cursor_close_demand<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,body:usize)->Result<RetirementDemand,ValueError>{if cursor.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}cursor.close_demands(body)}

pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    fn output_ready(&self)->bool{false}
        /// 🚦️ Explicit cursor owners override their close lifecycle; absent closure intent is unstarted.
    fn close_started(&self)->bool;
    /// 🎟️ Quotes one close action from the actual paid currencies.
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    /// ⏩️ Quotes the next immutable-source turn under the same original work limit used by advance.
    fn advance_demands(&self, _maximum_body_bytes: usize, _maximum_work_units:u64) -> Result<RetirementDemand, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "clone cursor lacks an original-source advance quote"))
    }
    /// ⛽️ Quotes actual selected work without exceeding the supplied original limit.
    fn advance_work_demand(&self,_body:usize,_maximum_work_units:u64)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original work quote"))}
    fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome;
    /// 📤️ Quotes the original ready-owner transfer work before handoff.
    fn take_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original take work quote"))}
    /// ♻️ Quotes the selected child retirement work before closure.
    fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close work quote"))}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>;
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>;
    /// ⛽️ Quotes the selected original close-start action before effects.
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close-start work quote"))}
    /// 📏️ Quotes the persistent writes of the selected close-start action.
    fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
    /// 🚦️ Performs the admitted close-start action and reports its actual work and physical writes.
    fn begin_close_paid(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>;
    
    fn terminal_is_empty(&self) -> bool;
    /// 🪜️ Observes the depth of the next controlled close transition without changing ownership.
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    /// 🧮️ Observes the next payload-work grant without allocating or releasing retained owners.
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close work demand"))
    }
    /// 📐️ Observes the next granted close birth; unsupported cursors retain their owners.
    fn next_close_capacity_byte_demand(&self, _maximum_release_bytes: usize) -> Result<usize, ValueError> {
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close birth demand"))
    }
    /// 📪️ Observes the next body or scaffold release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close release demand"))
    }
    /// 🎯️ Executes the same body-selected close only after every granted axis admits its fixed quote.
    fn close_selected(&mut self,body:usize,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome{
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        let selected=(||{let demand=self.close_demands(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(None)}if self.close_demands(demand.copy_bytes)?!=demand{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected retirement quote changes under its exact admitted copy frontier"))}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}))})();
        match selected{Ok(Some(grant))=>self.close_step(grant,actual),Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}}
    }
    fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome{
        RetainedWorkOutcome{result:if grant.maximum_items==0{Ok(RetainedCloneStep::Progress(Default::default()))}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no capacity-bearing close authority").into())},work:0}
    }
}

pub use semio_framework_value_clone_source::{RetainedCloneClose,RetainedEmptyOwner,RetainedCloseSelection,close_retained_binding};


/// 📤️ Quotes one ownership transition without creating another retained payload.
pub fn retained_clone_take_demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<T>>(),depth:1,..Default::default()}}
/// 📏️ Quotes a child's close start and its parent's persistent close-start flag.
pub fn retained_clone_child_close_start_demand<T:RetainedClone>(child:&impl RetainedCloneCursor<T>)->Result<RetirementDemand,ValueError>{let mut demand=child.begin_close_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::invariant("child close-start copy overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"child close-start depth overflow"))?;Ok(demand)}
/// 🧳️ Moves the actual ready output slot once and reports its defining work.
pub fn take_retained_clone_output<T>(output:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{
 if output.is_none()||!retained_clone_grant_admits(grant,retained_clone_take_demand::<T>()){return RetainedWorkOutcome{result:Ok(None),work:0}}
 let value=match output.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("output absent").into()),work:0}};RetainedWorkOutcome{result:Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<T>>(),..Default::default()}))),work:1}
}

use std::mem::{size_of,ManuallyDrop};
#[path="../🌱️dynamic/🦀️.rs"]pub mod dynamic;
#[path="../📋️paged-list/🦀️.rs"]pub mod paged_list;
#[path="../🧺️std-collections/🦀️.rs"]mod std_collections;
pub use std_collections::{RetainedCloneEntries,RetainedCloneEntriesCursor};
#[path="../🔗️issuance-clone/🦀️.rs"]pub mod issuance_clone;
#[path="../📋️field/🦀️.rs"]mod field;
pub use field::RetainedFieldCursor;
#[path="../🔮️demands/🦀️.rs"]mod normal_demands;
pub use normal_demands::RetainedCloneDemandCursor;
#[path="../🧬️contract/🔢️scalar-cursor/🦀️.rs"]mod scalar_cursor;
pub use scalar_cursor::*;
#[path="../🧬️contract/🧮️array-cursor/🦀️.rs"]mod array_cursor;
pub use array_cursor::*;
#[path="../🧬️contract/🔤️string-cursor/🦀️.rs"]mod string_cursor;
pub use string_cursor::*;
#[path="../🧬️contract/📚️vec-cursor/🦀️.rs"]mod vec_cursor;
pub use vec_cursor::*;
#[path="../🧬️contract/🎁️option-cursor/🦀️.rs"]mod option_cursor;
pub use option_cursor::*;
#[path="../🧬️contract/🗃️box-cursor/🦀️.rs"]mod box_cursor;
pub use box_cursor::*;
#[path="../🧬️contract/🧱️tuple-cursors/🦀️.rs"]mod tuple_cursors;
pub use tuple_cursors::*;
#[cfg(test)]#[path="../🧮️scalar/🧪️tests/🦀️.rs"]mod scalar_authority_tests;
#[cfg(test)]#[path="../🚚️bulk-vec/🧪️tests/🦀️.rs"]mod bulk_vec_tests;
#[cfg(test)]pub(crate) use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};
#[cfg(test)]#[path="../🧬️contract/🧪️tests/♻️close/🦀️.rs"]mod original_close_frame_tests;
/// 🎟️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn retained_clone_close_demands<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError> {
    cursor.close_demands(maximum_body_bytes)
}

/// 🪆️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_grant(grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError> {
    Ok(RetainedCloneGrant {maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone requires enclosing depth"))?,..grant})
}

/// 🌳️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,maximum_copy_bytes:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError> {
    let mut demand=child.advance_demands(maximum_copy_bytes,maximum_work_units)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone depth overflow"))?;
    Ok(demand)
}

/// 🎬️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn nested_retained_clone_close_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError> {
    let mut demand=child.close_demands(maximum_body_bytes)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone close depth overflow"))?;
    Ok(demand)
}

/// 📤️ Quotes the actual child output move under one enclosing cursor frame.
pub fn nested_retained_clone_take_demands<T:RetainedClone>(child:&impl RetainedCloneCursor<T>)->Result<RetirementDemand,ValueError>{let mut demand=child.take_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"nested clone take depth overflow"))?;Ok(demand)}
