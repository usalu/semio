//! 📋️ Admitted typed frames retain their actual owners until every payload and backing allocation closes.

use semio_framework_value_retirement::{RetireOwned,admit_owned_retirement,owned_retirement_birth_bytes};
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
        if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(refusal(ValueRefusalKind::InvariantViolated,"retirement queue allocator marker is occupied")),work:0}}let lower=self.frames.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant},effect.actual());let work=1;let result=lower.map_err(|error|ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}));self.step_progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};RetainedWorkOutcome{result,work}
    }
    fn placement_bytes<T:?Sized>()->usize{size_of::<Option<Box<T>>>()+size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>()+size_of::<Box<dyn ErasedSnapshotRetirement>>()+2*size_of::<usize>()}
    pub fn admit_owned_demand<T:RetireOwned>(&self,value:&T)->Result<RetirementDemand,ValueError>{let mut demand=retirement::controlled::controlled_retirement_birth_demand(value)?;demand.copy_bytes=demand.copy_bytes.checked_add(Self::placement_bytes::<dyn ErasedSnapshotRetirement>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"retirement queue placement extent overflow"))?;demand.depth=self.frames.len().checked_add(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue frame depth overflow"))?;Ok(demand)}
    pub fn admit_owned<T:RetireOwned>(&mut self,original:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let Some(value)=original.as_ref()else{return RetainedWorkOutcome{result:Ok(None),work:0}};let demand=match self.admit_owned_demand(value){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let grant=effect.grant();
        if effect.maximum_work_units()<2||!self.has_reserved_slot()||self.pending.is_some()||!retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let placement=Self::placement_bytes::<dyn ErasedSnapshotRetirement>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,maximum_depth:1,..grant};let turn=admit_owned_retirement(original,&mut effect.reborrow(child,effect.maximum_work_units()-1).expect("queue child exceeds original effect"));let mut work=turn.work;let result=turn.result.and_then(|owner|match owner{None=>Ok(None),Some((owner,mut progress))=>{*self.pending=Some(owner);let placed=self.frames.adopt_reserved(&mut self.pending).map_err(|error|ValueError::from(error).with_retained_progress(progress))?;work+=placed.work;progress.copied_bytes+=placement;Ok(Some(progress))}});RetainedWorkOutcome{result,work}
    }
    pub fn admit_typed_retirement<T:ErasedSnapshotRetirement+'static>(&mut self,original:&mut Option<Box<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let grant=effect.grant();let bytes=Self::placement_bytes::<T>();if original.is_none()||self.pending.is_some()||effect.maximum_work_units()<1||!self.has_reserved_slot()||grant.maximum_items==0||grant.maximum_depth<self.frames.len()+1||grant.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(None),work:0}}
        *self.pending=original.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);self.place_pending(bytes)

    }
    pub fn admit_retirement(&mut self,original:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
        let grant=effect.grant();let bytes=Self::placement_bytes::<dyn ErasedSnapshotRetirement>();if original.is_none()||self.pending.is_some()||effect.maximum_work_units()<1||!self.has_reserved_slot()||grant.maximum_items==0||grant.maximum_depth<self.frames.len()+1||grant.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(None),work:0}}
        *self.pending=original.take();self.place_pending(bytes)

    }
    fn place_pending(&mut self,bytes:usize)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{match self.frames.adopt_reserved(&mut self.pending){Ok(step)=>RetainedWorkOutcome{result:Ok(step.progressed.then_some(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:step.work},Err(error)=>RetainedWorkOutcome{result:Err(ValueError::from(error)),work:0}}}
    fn selected(&self)->Option<&dyn ErasedSnapshotRetirement>{self.pending.as_deref().or_else(||self.frames.get(self.frames.len().saturating_sub(1)).map(Box::as_ref))}
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}self.selected().map_or(Ok(1),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
    fn requires_original_birth(&self)->bool{self.selected().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_birth())}
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{let retained=self.len();let Some(depth)=grant.maximum_depth.checked_sub(retained)else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}};let index=self.frames.len().saturating_sub(1);let owner=if let Some(owner)=self.pending.as_mut(){Some(owner)}else{self.frames.get_mut(index)};match owner{Some(owner)=>owner.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:depth,..grant}),None=>RetainedWorkOutcome{result:Err(refusal(ValueRefusalKind::InvariantViolated,"retirement queue has no selected original birth").into()),work:0}}}
    /// ⏭️ Quotes only the original active owner or its terminal frame and queue backing.
    pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        match self.selected(){
            Some(owner)if owner.terminal_is_empty()=>Ok(RetirementDemand{release_bytes:size_of_val(owner),depth:self.len(),..Default::default()}),
            Some(owner)=>{let mut d=owner.next_demand(body)?;d.depth=d.depth.checked_add(self.len()).ok_or_else(||refusal(ValueRefusalKind::DepthLimit,"retirement queue nested depth overflow"))?;Ok(d)},
            None=>Ok(RetirementDemand{release_bytes:self.frames.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()}),
        }
    }    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.selected().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
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
            None=>Ok(1),
        }
    }
    /// 🧾️ Borrows the original queue turn receipt, including genuine failed-child effects.
    pub fn step_progress(&self)->RetainedCloneProgress{self.step_progress}
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{self.step_with(effect,|owner,effect|owner.close_step(effect))}
    fn requires_original_control(&self)->bool{self.selected().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())}
    fn step_with(&mut self,effect:&mut RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn ErasedSnapshotRetirement,&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome)->semio_framework_value_retirement::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(||{
        self.step_progress=empty_progress();match {let turn=self.step_original(effect,invoke);work=turn.work;turn.result}{Ok(step)=>{self.step_progress=step.progress();Ok(step)},Err(error)=>{if error.retained_progress()!=empty_progress(){self.step_progress=error.retained_progress();}Err(error.with_retained_progress(self.step_progress))}}
    })();RetainedWorkOutcome{result,work}}
    fn step_original(&mut self,effect:&mut RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn ErasedSnapshotRetirement,&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome)->semio_framework_value_retirement::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(||{
        let empty=empty_progress();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0||effect.maximum_work_units()<self.next_computational_work(grant.maximum_copy_bytes)?{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.next_demand(grant.maximum_copy_bytes)?;
        if demand.depth>grant.maximum_depth{return Err(refusal(ValueRefusalKind::DepthLimit,"retirement queue exceeds admitted depth").into());}
        if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
        if self.frames.is_empty()&&self.pending.is_none() {
            let step=self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;work=step.work;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}));
        }
        let index=self.frames.len().saturating_sub(1);
        let owner=self.selected().unwrap();
        if owner.terminal_is_empty() {
            let bytes=size_of_val(owner);
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            work=1;if self.pending.is_some(){self.pending.take();}else{self.frames.pop();}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let retained=self.len();
        let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-retained,..grant};
        let owner=if let Some(owner)=self.pending.as_mut(){owner}else{self.frames.get_mut(index).unwrap()};
        let turn=invoke(owner.as_mut(),&mut effect.reborrow(child_grant,effect.maximum_work_units()).expect("queue child exceeds original effect"));work=turn.work;let step=turn.result?;
        self.step_progress=step.progress();
        let progress=admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"retirement queue original frame")?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    })();RetainedWorkOutcome{result,work}}
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
fn cursor_turn(turn:semio_framework_value_retirement::OriginalRetirementOutcome)->RetirementStep{match turn.result{Err(error)=>RetirementStep::Failure(error,turn.work),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work)}}
impl RetirementCursor for RetirementQueue {
 fn requires_original_birth(&self)->bool{RetirementQueue::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{cursor_turn(RetirementQueue::close_original_birth_step(self,original,grant))}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{RetirementQueue::next_computational_work(self,body)}

 fn requires_original_control(&self)->bool{RetirementQueue::requires_original_control(self)}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(refusal(ValueRefusalKind::UnsupportedOwner,"retirement queue has no selected original child").into(),0)}cursor_turn(self.step_with(effect,|owner,effect|owner.close_step_controlled(effect,control)))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(refusal(ValueRefusalKind::UnsupportedOwner,"retirement queue has no selected original child").into(),0)}cursor_turn(self.step_with(effect,|owner,effect|owner.close_original_step(original,effect)))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetirementQueue::next_demand(self,body)}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
        let turn=self.step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));match turn.result {
            Err(error)=>RetirementStep::Failure(error,turn.work),
            Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),
            Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work),
        }
    }
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
#[path="🧪️tests/actual.rs"]
mod actual_work_tests;
