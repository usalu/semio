//! 📜️ Clone and cursor traits of the retained clone contract: the retained clone trait with its bitwise hooks, the bulk page bytes, the bulk run helpers, the clone cursor trait, the close record and the binding close helper. See [`super`].

use super::*;
pub use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete};

pub use semio_framework_value_retirement::forward_original_value_reply;
pub use semio_framework_value_native_clone_engine::{RetainedCopy,RetainedCursorRelease,NativeArrayPrefix};

pub use semio_framework_value_native_cursor_backing::{NativeProjectionBacking,NativeSeedTemplate};

pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    /// 🪡️ Explicit resource-free first-party seed evidence permits separately funded field initialization.
    const NATIVE_CURSOR_SEED:Option<NativeSeedTemplate<Self::Cursor>>=None;
    /// 🌱️ Constructs only fixed inline metadata; this seed cannot allocate, call out or unwind.
    fn retained_clone_cursor() -> Self::Cursor;
    /// 📋️ Returns only fixed native Copy evidence; this query must be pure, nonallocating, callback-free and nonunwinding.
    fn native_copy()->Option<RetainedCopy<Self>>{None}

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



/// 🎯️ Describes one native frontier without authorizing Work, storage or hierarchy.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedCloneSelection{demand:RetirementDemand,work:u64}
impl RetainedCloneSelection{
    pub const fn new(demand:RetirementDemand,work:u64)->Self{Self{demand,work}}
    pub const fn demand(self)->RetirementDemand{self.demand}
    pub const fn work(self)->u64{self.work}
}
/// 🧷️ Borrows the actual typed projection held beside one privately owned native child.
pub struct RetainedCapturedSource<'a,T:RetainedClone,C>{
    projection:&'a RetainedOwnedProjectionAdmission<T>,
    cursor:*const C,
    invocation:OriginalEffectInvocationIdentity,
    mode:OriginalEffectCallbackMode,
    selection:RetainedCloneSelection,
}
impl<'a,T:RetainedClone,C> RetainedCapturedSource<'a,T,C>{
    /// 🪡️ The private parent retains this exact seeded child and immutable projection through the invocation.
    pub(crate) fn new(projection:&'a RetainedOwnedProjectionAdmission<T>,cursor:&C,effect:&RetainedAllocationEffect<'_>,selection:RetainedCloneSelection)->Result<Self,ValueError>{
        let invocation=effect.invocation().ok_or_else(||ValueError::invariant("captured cursor has no original invocation"))?;
        projection.borrow()?;
        Ok(Self{projection,cursor,invocation:invocation.identity(),mode:invocation.mode(),selection})
    }

