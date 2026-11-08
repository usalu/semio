//! ♻️ Original slot retirement retains actual borrowed replies before its next paid transition.
use super::*;
use semio_framework_value_retirement::{RetireOwned,retirement::{RetirementCursor,RetirementStep}};
use semio_framework_value_retained_clone::RetainedCloneClose;
use crate::{FactoryAuthority,ErasedSnapshotRetirement};
use semio_framework_value_retirement::tickets::FactoryTicketClose;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Route{Read,Ticket,Intrinsic,Factory}
#[doc(hidden)]
pub struct OriginalSlotRetirement<T:RetireOwned>{
 slot:ManuallyDrop<Option<RetainedOriginalSlot<T>>>,payload:ManuallyDrop<Option<T>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
 active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,ticket:FactoryTicketClose,
 intrinsic:RetainedCloneClose,factory_close:Option<FactoryAuthority>,
 reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,
 route:Option<(OriginalEffectInvocationIdentity,Route,bool)>,
 selected:Option<(RetirementDemand,u64,usize,RetainedCloneGrant)>,
}
impl<T:RetireOwned> OriginalSlotRetirement<T>{
 fn bridge_bytes()->usize{2*size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>()+size_of::<RetirementStep>()}
 fn child_body(body:usize)->usize{body.saturating_sub(Self::bridge_bytes())}
 fn action(&self)->Option<Route>{
  if let Some((_,route,_))=self.route{return Some(route)}
  if self.slot.as_ref().is_some_and(RetainedOriginalSlot::has_read){return Some(Route::Read)}
  if self.active.is_some(){return Some(Route::Ticket)}
  if !self.intrinsic.is_empty(){return Some(Route::Intrinsic)}
  if self.factory_close.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){return Some(Route::Factory)}
  None
 }
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some((d,_,_,_))=self.selected{return Ok(d)}
  let mut d=if let Some(route)=self.action(){let child=Self::child_body(body);let mut d=match route{Route::Read=>{let slot=self.slot.as_ref().expect("selected original slot");slot.return_demands(slot.key(),child)?},Route::Ticket=>self.ticket.next_demand(&self.active,child)?,Route::Intrinsic=>self.intrinsic.next_owner_demands::<T>(false,&None,child)?,Route::Factory=>self.factory_close.as_ref().expect("selected exact factory").demands(child)?};d.copy_bytes=d.copy_bytes.checked_add(Self::bridge_bytes()).ok_or_else(||fault("slot retirement reply extent overflow"))?;d
  }else if self.slot.is_some(){RetirementDemand{copy_bytes:size_of::<Option<RetainedOriginalSlot<T>>>()+2*size_of::<Option<T>>(),depth:1,..Default::default()}}
  else if self.payload.is_some(){if self.factory.is_some(){crate::FactoryOwnedRetirement::<T>::original_admission_demands(&self.payload)}else{self.intrinsic.next_owner_demands::<T>(true,&None,body)?}}
  else if self.factory.is_some(){RetirementDemand{copy_bytes:size_of::<Option<FactoryAuthority>>()+size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>(),depth:1,..Default::default()}}
  else if self.factory_close.is_some(){RetirementDemand{copy_bytes:size_of::<Option<FactoryAuthority>>(),depth:1,..Default::default()}}
  else{Default::default()};if d.depth>0{d.depth=d.depth.checked_add(1).ok_or_else(||fault("slot retirement depth overflow"))?}Ok(d)
 }
 fn work(&self,body:usize)->Result<u64,ValueError>{
  if let Some((_,work,_,_))=self.selected{return Ok(work)}
  if let Some(route)=self.action(){let child=Self::child_body(body);let work=match route{Route::Read=>{let slot=self.slot.as_ref().expect("selected original slot");slot.return_work_demand(slot.key(),child)?},Route::Ticket=>self.ticket.next_work(&self.active,child)?,Route::Intrinsic=>self.intrinsic.next_owner_work::<T>(false,&None,child)?,Route::Factory=>self.factory_close.as_ref().expect("selected exact factory").next_computational_work(child)?};return work.checked_add(1).ok_or_else(||fault("slot retirement reply Work overflow"))}
  if self.payload.is_some()&&self.factory.is_none(){return self.intrinsic.next_owner_work::<T>(true,&None,body)}Ok(u64::from(!self.empty()))
 }
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  let boundary=effect.performed_boundary();let grant=effect.grant();
  if self.route.is_none(){
   if recover{return Err(fault("slot retirement recovery has no original child"))}
   let d=self.demand(grant.maximum_copy_bytes)?;let work=self.work(grant.maximum_copy_bytes)?;
   if !retained_clone_grant_admits(grant,d)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
   if self.empty(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
   if let Some(route)=self.action(){let invocation=effect.invocation().ok_or_else(||fault("slot retirement lacks its original invocation"))?;self.route=Some((invocation.identity(),route,original.is_some()));self.selected=Some((d,work,Self::child_body(grant.maximum_copy_bytes),grant));
   }else{
    if self.slot.is_some(){if !effect.performed(1)?{return Ok(reply.complete(Err(fault("slot retirement extraction exceeded Work").into())))}let slot=self.slot.take().expect("funded original slot");if !slot.is_empty(){*self.payload=Some(slot.try_into_original().unwrap_or_else(|_|panic!("quoted slot has no immutable readers")))}return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},effect.performed_since(boundary)?))))}
    if self.payload.is_some(){
     if self.factory.is_some(){let mut child=None;drop(crate::FactoryOwnedRetirement::<T>::admit_original_into(&mut self.payload,&mut self.factory,&mut self.active,effect,OriginalEffectReply::borrow(&mut child)?)?);return Ok(reply.complete(child.take().ok_or_else(||fault("native factory admission omitted its reply"))?.map(|step|RetirementStep::Progress(step.progress(),effect.performed_since(boundary).expect("same actual Work")))))}
     let turn=self.intrinsic.begin_granted(&mut self.payload,effect);return Ok(reply.complete(turn.result.map(|step|RetirementStep::Progress(step.unwrap_or(RetainedCloneStep::Progress(Default::default())).progress(),effect.performed_since(boundary).expect("same actual Work"))).map_err(Into::into)))
    }
    if !effect.performed(1)?{return Ok(reply.complete(Err(fault("slot issuer metadata exceeded Work").into())))}if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));}else{self.factory_close=None}
    return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},effect.performed_since(boundary)?))))
   }
  }
  let(identity,route,with_original)=self.route.ok_or_else(||fault("slot retirement child route absent"))?;
  let invocation=effect.invocation().ok_or_else(||fault("slot retirement child invocation absent"))?;
  if invocation.identity()!=identity||with_original!=original.is_some(){return Err(fault("slot retirement changed its actual invocation or original route"))}
  let(d,_,body,original_grant)=self.selected.ok_or_else(||fault("slot retirement selected quote absent"))?;
  if grant!=original_grant{return Err(fault("slot retirement changed its frozen original grant"))}
  if self.demand(grant.maximum_copy_bytes)?!=d{return Err(fault("slot retirement physical quote changed"))}
  if self.reply.is_none(){
   let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.checked_sub(Self::bridge_bytes()).ok_or_else(||fault("slot reply copy reservation absent"))?,maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(||fault("slot child depth absent"))?,..grant};
   let remaining=effect.remaining_work_units()?.checked_sub(1).ok_or_else(||fault("slot reply Work reservation absent"))?;let mut child=effect.reborrow(child_grant,remaining)?;
   let target=OriginalEffectReply::borrow(&mut self.reply)?;
   let done=match route{
    Route::Read=>{let slot=self.slot.as_mut().expect("original slot remains resident");let key=slot.key();match original.as_mut(){Some(original)=>if recover{slot.recover_return_read_original_into(key,&mut self.factory,original,&mut child,target)?}else{slot.return_read_original_into(key,&mut self.factory,original,&mut child,target)?},None=>if recover{slot.recover_return_read_into(key,&mut self.factory,&mut child,target)?}else{slot.return_read_into(key,&mut self.factory,&mut child,target)?}}},
    Route::Ticket=>match original.as_mut(){Some(original)=>if recover{self.ticket.recover_close_original_into(&mut self.active,original,body,&mut child,target)?}else{self.ticket.close_original_into(&mut self.active,original,body,&mut child,target)?},None=>if recover{self.ticket.recover_close_into(&mut self.active,body,&mut child,target)?}else{self.ticket.close_into(&mut self.active,body,&mut child,target)?}},
    Route::Intrinsic=>match original.as_mut(){Some(original)=>if recover{self.intrinsic.recover_step_original_into(original,body,&mut child,target)?}else{self.intrinsic.step_original_into(original,body,&mut child,target)?},None=>if recover{self.intrinsic.recover_step_into(body,&mut child,target)?}else{self.intrinsic.step_into(body,&mut child,target)?}},
    Route::Factory=>{let factory=self.factory_close.as_mut().expect("actual factory remains resident");match original.as_mut(){Some(original)=>if recover{factory.recover_step_original_into(original,body,&mut child,target)?}else{factory.step_original_into(original,body,&mut child,target)?},None=>if recover{factory.recover_step_into(body,&mut child,target)?}else{factory.step_into(body,&mut child,target)?}}},
   };drop(done);let _=body;
  }
  if self.reply.is_none(){return Ok(reply.pending())}
  if !effect.performed(1)?{return Err(fault("slot retirement reply publication exceeded original Work"))}
  let work=effect.performed_since(boundary)?;let result=self.reply.take().expect("actual resident slot child reply");self.selected=None;let mut scope=reply.scope(&mut self.route,(identity,route,with_original))?;
  drop(scope.borrow()?.complete(result.map(|step|{let mut progress=step.progress();progress.copied_bytes=progress.copied_bytes.checked_add(Self::bridge_bytes()).expect("quoted slot reply extent");RetirementStep::Progress(progress,work)})));Ok(scope.forwarded())
 }
 fn empty(&self)->bool{self.slot.is_none()&&self.payload.is_none()&&self.factory.is_none()&&self.active.is_none()&&self.ticket.is_empty()&&self.intrinsic.is_empty()&&self.factory_close.is_none()&&self.reply.is_none()&&self.route.is_none()&&self.selected.is_none()}
}
impl<T:RetireOwned> RetireOwned for RetainedOriginalSlot<T>{type RetirementCursor=OriginalSlotRetirement<T>;fn retirement_seed(self)->Self::RetirementCursor{OriginalSlotRetirement{slot:ManuallyDrop::new(Some(self)),payload:ManuallyDrop::new(None),factory:ManuallyDrop::new(None),active:ManuallyDrop::new(None),ticket:Default::default(),intrinsic:Default::default(),factory_close:None,reply:ManuallyDrop::new(None),route:None,selected:None}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalSlotRetirement<T>>())}fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}}
impl<T:RetireOwned> RetirementCursor for OriginalSlotRetirement<T>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.work(body)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
 fn requires_original_birth(&self)->bool{if self.route.is_some(){return false}match self.action(){Some(Route::Read)=>self.slot.as_ref().is_some_and(RetainedOriginalSlot::read_requires_original_birth),Some(Route::Ticket)=>self.active.as_ref().is_some_and(|owner|owner.requires_original_birth()),Some(Route::Intrinsic)=>self.intrinsic.requires_original_birth(),Some(Route::Factory)=>self.factory_close.as_ref().is_some_and(FactoryAuthority::requires_original_birth),None=>false}}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{
  if grant.maximum_depth==0{return RetirementStep::BudgetExhausted}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  let poll=match self.action(){Some(Route::Read)=>self.slot.as_mut().expect("selected read birth").read_original_birth_step(original,child),Some(Route::Ticket)=>self.active.as_mut().expect("selected ticket birth").close_original_birth_step(original,child),Some(Route::Intrinsic)=>self.intrinsic.step_original_birth(original,child),Some(Route::Factory)=>self.factory_close.as_mut().expect("selected exact issuer birth").close_original_birth_step(original,child),None=>return RetirementStep::Failure(fault("slot has no selected original birth").into(),0)};
  match poll{crate::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 }
 fn requires_original_control(&self)->bool{if let Some((_,_,mode))=self.route{return mode}match self.action(){Some(Route::Read)=>self.slot.as_ref().is_some_and(RetainedOriginalSlot::read_requires_original_control),Some(Route::Ticket)=>self.active.as_ref().is_some_and(|owner|owner.requires_original_control()),Some(Route::Intrinsic)=>self.intrinsic.requires_original_control(),Some(Route::Factory)=>self.factory_close.as_ref().is_some_and(FactoryAuthority::requires_original_control),None=>false}}
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Drop for OriginalSlotRetirement<T>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.empty(),"original slot retirement retains source, payload, issuer or reply");unsafe{ManuallyDrop::drop(&mut self.slot);ManuallyDrop::drop(&mut self.payload);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.reply)}}}
