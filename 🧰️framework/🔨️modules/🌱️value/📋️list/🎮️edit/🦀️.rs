//! 🎮️ Moves ordered native list slots one admitted step at a time.

use super::{PagedList,PagedListAllocationError,PagedListError,PagedListProgress,PagedListRefusalKind,PagedListReservationFailure,PagedListReservationOutcome};
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneGrant,ValueError,ValueRefusalKind};
/// 🪢️ Keeps original edit, allocation and Work refusals independent.
#[derive(Debug)]
pub enum PagedListEditFailure{Edit(PagedListError),Allocation{error:PagedListAllocationError,copied_bytes:usize},Work(ValueError)}
impl From<PagedListError> for PagedListEditFailure{fn from(error:PagedListError)->Self{Self::Edit(error)}}
impl From<PagedListReservationFailure> for PagedListEditFailure{fn from(error:PagedListReservationFailure)->Self{match error{PagedListReservationFailure::Allocation{error,copied_bytes}=>Self::Allocation{error,copied_bytes},PagedListReservationFailure::Work(error)=>Self::Work(error)}}}
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListEditStep {
    pub complete: bool,
    pub moved_items: usize,
    pub progress: PagedListProgress,
}

/// 🚦️ Distinguishes an unadmitted edit from a completed original edit step.
#[derive(Debug)]
pub enum PagedListEditOutcome{Denied,Ready(PagedListEditStep)}

pub struct PagedListEditCursor {
    insertion: bool,
    target: usize,
    position: usize,
    expected_length: usize,
    owner: Option<usize>,
    started: bool,
    complete: bool,
}

impl PagedListEditCursor {
    pub fn insert(index: usize, original_length: usize) -> Self {
        Self { insertion: true, target: index, position: original_length, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn remove(index: usize, original_length: usize) -> Self {
        Self { insertion: false, target: index, position: index, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn is_finished(&self) -> bool { self.complete }

    pub fn step_demand<T,const N:usize>(&self,owner:&PagedList<T,N>,item:&Option<T>)->Result<semio_framework_value_control_contract::RetirementDemand,PagedListEditFailure>{use semio_framework_value_control_contract::RetirementDemand;let fail=|reason|PagedListEditFailure::Edit(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason});if self.owner.is_some_and(|address|address!=owner as *const _ as usize)||owner.len()!=self.expected_length{return Err(fail("ordered list edit owner or length changed"))}if self.complete{return Ok(Default::default())}if self.target>owner.len()||(!self.insertion&&self.target==owner.len()){return Err(fail("ordered list edit index is outside its owner"))}if self.owner.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<usize>>(),depth:1,..Default::default()})}if self.insertion&&!self.started{if item.is_none(){return Err(fail("ordered list insertion lost its source owner"))}if !owner.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:owner.next_reserve_copy_byte_demand()?,capacity_bytes:owner.next_allocation_bytes()?,depth:owner.next_reserve_depth_demand()?,..Default::default()})}return Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<T>()+2*size_of::<usize>()+2*size_of::<bool>(),depth:owner.next_push_depth_demand()?,..Default::default()})}if !self.insertion&&item.is_some(){return Err(fail("ordered list removal destination already owns an item"))}let shifting=if self.insertion{self.position>self.target}else{self.position+1<owner.len()};if shifting{return Ok(RetirementDemand{copy_bytes:3*size_of::<T>()+size_of::<usize>()+2*size_of::<bool>(),depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<T>()+2*size_of::<usize>()+size_of::<bool>(),depth:owner.next_pop_depth_demand()?,..Default::default()})}
    pub fn step_work_demand<T,const N:usize>(&self,owner:&PagedList<T,N>,item:&Option<T>)->Result<u64,PagedListEditFailure>{self.step_demand(owner,item)?;Ok(u64::from(!self.complete))}
    pub fn step<T,const N:usize>(&mut self,owner:&mut PagedList<T,N>,item:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<PagedListEditOutcome,PagedListEditFailure>{use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedWorkPoll};let demand=match self.step_demand(owner,item){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if self.complete{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,..Default::default()})),work:0})}let grant=effect.grant();if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand)||effect.maximum_work_units()==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Denied),work:0})}if self.owner.is_some()&&self.insertion&&!self.started&&!owner.has_reserved_slot(){let turn=owner.reserve_one(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|match step{PagedListReservationOutcome::Denied=>PagedListEditOutcome::Denied,PagedListReservationOutcome::Ready(progress)=>PagedListEditOutcome::Ready(PagedListEditStep{moved_items:usize::from(progress.progressed),progress,complete:false})}).map_err(Into::into),work:turn.work})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(ValueError::literal(ValueRefusalKind::WorkLimit,"ordered list edit exceeded its work authority"))),work:1}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:1})}let result=(||{if self.owner.is_none(){self.owner=Some(owner as *const _ as usize);return Ok(PagedListEditOutcome::Ready(PagedListEditStep{progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()},..Default::default()}))}if self.insertion&&!self.started{let mut progress=owner.place_reserved(item,grant.maximum_copy_bytes)?;if progress.progressed{self.expected_length+=1;self.started=true;self.complete=self.position==self.target;progress.work=1;progress.placed_bytes=demand.copy_bytes}return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:usize::from(progress.progressed),progress}))}let shifting=if self.insertion{self.position>self.target}else{self.position+1<owner.len()};if shifting{if self.insertion{owner.swap(self.position-1,self.position);self.position-=1}else{owner.swap(self.position,self.position+1);self.position+=1}self.started=true;self.complete=self.insertion&&self.position==self.target;return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()}}))}*item=owner.pop();self.expected_length-=1;self.complete=true;Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()}}))})();RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:1})}

}
