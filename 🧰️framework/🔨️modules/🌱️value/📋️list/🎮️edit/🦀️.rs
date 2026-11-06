//! 🎮️ Moves ordered native list slots one admitted step at a time.

use super::{PagedList,PagedListAllocationError,PagedListError,PagedListProgress,PagedListRefusalKind,PagedListReservationFailure,PagedListReservationOutcome};
use semio_framework_value_control_contract::{RetainedAllocationEffect,ValueError,ValueRefusalKind};
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

    pub fn step<T,const N:usize>(&mut self,owner:&mut PagedList<T,N>,item:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<PagedListEditOutcome,PagedListEditFailure>{
        fn pay(effect:&mut RetainedAllocationEffect<'_>)->Result<(),PagedListEditFailure>{match effect.performed(1){Ok(true)=>Ok(()),Ok(false)=>Err(PagedListEditFailure::Work(ValueError::literal(ValueRefusalKind::WorkLimit,"original list edit exceeded its work authority"))),Err(error)=>Err(PagedListEditFailure::Work(error))}}
        let before=effect.performed_work();let result=(||->Result<PagedListEditOutcome,PagedListEditFailure>{let refuse=|reason|PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason};let grant=effect.grant();let address=owner as *mut PagedList<T,N> as usize;if self.owner.is_some_and(|expected|expected!=address)||owner.len()!=self.expected_length{return Err(refuse("ordered list edit owner or length changed").into())}if self.complete{return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,..Default::default()}))}if self.target>owner.len()||(!self.insertion&&self.target==owner.len()){return Err(refuse("ordered list edit index is outside its owner").into())}if grant.maximum_items==0||grant.maximum_depth==0||effect.maximum_work_units()==0{return Ok(PagedListEditOutcome::Denied)}
        if self.owner.is_none(){let bytes=size_of::<Option<usize>>();if grant.maximum_copy_bytes<bytes{return Ok(PagedListEditOutcome::Denied)}pay(effect)?;self.owner=Some(address);return Ok(PagedListEditOutcome::Ready(PagedListEditStep{moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:bytes,..Default::default()},..Default::default()}))}
        if self.insertion&&!self.started{if item.is_none(){return Err(refuse("ordered list insertion lost its source owner").into())}if !owner.has_reserved_slot(){let turn=owner.reserve_one(effect);let progress=match turn.result?{PagedListReservationOutcome::Denied=>return Ok(PagedListEditOutcome::Denied),PagedListReservationOutcome::Ready(progress)=>progress};return Ok(PagedListEditOutcome::Ready(PagedListEditStep{moved_items:usize::from(progress.progressed),progress,complete:false}))}if size_of::<T>()>grant.maximum_copy_bytes{return Ok(PagedListEditOutcome::Denied)}pay(effect)?;let progress=owner.place_reserved(item,grant.maximum_copy_bytes)?;if progress.progressed{self.expected_length+=1;self.started=true;self.complete=self.position==self.target}return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:usize::from(progress.progressed),progress}))}
        if !self.insertion&&item.is_some(){return Err(refuse("ordered list removal destination already owns an item").into())}let shifting=if self.insertion{self.position>self.target}else{self.position+1<owner.len()};if shifting{let bytes=size_of::<T>().checked_mul(3).ok_or_else(||refuse("ordered list slot move byte count overflow"))?;if bytes>grant.maximum_copy_bytes{return Ok(PagedListEditOutcome::Denied)}pay(effect)?;if self.insertion{owner.swap(self.position-1,self.position);self.position-=1}else{owner.swap(self.position,self.position+1);self.position+=1}self.started=true;self.complete=self.insertion&&self.position==self.target;return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:bytes,..Default::default()}}))}if size_of::<T>()>grant.maximum_copy_bytes{return Ok(PagedListEditOutcome::Denied)}pay(effect)?;*item=owner.pop();self.expected_length-=1;self.complete=true;Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:size_of::<T>(),..Default::default()}}))
})();let work=effect.performed_work().checked_sub(before).expect("original list edit work moved backwards");semio_framework_value_control_contract::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result,work})

    }
}
