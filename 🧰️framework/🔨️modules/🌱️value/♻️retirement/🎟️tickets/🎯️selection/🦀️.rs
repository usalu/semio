//! 🎯️ Exclusive original cursor and ticket loans preserve the selected action across grant refusal.
use super::*;
enum Owner<'a,T:ErasedSnapshotRetirement+?Sized>{Cursor(&'a mut T),Ticket(&'a mut Option<Box<T>>)}
/// 🧷️ A selected original action cannot be replayed or redirected while its exclusive owner loan exists.
pub struct RetirementSelection<'a,T:ErasedSnapshotRetirement+?Sized=dyn ErasedSnapshotRetirement>{owner:Owner<'a,T>,body:usize,demand:RetirementDemand}
/// 🎟️ Borrows an original typed cursor without allocating or advancing its close frontier.
pub fn select_retirement_cursor<T:ErasedSnapshotRetirement+?Sized>(owner:&mut T,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.next_demand(body)?;Ok(RetirementSelection{owner:Owner::Cursor(owner),body,demand})}
/// 🎫️ Borrows the actual optional ticket including its separately quoted physical shell release.
pub fn select_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(owner:&mut Option<Box<T>>,body:usize)->Result<RetirementSelection<'_,T>,ValueError>{let demand=owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))?;Ok(RetirementSelection{owner:Owner::Ticket(owner),body,demand})}
impl<T:ErasedSnapshotRetirement+?Sized> RetirementSelection<'_,T>{
 /// 📏️ Returns all currencies from the same immutable original action.
 pub fn demands(&self)->RetirementDemand{self.demand}
 /// 🧮️ Returns the actual body policy selected before observing a potentially smaller grant.
 pub fn body(&self)->usize{self.body}
 fn demand_at(&self,body:usize)->Result<RetirementDemand,ValueError>{match &self.owner{Owner::Cursor(owner)=>owner.next_demand(body),Owner::Ticket(owner)=>owner.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))}}
 fn admitted(&self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneGrant>,ValueError>{if !crate::retained_clone::retained_clone_grant_admits(grant,self.demand){return Ok(None)}if self.demand_at(self.demand.copy_bytes)?!=self.demand{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected retirement quote changes under its exact admitted copy frontier"))}Ok(Some(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.demand.copy_bytes,maximum_capacity_bytes:self.demand.capacity_bytes,maximum_release_bytes:self.demand.release_bytes,maximum_depth:self.demand.depth}))}
 /// ⏭️ Executes the selected ordinary action after every original currency is admitted.
 pub fn step(self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let Some(grant)=self.admitted(grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};match self.owner{Owner::Cursor(owner)=>owner.close_step(grant),Owner::Ticket(owner)=>close_factory_ticket(owner,grant)}}
 /// 🪪️ Forwards the actual original recipient only after admission of the unchanged selected action.
 pub fn step_controlled(self,grant:RetainedCloneGrant,control:&mut dyn crate::OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{let Some(grant)=self.admitted(grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};match self.owner{Owner::Cursor(owner)=>{if !owner.requires_original_control(){control.charge(self.demand.capacity_bytes)?;}owner.close_step_controlled(grant,control)},Owner::Ticket(owner)=>close_factory_ticket_controlled(owner,grant,control)}}
}
