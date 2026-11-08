//! 🔗️ Shared immutable owners retain Arc backing custody until aliases and weak leases close.

pub use semio_framework_value_allocation_return::original;

use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{sync::Arc,mem::ManuallyDrop};




pub struct SharedControlledRetirement<T:RetireOwned+Sync> {source:ManuallyDrop<Option<Arc<T>>>,owned:ManuallyDrop<Option<ControlledRetirement<T>>>,lease_only:bool,last_work:u128}
impl<T:RetireOwned+Sync> SharedControlledRetirement<T> {
    pub fn new(source:Arc<T>)->Self {Self::with_custody(source,false)}
    pub fn lease(source:Arc<T>)->Self {Self::with_custody(source,true)}
    fn with_custody(source:Arc<T>,lease_only:bool)->Self {Self {source:ManuallyDrop::new(Some(source)),owned:ManuallyDrop::new(None),lease_only,last_work:0}}
    pub fn terminal_is_empty(&self)->bool {self.source.is_none()&&self.owned.is_none()}
    /// 🎟️ One original shared lease or one original controlled payload supplies the complete action quote.
    pub fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if let Some(source)=self.source.as_ref(){return Ok(crate::RetirementDemand{release_bytes:if (self.lease_only||Arc::strong_count(source)==1)&&Arc::weak_count(source)==0{arc_bytes::<T>()}else{0},copy_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}
        self.owned.as_ref().map_or(Ok(Default::default()),|owner|if owner.terminal_is_empty(){Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)})
    }    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.next_demand(0).map(|d|d.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.next_demand(copy).map(|d|d.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if let Some(source)=self.source.as_ref(){return Ok(if (self.lease_only||Arc::strong_count(source)==1)&&Arc::weak_count(source)==0{arc_bytes::<T>()}else{0});}self.owned.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.owned.as_ref().map_or(Ok(usize::from(self.source.is_some())),ControlledRetirement::next_depth_demand)}
    /// ⛽️ Quotes the original shared handoff, child step or owner clearance.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_demand(body)?;if self.source.is_some(){return Ok(1)}match self.owned.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
    /// ⏰️ Borrows actual shared handoff, child or terminal custody work of the last invoked turn.
    pub fn last_turn_work(&self)->u128{self.last_work}
    pub fn step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
      if self.source.is_none(){if let Some(owner)=self.owned.as_mut(){if !owner.terminal_is_empty(){let poll=owner.step(effect);self.last_work=poll.work();return poll}}}
      let result=(||{let grant=effect.grant();
        self.last_work=0;let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0||self.next_computational_work(grant.maximum_copy_bytes)?>effect.maximum_work_units(){return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"shared retirement exceeds admitted depth").into());}
        if let Some(source)=self.source.as_mut(){
            if self.lease_only {
                let extent=if Arc::weak_count(source)==0{arc_bytes::<T>()}else{0};
                if grant.maximum_release_bytes<extent{return Ok(RetainedCloneStep::Progress(empty));}
                self.last_work=1;let value=Arc::into_inner(self.source.take().ok_or_else(||ValueError::invariant("source absent"))?);
                let released_bytes=if let Some(value)=value{*self.owned=Some(ControlledRetirement::retain(value));extent}else{0};
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),released_bytes,..empty}));
            }
            let extent=if Arc::weak_count(source)==0{arc_bytes::<T>()}else{0};
            if grant.maximum_release_bytes<extent||Arc::strong_count(source)!=1{return Ok(RetainedCloneStep::Progress(empty));}
            self.last_work=1;let source=self.source.take().ok_or_else(||ValueError::invariant("source absent"))?;
            match Arc::try_unwrap(source){Ok(value)=>{*self.owned=Some(ControlledRetirement::retain(value));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),released_bytes:extent,..empty}));},Err(source)=>{*self.source=Some(source);return Ok(RetainedCloneStep::Progress(empty));}}
        }
        if self.owned.as_ref().is_some_and(|owner|owner.terminal_is_empty()){self.last_work=1;*self.owned=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}Ok(RetainedCloneStep::Complete(empty))
})();crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result,work:self.last_work})
    }
}

