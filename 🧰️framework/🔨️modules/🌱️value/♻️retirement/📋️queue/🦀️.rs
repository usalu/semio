//! 📋️ Admitted typed frames retain their actual owners until every payload and backing allocation closes.

use super::{RetireOwned,RetirementCursor,RetirementStep,admit_owned_retirement,owned_retirement_birth_bytes};
use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,list::PagedList,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::mem::ManuallyDrop;

#[must_use="retirement queue payloads and backing must reach terminal-empty"]
pub struct RetirementQueue {frames:ManuallyDrop<PagedList<Box<dyn ErasedSnapshotRetirement>,{usize::MAX}>>,step_progress:RetainedCloneProgress}
impl Default for RetirementQueue {fn default()->Self {Self {frames:ManuallyDrop::new(PagedList::default()),step_progress:Default::default()}}}
impl RetirementQueue {
    pub fn len(&self)->usize {self.frames.len()}
    pub fn has_reserved_slot(&self)->bool {self.frames.has_reserved_slot()}
    pub fn terminal_is_empty(&self)->bool {self.frames.terminal_is_empty()}
    pub const fn frame_birth_bytes<T:RetireOwned>()->usize {owned_retirement_birth_bytes::<T>()}
    pub fn next_reserve_capacity_byte_demand(&self)->Result<usize,ValueError> {Ok(self.frames.next_capacity_allocation_bytes(self.frames.len().checked_add(1).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"retirement queue length overflow"))?).map_err(ValueError::from)?.unwrap_or(0))}
    pub fn reserve_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        self.step_progress=empty_progress();let result=self.reserve_original(grant);match result{Ok(progress)=>{self.step_progress=progress;Ok(progress)},Err(error)=>{self.step_progress=error.retained_progress();Err(error)}}
    }
    fn reserve_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let empty=empty_progress();
        if grant.maximum_items==0 || self.has_reserved_slot(){return Ok(empty);}
        if grant.maximum_depth<self.frames.len()+1{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue reservation exceeds admitted depth"));}
        if self.next_reserve_capacity_byte_demand()?>grant.maximum_capacity_bytes{return Ok(empty);}
        let step=self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error|{let progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..empty};ValueError::from(error.refusal()).with_retained_progress(progress)})?;
        Ok(RetainedCloneProgress {copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..empty})
    }
    pub fn admit_owned<T:RetireOwned>(&mut self,value:T,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,T)> {
        if grant.maximum_items==0{return Err((refusal(ValueRefusalKind::WorkLimit,"retirement queue frame requires one admitted item"),value));}
        if grant.maximum_depth<self.frames.len()+1{return Err((refusal(ValueRefusalKind::DepthLimit,"retirement queue frame exceeds admitted depth"),value));}
        if !self.has_reserved_slot(){return Err((refusal(ValueRefusalKind::OwnershipLimit,"retirement queue frame requires an admitted backing slot"),value));}
        let (owner,progress)=admit_owned_retirement(value,RetainedCloneGrant {maximum_depth:1,..grant})?;
        assert!(self.frames.push_reserved(owner).is_ok(),"retirement queue reserved slot disappeared");
        Ok(progress)
    }
    /// 🎟️ Transfers an original admitted frame into reserved custody without allocating another shell.
    pub fn admit_typed_retirement<T:ErasedSnapshotRetirement+'static>(&mut self,original:&mut Option<Box<T>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if original.is_none()||grant.maximum_items==0||grant.maximum_depth<self.frames.len()+1{return Ok(None)}
        if !self.has_reserved_slot(){return Err(refusal(ValueRefusalKind::OwnershipLimit,"original typed retirement requires its reserved queue slot"))}
        assert!(self.frames.push_reserved(original.take().unwrap()).is_ok(),"original typed queue reserved slot disappeared");Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    /// 🎟️ Transfers an original admitted frame into reserved custody without allocating another shell.
    pub fn admit_retirement(&mut self,owner:Box<dyn ErasedSnapshotRetirement>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Box<dyn ErasedSnapshotRetirement>)> {
        if grant.maximum_items==0{return Err((refusal(ValueRefusalKind::WorkLimit,"retirement queue transfer requires one admitted item"),owner));}
        if grant.maximum_depth<self.frames.len()+1{return Err((refusal(ValueRefusalKind::DepthLimit,"retirement queue transfer exceeds admitted depth"),owner));}
        if !self.has_reserved_slot(){return Err((refusal(ValueRefusalKind::OwnershipLimit,"retirement queue transfer requires an admitted backing slot"),owner));}
        assert!(self.frames.push_reserved(owner).is_ok(),"retirement queue reserved transfer slot disappeared");
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.frames.get(self.frames.len().saturating_sub(1)).map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
    pub fn next_capacity_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,ValueError> {self.frames.get(self.frames.len().saturating_sub(1)).map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(0)}else{owner.next_capacity_byte_demand(maximum_body_bytes)})}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        match self.frames.get(self.frames.len().saturating_sub(1)) {
            Some(owner) if owner.terminal_is_empty()=>Ok(size_of_val(owner.as_ref())),
            Some(owner)=>owner.next_release_byte_demand(),
            None=>self.frames.next_release_allocation_bytes().map_err(ValueError::from),
        }
    }
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {
        if self.terminal_is_empty(){return Ok(0);}
        match self.frames.get(self.frames.len().saturating_sub(1)) {
            Some(owner) if !owner.terminal_is_empty()=>self.frames.len().checked_add(owner.next_depth_demand()?).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue nested depth overflow")),
            Some(_)=>Ok(self.frames.len()),
            None=>Ok(1),
        }
    }
    /// 🧾️ Borrows the original queue turn receipt, including genuine failed-child effects.
    pub fn step_progress(&self)->RetainedCloneProgress{self.step_progress}
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.step_progress=empty_progress();match self.step_original(grant){Ok(step)=>{self.step_progress=step.progress();Ok(step)},Err(error)=>{if error.retained_progress()!=empty_progress(){self.step_progress=error.retained_progress();}Err(error.with_retained_progress(self.step_progress))}}
    }
    fn step_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=empty_progress();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if self.next_depth_demand()?>grant.maximum_depth{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue exceeds admitted depth"));}
        if self.next_copy_byte_demand()?>grant.maximum_copy_bytes||self.next_capacity_byte_demand(grant.maximum_copy_bytes)?>grant.maximum_capacity_bytes||self.next_release_byte_demand()?>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
        if self.frames.is_empty() {
            let step=self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}));
        }
        let index=self.frames.len()-1;
        let owner=self.frames.get(index).unwrap();
        if owner.terminal_is_empty() {
            let bytes=size_of_val(owner.as_ref());
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.frames.pop());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let retained=self.frames.len();
        let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-retained,..grant};
        let owner=self.frames.get_mut(index).unwrap();
        let step=owner.close_step(child_grant)?;
        self.step_progress=step.progress();
        let progress=admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"retirement queue original frame")?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}
impl Drop for RetirementQueue {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retirement queue abandoned original ownership");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.frames);}}}
}
impl RetireOwned for RetirementQueue {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl RetirementCursor for RetirementQueue {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        match self.step(grant) {
            Err(error)=>RetirementStep::Failure(error),
            Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,
            Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),
        }
    }
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_birth_bytes(&self,maximum_body_bytes:usize)->Option<usize> {self.next_capacity_byte_demand(maximum_body_bytes).ok()}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
fn empty_progress()->RetainedCloneProgress {RetainedCloneProgress {copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0}}
fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError {ValueError::literal(kind,message)}
