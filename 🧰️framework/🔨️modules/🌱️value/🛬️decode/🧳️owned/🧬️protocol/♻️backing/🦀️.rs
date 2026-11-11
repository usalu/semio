//! 🧳️ Receiver backing retains its original native child reply and paid dispatch headers through unwind.
use crate::{NativeReceiver,OriginalEffectInvocationIdentity,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalWorkBindingIdentity,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use semio_framework_value_native_storage::NativeOwnedStorage;
use std::mem::{ManuallyDrop,size_of};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
#[derive(Clone,Copy,PartialEq,Eq)]
struct CloseLoanBinding{ledger:semio_framework_value_control_contract::NativeAllocationIdentity,recipient:Option<u64>,work:OriginalWorkBindingIdentity}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Selection{invocation:Option<OriginalEffectInvocationIdentity>,grant:RetainedCloneGrant,body:usize,address:Option<usize>,retired:Option<usize>,original:bool,loan:Option<CloseLoanBinding>,code:u8,demand:RetirementDemand,child:RetirementDemand,work:u64,child_work:u64,prefix:usize}
/// 🧷️ Holds only the original backing action, Work association, and its actual resident child refusal.
pub struct NativeReceiverBackingClose{selected:Option<Selection>,answer:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,work:Option<OriginalWorkBindingIdentity>}
impl Default for NativeReceiverBackingClose{fn default()->Self{Self::vacant()}}
fn invalid()->ValueError{ValueError::invariant("native receiver backing changed its original owner, invocation or Work scope")}
fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native receiver header lacks an admitted real terminal destructor")}
impl NativeReceiverBackingClose{
 /// 🌱️ Creates the strict vacant inline backing dispatcher without issuing authority.
 pub const fn vacant()->Self{Self{selected:None,answer:ManuallyDrop::new(None),work:None}}
 /// 🔍️ Reports whether this dispatcher retains any original action or Work association.
 pub fn terminal_is_empty(&self)->bool{self.selected.is_none()&&self.answer.is_none()&&self.work.is_none()}
 /// 🧭️ Reports the exact selected original-loan route while its action remains live.
 pub fn original_route(&self)->Option<bool>{self.selected.map(|selected|selected.original)}
 fn metadata()->usize{size_of::<Option<Selection>>()+size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>()+size_of::<Option<OriginalWorkBindingIdentity>>()}
 fn retired_address<C>(retired:&Option<Box<C>>)->Option<usize>{retired.as_ref().map(|owner|std::ptr::from_ref(&**owner)as usize)}
 fn quote<I:?Sized,T,C:NativeReceiver<I,T>>(&self,storage:&NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize,original:bool)->Result<Selection,ValueError>{
  if let Some(selected)=self.selected{return if selected.body==body{Ok(selected)}else{Err(ValueError::invariant("native receiver backing body changed"))}}let prefix=Self::metadata();let child_body=body.saturating_sub(prefix);
  let(code,child,child_work)=if let Some(owner)=retired.as_ref(){if storage.reserved(){return Err(invalid())}let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}(4,release.boxed_demand(),1)}
  else if let Some(owner)=storage.get(){if !owner.terminal_is_empty(){if owner.is_closing(){(2,owner.close_demands(child_body)?,owner.close_work_demand(child_body)?)}else{(1,owner.begin_close_demand()?,owner.begin_close_work_demand()?)}}
   else{let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}(3,NativeOwnedStorage::<C>::box_demand(),storage.box_work_demand())}}
  else if storage.reserved(){(5,storage.release_demand(),1)}
  else if self.work.is_some(){(6,RetirementDemand::default(),0)}
  else{(0,RetirementDemand::default(),0)};
  let demand=if code==0{Default::default()}else{RetirementDemand{copy_bytes:child.copy_bytes.checked_add(prefix).ok_or_else(invalid)?,depth:child.depth.checked_add(1).ok_or_else(invalid)?.max(1),..child}};let work=child_work.checked_add(u64::from(code!=0)).ok_or_else(invalid)?;
  Ok(Selection{invocation:None,grant:RetainedCloneGrant::default(),body,address:storage.address(),retired:Self::retired_address(retired),original,loan:None,code,demand,child,work,child_work,prefix})
 }
 /// 📐️ Includes every actual routing, original reply, Work binding and backing header extent.
 pub fn demands<I:?Sized,T,C:NativeReceiver<I,T>>(&self,storage:&NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.quote::<I,T,C>(storage,retired,body,false)?.demand)}
 /// ⏱️ Quotes one native dispatch write and the exact selected child Work.
 pub fn work_demand<I:?Sized,T,C:NativeReceiver<I,T>>(&self,storage:&NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize)->Result<u64,ValueError>{Ok(self.quote::<I,T,C>(storage,retired,body,false)?.work)}
 fn finish<'r>(&mut self,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{let Some(answer)=self.answer.as_ref()else{return Ok(reply.pending())};let selected=self.selected.ok_or_else(invalid)?;let progress=match answer{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let mut progress=progress.checked_add(RetainedCloneProgress{copied_bytes:selected.prefix,..Default::default()}).map_err(|_|invalid())?;progress.copied_items=progress.copied_items.max(1);let answer=self.answer.take().ok_or_else(invalid)?;self.selected=None;self.work=None;let terminal=self.terminal_is_empty();Ok(reply.complete(match answer{Ok(_)=>Ok(if terminal{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),Err(error)=>Err(error.with_retained_progress(progress))}))}
 fn drive<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  let invocation=effect.invocation().ok_or_else(invalid)?.identity();let loan=match original.as_deref_mut(){None=>None,Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(invalid())}Some(CloseLoanBinding{ledger:original.ledger_identity(),recipient:original.retirement_recipient_identity(),work})}};if self.work.is_some_and(|work|!work.matches_effect(effect)){return Err(ValueError::invariant("native receiver backing changed original Work scope"))}
  let selected=if let Some(selected)=self.selected{if !recover{return Err(ValueError::invariant("native receiver backing requires recovery of its resident action"))}if selected.invocation!=Some(invocation){return Err(ValueError::invariant("native receiver backing changed original invocation"))}if selected.grant!=effect.grant(){return Err(ValueError::invariant("native receiver backing changed original grant"))}if selected.body!=body||selected.original!=original.is_some()||selected.loan!=loan{return Err(ValueError::invariant("native receiver backing changed original close request"))}if selected.address!=storage.address()||selected.retired!=Self::retired_address(retired){return Err(ValueError::invariant("native receiver backing replaced its physical owner"))}selected}
  else{if recover{return Err(invalid())}let mut selected=self.quote::<I,T,C>(storage,retired,body,original.is_some())?;if selected.code==0{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if !retained_clone_grant_admits(effect.grant(),selected.demand)||effect.remaining_work_units()?<selected.work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"receiver backing dispatch exceeded original Work"))}self.work=Some(OriginalWorkBindingIdentity::from_effect(effect));selected.invocation=Some(invocation);selected.grant=effect.grant();selected.loan=loan;self.selected=Some(selected);selected};
  if self.answer.is_none(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:selected.child.copy_bytes,maximum_capacity_bytes:selected.child.capacity_bytes,maximum_release_bytes:selected.child.release_bytes,maximum_depth:selected.child.depth};let mut child=effect.reborrow(grant,selected.child_work.min(effect.remaining_work_units()?))?;
   match selected.code{
    1=>{let owner=storage.get_mut().ok_or_else(invalid)?;let inner=OriginalEffectReply::borrow(&mut self.answer)?;if recover{owner.recover_begin_close_into(&mut child,inner)?;}else{owner.begin_close_into(&mut child,inner)?;}},
    2=>{let owner=storage.get_mut().ok_or_else(invalid)?;let inner=OriginalEffectReply::borrow(&mut self.answer)?;match(original.as_deref_mut(),recover){(Some(original),false)=>{owner.close_original_step(original,selected.body.saturating_sub(selected.prefix),&mut child,inner)?;},(Some(original),true)=>{owner.recover_close_original_step(original,selected.body.saturating_sub(selected.prefix),&mut child,inner)?;},(None,false)=>{owner.close_step(selected.body.saturating_sub(selected.prefix),&mut child,inner)?;},(None,true)=>{owner.recover_close_step(selected.body.saturating_sub(selected.prefix),&mut child,inner)?;}}},
    3=>{let owner=storage.get().ok_or_else(invalid)?;let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}let turn=storage.take_box_into(retired,&mut child);match turn{RetainedWorkPoll::Pending{..}=>{},RetainedWorkPoll::Complete(turn)=>{*self.answer=Some(turn.result.map_err(Into::into));}}},
    4=>{let owner=retired.as_ref().ok_or_else(invalid)?;let release=owner.native_terminal_release().ok_or_else(unsupported)?;if !release.validates(owner){return Err(unsupported())}let result=match child.performed(1){Ok(true)=>{drop(retired.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:selected.child.copy_bytes,released_bytes:selected.child.release_bytes,..Default::default()}))},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"receiver header release exceeded original Work").into()),Err(error)=>Err(error.into())};*self.answer=Some(result);},
    5=>{let turn=storage.release(&mut child);*self.answer=Some(turn.result.map_err(Into::into));},
    6=>{self.work=None;*self.answer=Some(Ok(RetainedCloneStep::Complete(Default::default())));},
    _=>return Err(invalid())
   }
  }self.finish(reply)
 }
 /// 📬️ Starts only this backing owner's genuine original borrowed action.
 pub fn close_into<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive::<I,T,C>(storage,retired,body,None,effect,reply,false)}
 /// 🔁️ Reenters the same child only if it has not already recorded its original reply.
 pub fn recover_close_into<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive::<I,T,C>(storage,retired,body,None,effect,reply,true)}
 /// 🪢️ Lends the actual original control loan to the already selected native child.
 pub fn close_original_into<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive::<I,T,C>(storage,retired,body,Some(original),effect,reply,false)}
 /// 🛟️ Recovers the same original control path and retained child reply.
 pub fn recover_close_original_into<'r,I:?Sized,T,C:NativeReceiver<I,T>>(&mut self,storage:&mut NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,body:usize,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive::<I,T,C>(storage,retired,body,Some(original),effect,reply,true)}
}
impl Drop for NativeReceiverBackingClose{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native receiver backing requires its original paid terminal cleanup");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.answer)}}}}
