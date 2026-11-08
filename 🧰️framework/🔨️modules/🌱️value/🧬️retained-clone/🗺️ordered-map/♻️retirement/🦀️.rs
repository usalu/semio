//! ♻️ Native edit retirement retains its original borrowed child reply and actual Work frontier.
use super::*;
use crate::retirement::{RetirementCursor,RetirementStep};
use crate::retained_clone::{OriginalEffectReply as Reply,OriginalEffectReplyComplete as Complete,OriginalEffectCallbackMode as Mode,OriginalEffectInvocationIdentity,OriginalWorkBindingIdentity,OriginalRetirementCloseLoan};
use std::mem::ManuallyDrop;
#[derive(Clone,Copy,PartialEq,Eq)]
pub(super) struct CloseKey{pub body:usize,invocation:OriginalEffectInvocationIdentity,grant:RetainedCloneGrant,work:OriginalWorkBindingIdentity,original:Option<(crate::NativeAllocationIdentity,Option<u64>,OriginalWorkBindingIdentity)>}
#[derive(Clone,Copy,PartialEq,Eq)]
pub(super) struct CloseRoute{pub key:CloseKey,pub operation:u8,pub demand:crate::RetirementDemand,pub work:u64}
pub(super) fn close_key(body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&crate::RetainedAllocationEffect<'_>,recover:bool)->Result<CloseKey,ValueError>{let invalid=||ValueError::invariant("native edit retirement changed its original action");let invocation=effect.invocation().ok_or_else(invalid)?;if invocation.mode()!=if recover{Mode::Recover}else{Mode::Receive}{return Err(invalid())}let original=match original{Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(invalid())}Some((original.ledger_identity(),original.retirement_recipient_identity(),work))},None=>None};Ok(CloseKey{body,invocation:invocation.identity(),grant:effect.grant(),work:OriginalWorkBindingIdentity::from_effect(effect),original})}
#[doc(hidden)]
pub trait EditOwner:Send+'static{
 fn demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>;
 fn work(&self,body:usize)->Result<u64,ValueError>;
 fn close<'r>(&mut self,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn original_birth(&self)->bool;
 fn original_control(&self)->bool;
 fn birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
 fn empty(&self)->bool;
}
#[doc(hidden)]
pub struct EditRetirement<C:EditOwner>{cursor:ManuallyDrop<C>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,crate::OriginalRetirementFailure>>>,route:Option<(CloseKey,u128)>}
impl<C:EditOwner> EditRetirement<C>{
 fn relay<'r>(&mut self,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetirementStep,crate::OriginalRetirementFailure>,recover:bool)->Result<Complete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{
  let key=close_key(body,original.as_deref_mut(),effect,recover)?;let invalid=||ValueError::invariant("native edit retirement lost its retained reply");if recover{if !self.route.is_some_and(|(selected,_)|selected==key){return Err(invalid())}}else{if self.route.is_some()||self.reply.is_some(){return Err(invalid())}self.route=Some((key,effect.performed_work()))}
  if self.reply.is_none(){let done=self.cursor.close(body,original,effect,Reply::borrow(&mut self.reply)?,recover)?;drop(done)}let Some(result)=self.reply.take()else{return Ok(reply.pending())};let(_,start)=self.route.take().ok_or_else(invalid)?;let work=effect.performed_work().checked_sub(start).ok_or_else(invalid)?;Ok(reply.complete(result.map(|step|RetirementStep::Progress(step.progress(),work))))
 }
}
impl<C:EditOwner> RetirementCursor for EditRetirement<C>{
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{if self.route.is_some_and(|(key,_)|key.body!=body){return Err(ValueError::invariant("native edit retirement changed its selected body"))}self.cursor.demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_demand(body)?;self.cursor.work(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.cursor.original_birth()}
 fn requires_original_control(&self)->bool{self.route.is_some_and(|(key,_)|key.original.is_some())||self.cursor.original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{match self.cursor.birth_step(original,grant){crate::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}}
 fn terminal_is_empty(&self)->bool{self.route.is_none()&&self.reply.is_none()&&self.cursor.empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<C:EditOwner> Drop for EditRetirement<C>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.reply)}}else if !std::thread::panicking(){panic!("native edit retirement retains its original owner")}}}
macro_rules! edit_owner{($cursor:ident)=>{
 impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> EditOwner for $cursor<K,V>{
  fn demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{self.close_demands(body)}
  fn work(&self,body:usize)->Result<u64,ValueError>{self.close_work_demand(body)}
  fn close<'r>(&mut self,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(body,original,effect,reply,recover)}
  fn original_birth(&self)->bool{self.requires_original_birth()}
  fn original_control(&self)->bool{self.requires_original_control()}
  fn birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.close_original_birth_step(original,grant)}
  fn empty(&self)->bool{self.terminal_is_empty()}
 }
 impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> RetireOwned for $cursor<K,V>{
  type RetirementCursor=EditRetirement<Self>;
  fn retirement_seed(self)->Self::RetirementCursor{EditRetirement{cursor:ManuallyDrop::new(self),reply:ManuallyDrop::new(None),route:None}}
  fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
  fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
 }
};}
edit_owner!(RetainedOrderedMapInsertCursor);
edit_owner!(RetainedOrderedMapRemoveCursor);
