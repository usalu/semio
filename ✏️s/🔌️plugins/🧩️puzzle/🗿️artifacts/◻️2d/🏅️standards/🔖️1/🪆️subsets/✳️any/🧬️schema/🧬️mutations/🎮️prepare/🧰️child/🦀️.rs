//! 🧰️ Owns one genuine native preparation controller under exact birth and terminal grants.

use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_persistent::list::{PagedList,PagedListEditCursor,PagedListEditFailure,PagedListEditOutcome,PagedListEditStep,PagedListReservationFailure,PagedListReservationOutcome};
use std::mem::size_of;


/// 🧾️ Charges one local preparation event to the original effect; `false` means the quote was not funded and nothing was charged.
pub fn pay(effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError> {
    if effect.remaining_work_units()?<1 {return Ok(false);}
    if effect.performed(1)? {Ok(true)} else {Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native preparation event exceeded its quoted Work"))}
}

/// ⏳️ A suspended child keeps its state and retries next turn as zero progress; only a settled failure is an error.
pub fn settled_step(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError> {
    match poll {RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(OriginalRetirementFailure::into_value_error)}
}

/// 🪢️ Runs one non-suspending reply-shaped callee inside the original effect and returns its recorded step.
pub fn replied(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>)->Result<RetainedCloneStep,ValueError> {
    let mut slot=None;
    let reply=OriginalEffectReply::borrow(&mut slot)?;
    drop(action(&mut *effect,reply)?);
    slot.take().ok_or_else(||ValueError::invariant("native preparation callee suspended without a recorded reply"))?.map_err(OriginalRetirementFailure::into_value_error)
}

/// 🪢️ Like [`replied`] for a callee whose failure is already a value refusal.
pub fn replied_value(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>)->Result<RetainedCloneStep,ValueError> {
    let mut slot=None;
    let reply=OriginalEffectReply::borrow(&mut slot)?;
    drop(action(&mut *effect,reply)?);
    slot.take().ok_or_else(||ValueError::invariant("native preparation callee suspended without a recorded reply"))?
}

/// 🧬️ Advances one native clone cursor over its borrowed original source under this effect.
pub fn cursor_advance<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    let maximum=effect.remaining_work_units()?;
    replied(effect,|effect,reply|cursor.advance_into(source,body,maximum,effect,reply))
}

/// 📤️ Moves the ready owner of one native clone cursor into `output`.
pub fn cursor_take<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    replied_value(effect,|effect,reply|cursor.take(output,effect,reply))
}

/// 📤️ Moves the ready owner out of one native clone cursor; `None` means the take was denied and nothing moved.
pub fn cursor_taken<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<T>,ValueError> {
    let mut output=None;
    cursor_take(cursor,&mut output,effect)?;
    Ok(output)
}

/// 🚦️ Starts the paid close of one native clone cursor.
pub fn cursor_begin_close<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError> {
    cursor.begin_close_paid(effect).result
}

/// ♻️ Advances the paid close of one native clone cursor; a suspended child retries next turn.
pub fn cursor_close<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    let body=effect.grant().maximum_copy_bytes;
    settled_step(cursor.close_selected(body,effect))
}

/// ♻️ Starts then advances the paid close of one native clone cursor until it is terminal-empty.
pub fn cursor_retire<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    if cursor.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
    if !cursor.close_started() {return Ok(RetainedCloneStep::Progress(cursor_begin_close(cursor,effect)?));}
    cursor_close(cursor,effect)
}

/// ♻️ Advances one controlled retirement; a suspended child retries next turn.
pub fn controlled_step<T:RetireOwned>(owner:&mut ControlledRetirement<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    let body=effect.grant().maximum_copy_bytes;
    settled_step(owner.step(body,effect))
}

/// 📄️ Reserves one output page slot under this effect; `None` means the reservation was denied and nothing moved.
pub fn list_reserve<T,const N:usize>(list:&mut PagedList<T,N>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError> {
    match list.reserve_one_funded(effect).result {
        Ok(PagedListReservationOutcome::Ready(progress))=>Ok(Some(progress)),
        Ok(PagedListReservationOutcome::Denied)=>Ok(None),
        Err(PagedListReservationFailure::Work(error))=>Err(error),
        Err(PagedListReservationFailure::Allocation{..})=>Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"ordered list page allocation failed")),
    }
}

/// 🩻 Names the refusal of one ordered list edit without losing its cause.
pub fn edit_fault(failure:PagedListEditFailure)->ValueError {
    match failure {
        PagedListEditFailure::Edit(error)=>ValueError::from(error),
        PagedListEditFailure::Work(error)=>error,
        PagedListEditFailure::Allocation{..}=>ValueError::literal(ValueRefusalKind::AllocationFailed,"ordered list edit allocation failed"),
    }
}