    /// 📋️ Carries the same declared frontier with the actual parent-owned source.
    pub fn into_selected(self,cursor:&C,effect:&RetainedAllocationEffect<'_>,mode:OriginalEffectCallbackMode)->Result<(RetainedCloneRef<'a,T>,RetainedCloneSelection),ValueError>{let selection=self.selection;Ok((self.borrow_for(cursor,effect,mode)?,selection))}
    /// 🔎️ Checks the actual retained child and callback identity without borrowing any replacement source.
    pub fn matches_cursor(&self,cursor:&C,effect:&RetainedAllocationEffect<'_>,mode:OriginalEffectCallbackMode)->bool{
        std::ptr::eq(self.cursor,cursor)&&effect.invocation().map(|invocation|(invocation.identity(),invocation.mode()))==Some((self.invocation,self.mode))&&self.mode==mode
    }
    /// 🪪️ Validates protocol dispatch against the actual child and original invocation before borrowing its owner.
    pub fn borrow_for(self,cursor:&C,effect:&RetainedAllocationEffect<'_>,mode:OriginalEffectCallbackMode)->Result<RetainedCloneRef<'a,T>,ValueError>{
        if !self.matches_cursor(cursor,effect,mode){return Err(ValueError::invariant("captured source changed its actual cursor or invocation"))}
        self.projection.borrow()
    }
}

pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    /// 🎯️ Selects one paired frontier; unknown cursors retain their defining checked queries.
    fn advance_selection(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{Ok(RetainedCloneSelection::new(self.advance_demands(source,body,maximum)?,self.advance_work_demand(source,body,maximum)?))}
    fn captured_advance_selection(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{Ok(RetainedCloneSelection::new(self.captured_advance_demands(source,body,maximum)?,self.captured_advance_work_demand(source,body,maximum)?))}

    /// 📏️ Quotes a child resumed from its parent's actual typed original projection.
    fn captured_advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{self.advance_demands(source,body,maximum)}
    fn captured_advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{self.advance_work_demand(source,body,maximum)}
    /// 📥️ Unknown children retain their complete original Source authentication.
    fn advance_captured_into<'r>(&mut self,source:RetainedCapturedSource<'_,T,Self>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>where Self:Sized{let original=source.borrow_for(self,effect,OriginalEffectCallbackMode::Receive)?;self.advance_into(original,body,maximum,effect,reply)}
    /// 🔁️ Recovers only the parent-owned child under its same original invocation.
    fn recover_advance_captured_into<'r>(&mut self,source:RetainedCapturedSource<'_,T,Self>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>where Self:Sized{let original=source.borrow_for(self,effect,OriginalEffectCallbackMode::Recover)?;self.recover_advance_into(original,body,maximum,effect,reply)}

    /// 🎟️ This pure, nonallocating, callback-free and nonunwinding query returns explicit native lifecycle evidence; unknown cursors retain their real header.
    fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>> where Self:Sized{None}
    /// 📥️ Records an actual advance reply in the original caller-owned slot.
    fn advance_into<'reply>(&mut self,_source:RetainedCloneRef<'_,T>,_body:usize,_maximum_work_units:u64,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed advance producer"))}
    /// 🔁️ Recovers only the existing advance invocation and its resident output.
    fn recover_advance_into<'reply>(&mut self,_source:RetainedCloneRef<'_,T>,_body:usize,_maximum_work_units:u64,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed advance recovery"))}
    /// 📬️ Lends the original reply directly to the selected close producer.
    fn close_into<'reply>(&mut self,body:usize,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed close producer"))}
    fn recover_close_into<'reply>(&mut self,body:usize,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed close recovery"))}
    fn close_original_into<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_control(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed original close producer"))}self.close_into(body,effect,reply)}
    fn recover_close_original_into<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_control(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed original close recovery"))}self.recover_close_into(body,effect,reply)}


    /// 🪢️ A live original source may witness a native borrowed alias return.
    fn close_bound_demands(&self,_source:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
    fn close_bound_work_demand(&self,_source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{self.close_work_demand(body)}
    fn close_bound_into<'r>(&mut self,_source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_into(body,effect,reply)}
    fn recover_close_bound_into<'r>(&mut self,_source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_close_into(body,effect,reply)}
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
    /// 📤️ Quotes the original ready-owner transfer work before handoff.
    fn take_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original take work quote"))}
    /// ♻️ Quotes the selected child retirement work before closure.
    fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor lacks an original close work quote"))}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>;
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>;
    fn recover_take<'reply>(&mut self,_output:&mut Option<T>,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{Ok(reply.pending())}
    

    /// 🚦️ Publishes a close-start reply before any child producer boundary returns.
    fn begin_close_into<'reply>(&mut self,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed close-start producer"))}
    fn recover_begin_close_into<'reply>(&mut self,_effect:&mut RetainedAllocationEffect<'_>,_reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no borrowed close-start recovery"))}

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
    /// 🎯️ Executes only the original selected body under the same physical and Work receipt.
    fn close_selected(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
     if self.terminal_is_empty(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
     let selected:Result<Option<RetainedCloneGrant>,ValueError>=(||{let demand=self.close_demands(body)?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(None)}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}))})();
     match selected{Ok(Some(grant))=>{let maximum=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};match effect.reborrow(grant,maximum){Ok(mut child)=>self.close_step(body,&mut child),Err(error)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}},Ok(None)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),Err(error)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
    }
    fn requires_original_birth(&self)->bool{false}
    fn close_original_birth_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no selected original birth").into()),work:0})}
    fn requires_original_control(&self)->bool{false}
    fn close_original_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.close_step(body,effect)}
    fn close_step(&mut self,body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"clone cursor has no original close implementation").into()),work:0})}

}

pub use semio_framework_value_clone_source::{RetainedCloneClose,RetainedEmptyOwner,RetainedCloseSelection,close_retained_binding};


pub use semio_framework_value_control_contract::{retained_clone_take_demand,take_retained_clone_output};

pub use semio_framework_value_control_contract::{retained_clone_cursor_take_demand,take_retained_clone_cursor_output};
