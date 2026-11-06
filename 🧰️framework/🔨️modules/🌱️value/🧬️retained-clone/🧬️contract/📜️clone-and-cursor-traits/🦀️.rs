//! 📜️ Clone and cursor traits of the retained clone contract: the retained clone trait with its bitwise hooks, the bulk page bytes, the bulk run helpers, the clone cursor trait, the close record and the binding close helper. See [`super`].

use super::*;
pub use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete};

pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor;
    /// 🧬️ True for plain scalar values whose arrays clone in one funded bitwise copy turn.
    const BITWISE: bool = false;
    /// 📋️ Copies a whole array bitwise when [`Self::BITWISE`] is set; every other element clones one by one.
    fn bitwise_array<const N: usize>(_source: &[Self; N]) -> Option<[Self; N]> { None }
    /// 🚚️ Appends a contiguous run bitwise into already reserved vector capacity when [`Self::BITWISE`] is set; returns false for every element that must clone one by one.
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
pub fn clone_cursor_close_demand<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,body:usize)->Result<RetirementDemand,ValueError>{if cursor.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}if cursor.close_started(){cursor.close_demands(body)}else{cursor.begin_close_demand()}}

pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    fn output_ready(&self)->bool{false}
        /// 🚦️ Explicit cursor owners override their close lifecycle; absent closure intent is unstarted.
    fn close_started(&self)->bool{false}
    /// 🎟️ Quotes one close action from the actual paid currencies.
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    /// ⏩️ Quotes the next immutable-source turn under the same original work limit used by advance.
    fn advance_demands(&self, _source: RetainedCloneRef<'_, T>, _maximum_body_bytes: usize, _maximum_work_units:u64) -> Result<RetirementDemand, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "clone cursor lacks an original-source advance quote"))
    }
    /// ⛽️ Quotes actual selected work without exceeding the supplied original limit.
    fn advance_work_demand(&self,_source:RetainedCloneRef<'_,T>,_body:usize,_maximum_work_units:u64)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original work quote"))}
    fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome;
    /// 📤️ Quotes the original ready-owner transfer work before handoff.
    fn take_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original take work quote"))}
    /// ♻️ Quotes the selected child retirement work before closure.
    fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close work quote"))}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>;
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>;
    fn recover_take<'reply>(&mut self,_output:&mut Option<T>,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{Ok(reply.pending())}
    
    /// ⛽️ Quotes the selected original close-start action before effects.
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close-start work quote"))}
    /// 🚦️ Performs the admitted close-start action and reports actual work.
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"cursor lacks an original close-start physical quote"))}
fn begin_close_paid(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<crate::RetainedCloneProgress,ValueError>{RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close-start action")),work:0}}
    
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
    fn close_selected(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        let selected=(||{let demand=self.close_demands(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(None)}if self.close_demands(demand.copy_bytes)?!=demand{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected retirement quote changes under its exact admitted copy frontier"))}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}))})();
        match selected{Ok(Some(grant))=>match effect.reborrow(grant,effect.maximum_work_units()){Ok(mut child)=>self.close_step(&mut child),Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}}
    }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();
        RetainedWorkOutcome{result:if grant.maximum_items==0{Ok(RetainedCloneStep::Progress(Default::default()))}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no capacity-bearing close authority").into())},work:0}
    }
}

pub use semio_framework_value_clone_source::{RetainedCloneClose,RetainedEmptyOwner,RetainedCloseSelection,close_retained_binding};


pub use semio_framework_value_control_contract::{retained_clone_take_demand,take_retained_clone_output};

pub use semio_framework_value_control_contract::{retained_clone_cursor_take_demand,take_retained_clone_cursor_output};