/// 🧮️ Runs one ordered list edit step under this effect; `None` means the step was denied or suspended and nothing moved.
pub fn edit_step<T,const N:usize>(edit:&mut PagedListEditCursor,list:&mut PagedList<T,N>,item:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PagedListEditStep>,ValueError> {
    match edit.step(list,item,effect) {
        RetainedWorkPoll::Pending{..}=>Ok(None),
        RetainedWorkPoll::Complete(turn)=>match turn.result {Ok(PagedListEditOutcome::Ready(step))=>Ok(Some(step)),Ok(PagedListEditOutcome::Denied)=>Ok(None),Err(failure)=>Err(edit_fault(failure))},
    }
}

/// 🧾️ The physical receipt of one ordered list edit step.
pub fn edit_progress(step:PagedListEditStep)->RetainedCloneProgress {
    RetainedCloneProgress {copied_items:step.moved_items,copied_bytes:step.progress.placed_bytes,retained_capacity_bytes:step.progress.allocated_bytes,released_bytes:step.progress.released_allocation_bytes}
}

#[derive(Clone,Copy)]
pub enum Puzzle2dCloseAxis { Copy,Capacity(usize),Release,Depth }

impl Puzzle2dCloseAxis {
    pub fn inline<T>(self)->Result<usize,ValueError> {Ok(match self {Self::Copy=>size_of::<T>(),Self::Depth=>1,_=>0})}
    pub fn retained<T:semio_framework_value::retained_clone::RetainedClone,C:semio_framework_value::retained_clone::RetainedCloneCursor<T>>(self,owner:&C)->Result<usize,ValueError> {
        match self {Self::Copy=>owner.next_close_copy_byte_demand(),Self::Capacity(body)=>owner.next_close_capacity_byte_demand(body),Self::Release=>owner.next_close_release_byte_demand(),Self::Depth=>owner.next_close_depth_demand()}
    }
    pub fn retirement<T:semio_framework_value::retirement::RetireOwned>(self,owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>)->Result<usize,ValueError> {
        match self {Self::Copy=>owner.next_copy_byte_demand(),Self::Capacity(body)=>owner.next_capacity_byte_demand(body),Self::Release=>owner.next_release_byte_demand(),Self::Depth=>owner.next_depth_demand()}
    }
    pub fn binding(self,owner:&Option<semio_framework_value::retained_clone::RetainedCloneBinding>)->Result<usize,ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneBinding;
        match self {Self::Copy=>RetainedCloneBinding::copy_demand(owner),Self::Capacity(body)=>RetainedCloneBinding::capacity_demand(owner,body),Self::Release=>RetainedCloneBinding::release_demand(owner),Self::Depth=>RetainedCloneBinding::depth_demand(owner)}
    }
}

pub trait Puzzle2dNativePreparation: Default {
    fn begin_close(&mut self);
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
    fn terminal_is_empty(&self)->bool;
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,ValueError>;
}

pub struct Puzzle2dPreparationChild<C:Puzzle2dNativePreparation> { owner:Option<C>, closing:bool }

impl<C:Puzzle2dNativePreparation> Default for Puzzle2dPreparationChild<C> {
    fn default()->Self { Self {owner:None,closing:false} }
}

impl<C:Puzzle2dNativePreparation> Puzzle2dPreparationChild<C> {
    pub fn ensure(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError> {
        if self.closing {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native preparation child is closing"));}
        if self.owner.is_some() {return Ok(None);}
        let grant=effect.grant();
        let bytes=size_of::<C>();
        if grant.maximum_items==0 || grant.maximum_depth==0 || grant.maximum_copy_bytes < bytes || !pay(effect)? {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        self.owner=Some(C::default());
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()})))
    }

    pub fn owner_mut(&mut self)->&mut C {self.owner.as_mut().expect("native preparation child birth was admitted")}

    pub fn begin_close(&mut self) {self.closing=true;if let Some(owner)=self.owner.as_mut(){owner.begin_close();}}

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError> {
        let Some(owner)=self.owner.as_ref()else{return Ok(0)};
        if !owner.terminal_is_empty(){return match axis {Puzzle2dCloseAxis::Copy=>owner.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>owner.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>owner.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>owner.next_close_depth_demand()};}
        Ok(match axis {Puzzle2dCloseAxis::Copy=>size_of::<C>(),Puzzle2dCloseAxis::Depth=>1,_=>0})
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let result=(||->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit> {
            if !self.closing {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native preparation child must begin close").into());}
            let grant=effect.grant();
            if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
            let Some(owner)=self.owner.as_mut() else {return Ok(RetainedCloneStep::Complete(Default::default()));};
            if !owner.terminal_is_empty() {let step=semio_framework_value::RetirementPollExit::resume(owner.close_step(effect))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
            let bytes=size_of::<C>();
            if grant.maximum_copy_bytes < bytes || !pay(effect)? {return Ok(RetainedCloneStep::Progress(Default::default()));}
            drop(self.owner.take());
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}))
        })();
        semio_framework_value::RetirementPollExit::settle(effect,result)
    }

    pub fn terminal_is_empty(&self)->bool {self.owner.is_none()}
}

impl<C:Puzzle2dNativePreparation> Drop for Puzzle2dPreparationChild<C> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.owner.is_none(),"native preparation child dropped before exact terminal release");}
}
