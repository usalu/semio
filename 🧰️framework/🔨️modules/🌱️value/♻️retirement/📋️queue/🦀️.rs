//! 📋️ Admitted typed frames retain their actual owners until every payload and backing allocation closes.

use semio_framework_value_retirement::{OriginalRetirementPoll,RetireOwned,RetirementPollExit,admit_owned_retirement,owned_retirement_birth_bytes};
use semio_framework_value_retirement::retirement::{RetirementCursor,RetirementStep};
use semio_framework_value_paged_list::PagedList;
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement};
pub use semio_framework_value_paged_list::list;
use std::mem::ManuallyDrop;

#[must_use="retirement queue payloads and backing must reach terminal-empty"]
pub struct RetirementQueue {frames:ManuallyDrop<PagedList<Box<dyn ErasedSnapshotRetirement>,{usize::MAX}>>,pending:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,step_progress:RetainedCloneProgress}
impl Default for RetirementQueue {fn default()->Self {Self {frames:ManuallyDrop::new(PagedList::with_page_items(8)),pending:ManuallyDrop::new(None),step_progress:Default::default()}}}
impl RetirementQueue {
    pub fn len(&self)->usize {self.frames.len()+usize::from(self.pending.is_some())}
    pub fn has_reserved_slot(&self)->bool {self.frames.has_reserved_slot()}
    pub fn terminal_is_empty(&self)->bool {self.pending.is_none()&&self.frames.terminal_is_empty()}
    pub const fn frame_birth_bytes<T:RetireOwned>()->usize {owned_retirement_birth_bytes::<T>()}
    /// 📏️ Quotes the actual next original backing page and its queue wrapper depth.
    pub fn reserve_demand(&self)->Result<RetirementDemand,ValueError>{if self.has_reserved_slot(){return Ok(Default::default())}Ok(RetirementDemand{copy_bytes:self.frames.next_reserve_copy_byte_demand().map_err(ValueError::from)?,capacity_bytes:self.next_reserve_capacity_byte_demand()?,depth:self.frames.next_reserve_depth_demand().map_err(ValueError::from)?.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue backing depth overflow"))?,..Default::default()})}
    pub fn next_reserve_capacity_byte_demand(&self)->Result<usize,ValueError> {self.frames.next_allocation_bytes().map_err(ValueError::from)}
    pub fn reserve_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{
        self.step_progress=empty_progress();let grant=effect.grant();let demand=match self.reserve_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if self.has_reserved_slot()||effect.maximum_work_units()<1||!retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
        let Some(depth)=grant.maximum_depth.checked_sub(1)else{return RetainedWorkOutcome{result:Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue reservation depth is absent")),work:0}};let child=RetainedCloneGrant{maximum_depth:depth,..grant};let turn=self.frames.reserve_one_funded(&mut effect.nested(child));
        let result=match turn.result{Ok(list::PagedListReservationOutcome::Denied)=>Ok(Default::default()),Ok(list::PagedListReservationOutcome::Ready(progress))=>Ok(progress),Err(list::PagedListReservationFailure::Work(error))=>Err(error),Err(list::PagedListReservationFailure::Allocation{error,copied_bytes})=>{let progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()};Err(ValueError::from(error.refusal()).with_retained_progress(progress))}};
        self.step_progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};RetainedWorkOutcome{result,work:turn.work}
    }
    fn placement_depth(&self)->usize{(self.frames.len()+1).max(self.frames.adoption_depth_demand().unwrap_or(usize::MAX))}
    fn placement_bytes<T:?Sized>()->usize{size_of::<Option<Box<T>>>()+size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>()+size_of::<Box<dyn ErasedSnapshotRetirement>>()+2*size_of::<usize>()}
    pub fn admit_owned_demand<T:RetireOwned>(&self,value:&T)->Result<RetirementDemand,ValueError>{let mut demand=retirement::controlled::controlled_retirement_birth_demand(value)?;demand.copy_bytes=demand.copy_bytes.checked_add(Self::placement_bytes::<dyn ErasedSnapshotRetirement>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"retirement queue placement extent overflow"))?;demand.depth=self.frames.len().checked_add(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue frame depth overflow"))?.max(self.frames.adoption_depth_demand().map_err(ValueError::from)?);Ok(demand)}
    pub fn admit_owned<T:RetireOwned>(&mut self,original:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let Some(value)=original.as_ref()else{return RetainedWorkOutcome{result:Ok(None),work:0}};let demand=match self.admit_owned_demand(value){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let grant=effect.grant();
        let remaining=match effect.remaining_work_units(){Ok(remaining)=>remaining,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining<2||!self.has_reserved_slot()||self.pending.is_some()||!retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let placement=Self::placement_bytes::<dyn ErasedSnapshotRetirement>();let Some(copy)=grant.maximum_copy_bytes.checked_sub(placement)else{return RetainedWorkOutcome{result:Err(refusal(ValueRefusalKind::OwnershipLimit,"retirement queue placement exceeds the copy grant")),work:0}};let child=RetainedCloneGrant{maximum_copy_bytes:copy,maximum_depth:1,..grant};let turn=admit_owned_retirement(original,&mut effect.nested_within(child,remaining-1));let mut work=turn.work;let result=turn.result.and_then(|owner|match owner{None=>Ok(None),Some((owner,mut progress))=>{*self.pending=Some(owner);let placed=self.frames.adopt_reserved(&mut self.pending,effect);work+=placed.work;if let Err(error)=placed.result{return Err(error.with_retained_progress(progress))}if self.pending.is_some(){return Err(refusal(ValueRefusalKind::InvariantViolated,"retirement queue adoption denied after admission").with_retained_progress(progress))}progress.copied_bytes+=placement;Ok(Some(progress))}});RetainedWorkOutcome{result,work}
    }
    pub fn admit_typed_retirement<T:ErasedSnapshotRetirement+'static>(&mut self,original:&mut Option<Box<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let grant=effect.grant();let bytes=Self::placement_bytes::<T>();if original.is_none()||self.pending.is_some()||effect.maximum_work_units()<1||!self.has_reserved_slot()||grant.maximum_items==0||grant.maximum_depth<self.placement_depth()||grant.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(None),work:0}}
        *self.pending=original.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);self.place_pending(bytes,effect)

    }
    pub fn admit_retirement(&mut self,original:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let grant=effect.grant();let bytes=Self::placement_bytes::<dyn ErasedSnapshotRetirement>();if original.is_none()||self.pending.is_some()||effect.maximum_work_units()<1||!self.has_reserved_slot()||grant.maximum_items==0||grant.maximum_depth<self.placement_depth()||grant.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(None),work:0}}
        *self.pending=original.take();self.place_pending(bytes,effect)

    }
    fn place_pending(&mut self,bytes:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let step=self.frames.adopt_reserved(&mut self.pending,effect);let work=step.work;let result=step.result.and_then(|step|if self.pending.is_some(){Err(refusal(ValueRefusalKind::InvariantViolated,"retirement queue adoption denied after admission"))}else{Ok(step.progressed.then_some(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))});RetainedWorkOutcome{result,work}}
    fn selected(&self)->Option<&dyn ErasedSnapshotRetirement>{self.pending.as_deref().or_else(||self.frames.get(self.frames.len().saturating_sub(1)).map(Box::as_ref))}
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}self.selected().map_or(Ok(1),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
    fn requires_original_birth(&self)->bool{self.selected().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_birth())}
    fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{let done=|result:Result<RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:0});let retained=self.len();let Some(depth)=grant.maximum_depth.checked_sub(retained)else{return done(Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue retained frames exceed the admitted depth").into()))};let index=self.frames.len().saturating_sub(1);let owner=if let Some(owner)=self.pending.as_mut(){Some(owner)}else{self.frames.get_mut(index)};match owner{Some(owner)=>owner.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:depth,..grant}),None=>done(Err(refusal(ValueRefusalKind::InvariantViolated,"retirement queue has no selected original birth").into()))}}
    /// ⏭️ Quotes only the original active owner or its terminal frame and queue backing.
    pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        match self.selected(){
            Some(owner)if owner.terminal_is_empty()=>Ok(RetirementDemand{release_bytes:size_of_val(owner),depth:self.len(),..Default::default()}),
            Some(owner)=>{let mut d=owner.next_demand(body)?;d.depth=d.depth.checked_add(self.len()).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue nested depth overflow"))?;Ok(d)},
            None=>Ok(RetirementDemand{copy_bytes:self.frames.next_release_copy_byte_demand().map_err(ValueError::from)?,release_bytes:self.frames.next_release_allocation_bytes().map_err(ValueError::from)?,depth:self.frames.next_release_depth_demand().map_err(ValueError::from)?,..Default::default()}),
        }
    }    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.selected().map_or_else(||if self.terminal_is_empty(){Ok(0)}else{self.frames.next_release_copy_byte_demand().map_err(ValueError::from)},|owner|owner.next_copy_byte_demand())}
    pub fn next_capacity_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,ValueError> {self.selected().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(0)}else{owner.next_capacity_byte_demand(maximum_body_bytes)})}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        match self.selected() {
            Some(owner) if owner.terminal_is_empty()=>Ok(size_of_val(owner)),
            Some(owner)=>owner.next_release_byte_demand(),
            None=>self.frames.next_release_allocation_bytes().map_err(ValueError::from),
        }
    }
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {
        if self.terminal_is_empty(){return Ok(0);}
        match self.selected() {
            Some(owner) if !owner.terminal_is_empty()=>self.len().checked_add(owner.next_depth_demand()?).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue nested depth overflow")),
            Some(_)=>Ok(self.len()),
            None=>self.frames.next_release_depth_demand().map_err(ValueError::from),
        }
    }
    /// 🧾️ Borrows the original queue turn receipt, including genuine failed-child effects.
    pub fn step_progress(&self)->RetainedCloneProgress{self.step_progress}
    pub fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.step_with(body,effect,|owner,effect|semio_framework_value_retirement::close_owner(owner,body,effect))}
    fn requires_original_control(&self)->bool{self.selected().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())}
    fn step_with(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn ErasedSnapshotRetirement,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->OriginalRetirementPoll {let mut work=0;self.step_progress=empty_progress();
        let result=match self.step_original(body,effect,invoke,&mut work){Ok(step)=>{self.step_progress=step.progress();Ok(step)},Err(RetirementPollExit::Failed(error))=>{if error.retained_progress()!=empty_progress(){self.step_progress=error.retained_progress();}Err(RetirementPollExit::Failed(error.with_retained_progress(self.step_progress)))},Err(pending)=>Err(pending)};
        RetirementPollExit::finish(result,work)}
    fn step_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn ErasedSnapshotRetirement,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit> {let grant=effect.grant();
        let empty=empty_progress();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0||effect.maximum_work_units()<self.next_computational_work(body)?{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.next_demand(body)?;
        if demand.depth>grant.maximum_depth{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue exceeds admitted depth").into());}
        if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
        if self.frames.is_empty()&&self.pending.is_none() {
            let turn=self.frames.release_empty_page(effect);*work=turn.work;
            let step=match turn.result{Ok(list::PagedListReleaseOutcome::Denied)=>return Ok(RetainedCloneStep::Progress(empty)),Ok(list::PagedListReleaseOutcome::Ready(step))=>step,Err(list::PagedListReleaseFailure::Work(error))=>return Err(error.into()),Err(list::PagedListReleaseFailure::Release(error))=>return Err(ValueError::from(error).into())};
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}));
        }
        let index=self.frames.len().saturating_sub(1);
        let owner=self.selected().ok_or_else(||ValueError::invariant("retirement queue has no selected owner"))?;
        if owner.terminal_is_empty() {
            let bytes=size_of_val(owner);
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            if !effect.performed(1)?{return Err(refusal(ValueRefusalKind::WorkLimit,"retirement queue pop exceeds its work grant").into())}*work=1;if self.pending.is_some(){self.pending.take();}else{self.frames.pop();}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let retained=self.len();
        let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth.checked_sub(retained).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue retained frames exceed the admitted depth"))?,..grant};
        let owner=if let Some(owner)=self.pending.as_mut(){owner}else{self.frames.get_mut(index).ok_or_else(||ValueError::invariant("retirement queue lost its selected frame"))?};
        let turn=RetirementPollExit::settled(invoke(owner.as_mut(),&mut effect.nested(child_grant)))?;*work=turn.work;let step=turn.result?;
        self.step_progress=step.progress();
        let progress=admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"retirement queue original frame")?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}