impl<T:RetireOwned+Sync> crate::ErasedSnapshotRetirement for SharedControlledRetirement<T> {
    fn requires_original_control(&self)->bool{self.source.is_none()&&self.owned.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&crate::ErasedSnapshotRetirement::requires_original_control(owner))}
    fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let grant=effect.grant();match self.owned.as_mut(){Some(owner) if !owner.terminal_is_empty()&&self.source.is_none()=>{let turn=crate::ErasedSnapshotRetirement::close_step_controlled(owner,&mut effect.nested(grant),control);self.last_work=turn.work();turn},_=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared retirement has no selected original child").into()),work:0})}}
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();match self.owned.as_mut(){Some(owner) if !owner.terminal_is_empty()&&self.source.is_none()=>{let turn=crate::ErasedSnapshotRetirement::close_original_step(owner,original,&mut effect.nested(grant));self.last_work=turn.work();turn},_=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared retirement has no selected original child").into()),work:0})}}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{SharedControlledRetirement::next_computational_work(self,body)}
        fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{SharedControlledRetirement::next_demand(self,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll {self.step(effect)}
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Self::next_copy_byte_demand(self)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Self::next_capacity_byte_demand(self,copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Self::next_release_byte_demand(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
}

/// 📏️ Borrows the original shared-owner cursor frame before moving any lease.
pub const fn shared_retirement_birth_bytes<T:RetireOwned+Sync>()->usize {size_of::<SharedControlledRetirement<T>>()}

/// 🔏️ Admits one concrete shared frame and returns the same original lease on refusal.
pub fn shared_retirement_birth_demand<T:RetireOwned+Sync>()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<Arc<T>>>()+size_of::<SharedControlledRetirement<T>>(),capacity_bytes:size_of::<SharedControlledRetirement<T>>(),depth:1,..Default::default()}}
pub fn admit_shared_retirement<T:RetireOwned+Sync>(source:&mut Option<Arc<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,lease_only:bool)->crate::RetainedWorkOutcome<Option<(Box<dyn crate::ErasedSnapshotRetirement>,RetainedCloneProgress)>>{
 let mut work=0;let result=(||{if source.is_none(){return Ok(None)}if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared payload has no controlled retirement authority"))}
 let demand=shared_retirement_birth_demand::<T>();if effect.maximum_work_units()<1||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return Ok(None)}if effect.actual().is_some(){return Err(ValueError::invariant("shared frame allocator marker is occupied"))}
 work=match super::record_retirement_work(effect,1){Ok(work)=>work,Err((error,actual))=>{work=actual;return Err(error)}};*effect.actual()=Some(0);let layout=std::alloc::Layout::new::<SharedControlledRetirement<T>>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<SharedControlledRetirement<T>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"shared frame allocation failed"))};*effect.actual()=Some(layout.size());
 let owner=unsafe{pointer.as_ptr().write(SharedControlledRetirement::with_custody(source.take().expect("admitted original shared source"),lease_only));Box::from_raw(pointer.as_ptr())};Ok(Some((owner as Box<dyn crate::ErasedSnapshotRetirement>,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()})))})();crate::RetainedWorkOutcome{result,work}
}

impl<T:RetireOwned+Sync> Drop for SharedControlledRetirement<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared retirement abandoned actual Arc allocation ownership");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.owned);}}}}
impl<T:RetireOwned+Sync> RetirementCursor for SharedControlledRetirement<T> {
 fn requires_original_control(&self)->bool{crate::ErasedSnapshotRetirement::requires_original_control(self)}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_step_controlled(self,&mut effect.nested(grant),control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_original_step(self,original,&mut effect.nested(grant)))}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{SharedControlledRetirement::next_computational_work(self,body)}
        fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{SharedControlledRetirement::next_demand(self,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{super::erased_cursor_turn(self.step(effect))}
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned+Sync> RetireOwned for Arc<T> {
    fn retirement_ready(&self)->bool{T::retirement_ready(self.as_ref())}
    type RetirementCursor=SharedControlledRetirement<T>;fn retirement_seed(self)->Self::RetirementCursor {SharedControlledRetirement::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<SharedControlledRetirement<T>>())}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T:RetireOwned+Sync> RetireOwned for SharedControlledRetirement<T> {
    type RetirementCursor=Self;fn retirement_seed(self)->Self{self}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
/// 📦️ Exact original shared backing demand used by the admitted shared retirement owners.
pub fn shared_retirement_allocation_bytes<T>()->usize {std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("shared Arc allocation layout").0.pad_to_align().size()}
pub fn arc_bytes<T>()->usize {shared_retirement_allocation_bytes::<T>()}

#[cfg(test)]
#[path="🔐️unique/🧪️tests/🦀️.rs"]
mod shared_unique_tests;

pub use semio_framework_value_allocation_return::{admit_shared_allocation,admit_shared_allocation_from,shared_allocation_bytes,shared_uninitialized_demand,shared_allocation_value_demand,shared_allocation_slot_demand,try_uninitialized_arc,SharedAllocation};
