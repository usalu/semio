//! 🎯️ A selected close owns fresh CPU0 on refusal and exact actual child work on invocation.
use crate::{ErasedSnapshotRetirement,OriginalRetirementPoll,OriginalRetirementFailure,RetirementDemand,RetainedWorkOutcome,RetainedWorkPoll,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneStep,OriginalRetirementControl,factory_ticket_demands,close_factory_ticket,close_factory_ticket_controlled,settle_original_retirement_work};
enum Owner<'a,T:ErasedSnapshotRetirement+?Sized>{Cursor(&'a mut T),Ticket(&'a mut Option<Box<T>>)}
pub struct RetirementSelection<'a,T:ErasedSnapshotRetirement+?Sized=dyn ErasedSnapshotRetirement>{owner:Owner<'a,T>,body:usize,demand:RetirementDemand,work:u64}
/// 🧷️ Borrows the actual cursor and immutable selected quote without allocating or advancing.
pub fn select_retirement_cursor<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.next_demand(body)?;let work=if owner.terminal_is_empty(){0}else{owner.next_computational_work(body)?};Ok(RetirementSelection{owner:Owner::Cursor(owner),body,demand,work})}
/// 🎫️ Borrows the actual optional ticket and its separately quoted physical shell release.
pub fn select_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(owner:&mut Option<Box<T>>,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))?;let work=match owner.as_ref(){None=>0,Some(owner)if owner.terminal_is_empty()=>1,Some(owner)=>owner.next_computational_work(body)?};Ok(RetirementSelection{owner:Owner::Ticket(owner),body,demand,work})}
impl<T:ErasedSnapshotRetirement+?Sized> RetirementSelection<'_,T>{
 /// 📏️ Borrows all currencies from the same selected action.
 pub fn demands(&self)->RetirementDemand{self.demand}
 /// ⛽️ Observes the original computational quote captured with this selected action.
 pub fn work_demand(&self)->u64{self.work}
 /// 🎟️ Quotes the selected prepaid-node birth before its child when the original recipient has no spare.
 pub fn demands_controlled(&self,control:&mut dyn OriginalRetirementControl)->RetirementDemand{if self.needs_storage()&&control.failure_storage_pool().is_empty(){crate::original_failure_storage_birth_demand()}else{self.demand}}
 /// 🧷️ Quotes the same refill or selected child through its original decoder loan.
 pub fn demands_original(&self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>)->RetirementDemand{if self.needs_storage()&&original.failure_storage_pool().is_empty(){crate::original_failure_storage_birth_demand()}else{self.demand}}
 fn needs_storage(&self)->bool{match &self.owner{Owner::Cursor(owner)=>!owner.terminal_is_empty(),Owner::Ticket(owner)=>owner.is_some()}}
 /// 🧮️ Borrows the exact original body policy selected before observing a narrower grant.
 pub fn body(&self)->usize{self.body}
 fn demand_at(&self,body:usize)->Result<RetirementDemand,ValueError>{match &self.owner{Owner::Cursor(owner)=>owner.next_demand(body),Owner::Ticket(owner)=>owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))}}
 fn admitted(&self,effect:&crate::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneGrant>,ValueError>{let grant=effect.grant();if self.work>effect.maximum_work_units(){return Ok(None)}if !crate::retained_clone::retained_clone_grant_admits(grant,self.demand){return Ok(None)}if self.demand_at(self.demand.copy_bytes)?!=self.demand{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected retirement quote changes under its exact admitted copy frontier"))}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.demand.copy_bytes,maximum_capacity_bytes:self.demand.capacity_bytes,maximum_release_bytes:self.demand.release_bytes,maximum_depth:self.demand.depth}))}
 /// ⏭️ Executes the same selected action through its original bounded effect.
 pub fn step(self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=match self.admitted(effect){Ok(Some(grant))=>grant,Ok(None)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  let mut selected=effect.nested(grant);
  match self.owner{Owner::Cursor(owner)=>owner.close_step(&mut selected),Owner::Ticket(owner)=>close_factory_ticket(owner,&mut selected)}
 }
 /// 🪪️ Forwards the genuine original recipient and the same physical marker.
 pub fn step_controlled(self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->OriginalRetirementPoll{
  if self.needs_storage()&&control.failure_storage_pool().is_empty(){return crate::refill_original_failure_storage(control,effect.grant())}
  let grant=match self.admitted(effect){Ok(Some(grant))=>grant,Ok(None)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  let mut selected=effect.nested(grant);
  match self.owner{Owner::Cursor(owner)=>if owner.requires_original_control(){owner.close_step_controlled(&mut selected,control)}else{owner.close_step(&mut selected)},Owner::Ticket(owner)=>close_factory_ticket_controlled(owner,&mut selected,control)}
 }
 /// 🧵️ Forwards this selected action through its exact original decoder loan.
 pub fn step_original(self,effect:&mut crate::RetainedAllocationEffect<'_>,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>)->OriginalRetirementPoll{
  if self.needs_storage()&&original.failure_storage_pool().is_empty(){return crate::refill_original_failure_storage(original,effect.grant())}
  let grant=match self.admitted(effect){Ok(Some(grant))=>grant,Ok(None)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  let mut selected=effect.nested(grant);
  match self.owner{Owner::Ticket(owner)=>crate::close_factory_ticket_original(owner,original,&mut selected),Owner::Cursor(owner)=>if owner.requires_original_control(){owner.close_original_step(original,&mut selected)}else{owner.close_step(&mut selected)}}
 }
}