impl Drop for RetirementQueue {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retirement queue abandoned original ownership");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.frames);ManuallyDrop::drop(&mut self.pending);}}}
}
impl RetireOwned for RetirementQueue {
    type RetirementCursor=Self;
    fn retirement_seed(self)->Self::RetirementCursor{self}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
fn cursor_turn(poll:OriginalRetirementPoll)->RetirementStep{retirement::erased_cursor_turn(poll)}
impl RetirementCursor for RetirementQueue {
 fn requires_original_birth(&self)->bool{RetirementQueue::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{cursor_turn(RetirementQueue::close_original_birth_step(self,original,grant))}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{RetirementQueue::next_computational_work(self,body)}

 fn requires_original_control(&self)->bool{RetirementQueue::requires_original_control(self)}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(refusal(ValueRefusalKind::UnsupportedOwner,"retirement queue has no selected original child").into(),0)}cursor_turn(self.step_with(body,effect,|owner,effect|semio_framework_value_retirement::close_owner_controlled(owner,body,effect,control)))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(refusal(ValueRefusalKind::UnsupportedOwner,"retirement queue has no selected original child").into(),0)}cursor_turn(self.step_with(body,effect,|owner,effect|semio_framework_value_retirement::close_owner_original(owner,original,body,effect)))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetirementQueue::next_demand(self,body)}
    
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_birth_bytes(&self,maximum_body_bytes:usize)->Option<usize> {self.next_capacity_byte_demand(maximum_body_bytes).ok()}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}

fn empty_progress()->RetainedCloneProgress {RetainedCloneProgress {copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0}}
fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError {ValueError::literal(kind,message)}

#[cfg(test)]
#[path="🧪️tests/🎮️ownership/🦀️.rs"]
mod original_ownership_tests;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
#[path="🧪️tests/🎟️turn/🦀️.rs"]
mod original_turn_tests;
#[cfg(test)]
#[path="🧪️tests/🎮️controlled/🦀️.rs"]
mod original_controlled_tests;

#[cfg(test)]
#[path="🧪️tests/bank.rs"]
mod queue_bank;

#[cfg(test)]
#[path="🧪️tests/actual.rs"]
mod actual_work_tests;


