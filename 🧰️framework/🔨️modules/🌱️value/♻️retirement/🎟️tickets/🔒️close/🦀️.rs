//! 🎟️ Resident erased child replies survive callback suspension and unwind.
use crate::{ErasedSnapshotRetirement,OriginalRetirementCloseLoan,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError};
use semio_framework_value_control_contract::{OriginalEffectInvocationIdentity,OriginalEffectReply,OriginalEffectReplyComplete};
#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalLoan{ledger:semio_framework_value_control_contract::NativeAllocationIdentity,recipient:Option<u64>,work:semio_framework_value_control_contract::OriginalWorkBindingIdentity}
/// 🎟️ Keeps the original child reply and invocation beside its caller-owned ticket.
pub struct FactoryTicketClose{reply:Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,invocation:Option<OriginalEffectInvocationIdentity>,grant:Option<RetainedCloneGrant>,work:Option<u64>,original:Option<Option<OriginalLoan>>,body:Option<usize>}
impl Default for FactoryTicketClose{fn default()->Self{Self::vacant()}}
impl FactoryTicketClose{
 /// 🌱️ Creates a vacant ticket dispatcher without issuing an action.
 pub const fn vacant()->Self{Self{reply:None,invocation:None,grant:None,work:None,original:None,body:None}}
 pub fn pending_original_control(&self)->Option<bool>{self.original.map(|loan|loan.is_some())}
 pub fn is_empty(&self)->bool{self.reply.is_none()&&self.invocation.is_none()&&self.grant.is_none()&&self.work.is_none()&&self.original.is_none()&&self.body.is_none()}
 pub fn next_demand<T:ErasedSnapshotRetirement+?Sized>(&self,slot:&Option<Box<T>>,body:usize)->Result<RetirementDemand,ValueError>{if self.invocation.is_some(){if self.body!=Some(body){return Err(ValueError::invariant("ticket close changed its selected body"))}return self.grant.map(|g|RetirementDemand{copy_bytes:g.maximum_copy_bytes,capacity_bytes:g.maximum_capacity_bytes,release_bytes:g.maximum_release_bytes,depth:g.maximum_depth}).ok_or_else(||ValueError::invariant("ticket close invocation lost its original grant"))}match slot{Some(owner)=>super::factory_ticket_demands(owner,body),None=>Ok(RetirementDemand::default())}}
 pub fn next_work<T:ErasedSnapshotRetirement+?Sized>(&self,slot:&Option<Box<T>>,body:usize)->Result<u64,ValueError>{if let Some(work)=self.work{if self.body!=Some(body){return Err(ValueError::invariant("ticket work changed its selected body"))}return Ok(work)}if self.reply.is_some(){return Err(ValueError::invariant("ticket reply lost its original work quote"))}match slot{Some(owner)if !owner.terminal_is_empty()=>owner.next_computational_work(body),Some(owner)=>owner.native_header_release().ok_or_else(||ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"terminal ticket has no native header lifecycle"))?.work(),None=>Ok(0)}}
 pub fn close_into<'reply,T:ErasedSnapshotRetirement+?Sized>(&mut self,slot:&mut Option<Box<T>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.drive(slot,None,body,effect,reply,false)}
 pub fn recover_close_into<'reply,T:ErasedSnapshotRetirement+?Sized>(&mut self,slot:&mut Option<Box<T>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.drive(slot,None,body,effect,reply,true)}
 pub fn close_original_into<'reply,T:ErasedSnapshotRetirement+?Sized>(&mut self,slot:&mut Option<Box<T>>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.drive(slot,Some(original),body,effect,reply,false)}
 pub fn recover_close_original_into<'reply,T:ErasedSnapshotRetirement+?Sized>(&mut self,slot:&mut Option<Box<T>>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.drive(slot,Some(original),body,effect,reply,true)}
 fn drive<'reply,T:ErasedSnapshotRetirement+?Sized>(&mut self,slot:&mut Option<Box<T>>,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let invocation=effect.invocation().ok_or_else(||ValueError::invariant("ticket close requires its original producer invocation"))?.identity();let grant=effect.grant();let loan=match original.as_deref_mut(){None=>None,Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("ticket close changed its original Work"))}Some(OriginalLoan{ledger:original.ledger_identity(),recipient:original.retirement_recipient_identity(),work})}};
  if self.invocation.is_some_and(|value|value!=invocation)||self.grant.is_some_and(|value|value!=grant)||self.original.is_some_and(|value|value!=loan)||self.body.is_some_and(|value|value!=body){return Err(ValueError::invariant("ticket close original invocation or grant changed"))}
  if let Some(result)=self.reply.take(){self.grant=None;self.work=None;self.original=None;self.body=None;let scope=reply.scope(&mut self.invocation,invocation)?;let mut scope=scope;scope.borrow()?.complete(result);return Ok(scope.forwarded())}
  if recover&&self.invocation.is_none(){return Err(ValueError::invariant("ticket close recovery has no original child invocation"))}
  if self.invocation.is_none(){
   let Some(owner)=slot.as_ref()else{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))};
   if owner.requires_original_birth(){return Err(ValueError::invariant("ticket child birth must precede its general effect"))}
   let demand=super::factory_ticket_demands(owner,body)?;
   let work=if owner.terminal_is_empty(){owner.native_header_release().ok_or_else(||ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"terminal ticket has no native header lifecycle"))?.work()?}else{owner.next_computational_work(body)?};
   if !crate::retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if owner.terminal_is_empty(){let release=owner.native_header_release().ok_or_else(||ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"terminal ticket has no native header lifecycle"))?;return release.release_into(slot,effect,reply)}
   self.invocation=Some(invocation);self.grant=Some(grant);self.work=Some(work);self.original=Some(loan);self.body=Some(body);
  }
  let child=slot.as_mut().ok_or_else(||ValueError::invariant("ticket child disappeared during original close"))?;let child_reply=OriginalEffectReply::borrow(&mut self.reply)?.progress();
  if let Some(original)=original.as_mut(){if recover{child.recover_close_original_into(original,body,effect,child_reply)?;}else{child.close_original_into(original,body,effect,child_reply)?;}}
  else if recover{child.recover_close_into(body,effect,child_reply)?;}else{child.close_into(body,effect,child_reply)?;}
  if let Some(result)=self.reply.take(){self.grant=None;self.work=None;self.original=None;self.body=None;let mut scoped=reply.scope(&mut self.invocation,invocation)?;scoped.borrow()?.complete(result);Ok(scoped.forwarded())}else{Ok(reply.pending())}
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
