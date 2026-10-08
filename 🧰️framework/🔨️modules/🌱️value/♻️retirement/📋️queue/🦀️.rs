//! 📋️ Admitted typed frames retain their actual owners until every payload and backing allocation closes.

use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::{ErasedControlledRetirement,admit_controlled_retirement,controlled_retirement_birth_bytes}};
use crate::{ValueError,ValueRefusalKind,list::PagedList,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

#[must_use="retirement queue payloads and backing must reach terminal-empty"]
pub struct RetirementQueue {frames:ManuallyDrop<PagedList<Box<dyn ErasedControlledRetirement>,{usize::MAX}>>}
impl Default for RetirementQueue {fn default()->Self {Self {frames:ManuallyDrop::new(PagedList::default())}}}
impl RetirementQueue {
    pub fn len(&self)->usize {self.frames.len()}
    pub fn has_reserved_slot(&self)->bool {self.frames.has_reserved_slot()}
    pub fn terminal_is_empty(&self)->bool {self.frames.terminal_is_empty()}
    pub const fn frame_birth_bytes<T:RetireOwned>()->usize {controlled_retirement_birth_bytes::<T>()}
    pub fn next_reserve_capacity_byte_demand(&self)->Result<usize,ValueError> {Ok(self.frames.next_capacity_allocation_bytes(self.frames.len().checked_add(1).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"retirement queue length overflow"))?).map_err(ValueError::from)?.unwrap_or(0))}
    pub fn reserve_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let empty=empty_progress();
        if grant.maximum_items==0 || self.has_reserved_slot(){return Ok(empty);}
        if grant.maximum_depth<self.frames.len()+1{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue reservation exceeds admitted depth"));}
        if self.next_reserve_capacity_byte_demand()?>grant.maximum_capacity_bytes{return Ok(empty);}
        let step=self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::from(error.refusal()))?;
        Ok(RetainedCloneProgress {copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..empty})
    }
    pub fn admit_owned<T:RetireOwned>(&mut self,value:T,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,T)> {
        if grant.maximum_items==0{return Err((refusal(ValueRefusalKind::WorkLimit,"retirement queue frame requires one admitted item"),value));}
        if grant.maximum_depth<self.frames.len()+1{return Err((refusal(ValueRefusalKind::DepthLimit,"retirement queue frame exceeds admitted depth"),value));}
        if !self.has_reserved_slot(){return Err((refusal(ValueRefusalKind::OwnershipLimit,"retirement queue frame requires an admitted backing slot"),value));}
        let (owner,progress)=admit_controlled_retirement(value,RetainedCloneGrant {maximum_depth:1,..grant})?;
        assert!(self.frames.push_reserved(owner).is_ok(),"retirement queue reserved slot disappeared");
        Ok(progress)
    }
    pub fn next_copy_byte_demand(&self)->usize {self.frames.get(self.frames.len().saturating_sub(1)).map_or(0,|owner|owner.next_copy_byte_demand())}
    pub fn next_capacity_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,ValueError> {self.frames.get(self.frames.len().saturating_sub(1)).map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(0)}else{owner.next_capacity_byte_demand(maximum_body_bytes)})}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        match self.frames.get(self.frames.len().saturating_sub(1)) {
            Some(owner) if owner.terminal_is_empty()=>Ok(owner.frame_release_bytes()),
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
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=empty_progress();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if self.next_depth_demand()?>grant.maximum_depth{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue exceeds admitted depth"));}
        if self.frames.is_empty() {
            let step=self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}));
        }
        let index=self.frames.len()-1;
        let owner=self.frames.get(index).unwrap();
        if owner.terminal_is_empty() {
            let bytes=owner.frame_release_bytes();
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.frames.pop());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let retained=self.frames.len();
        self.frames.get_mut(index).unwrap().step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-retained,..grant})
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
            Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,
            Ok(RetainedCloneStep::Progress(progress)) if progress.copied_items==0=>RetirementStep::BudgetExhausted,
            Ok(RetainedCloneStep::Progress(progress)) if progress.copied_bytes!=0=>RetirementStep::ProcessedBytes(progress.copied_bytes),
            Ok(RetainedCloneStep::Progress(progress)) if progress.released_bytes!=0=>RetirementStep::Bytes(progress.released_bytes),
            Ok(RetainedCloneStep::Progress(_))=>RetirementStep::Advanced,
        }
    }
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn next_work_byte_demand(&self)->usize {self.next_copy_byte_demand()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_birth_bytes(&self,maximum_body_bytes:usize)->Option<usize> {self.next_capacity_byte_demand(maximum_body_bytes).ok()}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
fn empty_progress()->RetainedCloneProgress {RetainedCloneProgress {copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0}}
fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError {ValueError::literal(kind,message)}
