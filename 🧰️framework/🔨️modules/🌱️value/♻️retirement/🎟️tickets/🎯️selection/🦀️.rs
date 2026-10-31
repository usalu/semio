//! 🎯️ A selected close owns fresh CPU0 on refusal and exact actual child work on invocation.
use crate::{ErasedSnapshotRetirement,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneStep,OriginalRetirementControl,factory_ticket_demands,close_factory_ticket,close_factory_ticket_controlled};
enum Owner<'a,T:ErasedSnapshotRetirement+?Sized>{Cursor(&'a mut T),Ticket(&'a mut Option<Box<T>>)}
pub struct RetirementSelection<'a,T:ErasedSnapshotRetirement+?Sized=dyn ErasedSnapshotRetirement>{owner:Owner<'a,T>,body:usize,demand:RetirementDemand}
/// 🧷️ Borrows the actual cursor and immutable selected quote without allocating or advancing.
pub fn select_retirement_cursor<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.next_demand(body)?;Ok(RetirementSelection{owner:Owner::Cursor(owner),body,demand})}
/// 🎫️ Borrows the actual optional ticket and its separately quoted physical shell release.
pub fn select_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(owner:&mut Option<Box<T>>,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))?;Ok(RetirementSelection{owner:Owner::Ticket(owner),body,demand})}
impl<T:ErasedSnapshotRetirement+?Sized> RetirementSelection<'_,T>{
 /// 📏️ Borrows all currencies from the same selected action.
 pub fn demands(&self)->RetirementDemand{self.demand}
 /// 🧮️ Borrows the exact original body policy selected before observing a narrower grant.
 pub fn body(&self)->usize{self.body}
 fn demand_at(&self,body:usize)->Result<RetirementDemand,ValueError>{match &self.owner{Owner::Cursor(owner)=>owner.next_demand(body),Owner::Ticket(owner)=>owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))}}
 fn admitted(&self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneGrant>,ValueError>{if !crate::retained_clone::retained_clone_grant_admits(grant,self.demand){return Ok(None)}if self.demand_at(self.demand.copy_bytes)?!=self.demand{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected retirement quote changes under its exact admitted copy frontier"))}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.demand.copy_bytes,maximum_capacity_bytes:self.demand.capacity_bytes,maximum_release_bytes:self.demand.release_bytes,maximum_depth:self.demand.depth}))}
 /// ⏭️ Executes only this selected action; pre-owner refusals carry a fresh neutral observation.
 pub fn step(self,grant:RetainedCloneGrant)->RetainedWorkOutcome{let grant=match self.admitted(grant){Ok(Some(grant))=>grant,Ok(None)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};match self.owner{Owner::Cursor(owner)=>owner.close_step(grant),Owner::Ticket(owner)=>close_factory_ticket(owner,grant)}}
 /// 🪪️ Forwards the genuine original recipient after all independent selected axes admit.
 pub fn step_controlled(self,grant:RetainedCloneGrant,control:&mut dyn OriginalRetirementControl)->RetainedWorkOutcome{let grant=match self.admitted(grant){Ok(Some(grant))=>grant,Ok(None)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};match self.owner{Owner::Cursor(owner)=>{if !owner.requires_original_control(){if let Err(error)=control.charge(self.demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}}owner.close_step_controlled(grant,control)},Owner::Ticket(owner)=>close_factory_ticket_controlled(owner,grant,control)}}
}
