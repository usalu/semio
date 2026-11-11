//! 🔢️ Persisted binary operations become typed native projections through the vocabulary's own receiving cursor, never through a cold decode; the installer names this encoding explicitly.
use super::{ArtifactOriginalOperationReceiving,OriginalOperationReceivingAdmission};
use protocol::{OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneStep,RetainedOwnedProjection,retained_clone_grant_admits}};
use semio_framework_pack_json::{retained_alias,retained_alias_demand};
use semio_framework_value_native_owned::{NativeAllocationIdentity,native_decoding::{NativeDecodeControl,NativeReceivingFailure,OriginalReceivingPrimary}};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
use crate::native_source::{pay,polled};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Admit,Receive,Take,CloseCursor,Authority,TypedSource,TypedProjection,Ready}

/// 🔢️ Retains the original binary wire, the receiving cursor and the typed owners until funded closure.
pub struct OriginalBinaryReceiving<M:OpBinaryReceiving+Sync>{
 ledger:NativeAllocationIdentity,recipient:Option<u64>,wire:ManuallyDrop<Option<RetainedOwnedProjection<Vec<u8>>>>,
 cursor:ManuallyDrop<Option<M::Receiving>>,typed:ManuallyDrop<Option<M>>,authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,
 typed_admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<FactoryOwnedSourceAuthority<M>,FactoryOwnedSourceAuthority<M>>>>,typed_source:ManuallyDrop<Option<RetainedCloneSource<FactoryOwnedSourceAuthority<M>>>>,
 output:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:Option<FactoryAuthority>,
 retirement:RetainedCloneClose,phase:Phase,closing:bool,faulted:bool,taking_owned:bool,
 take_reply:Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,take_pending:bool,
}
fn refused(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(||invalid("original binary native construction refused")).with_retained_progress(progress)}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn close_header<T>()->RetirementDemand{let _=std::marker::PhantomData::<T>;RetirementDemand{depth:1,..Default::default()}}
impl<M:OpBinaryReceiving+Sync> OriginalBinaryReceiving<M>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 pub fn admit(wire:RetainedOwnedProjection<Vec<u8>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<Vec<u8>>)>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err((invalid("original binary receiving shell was not funded"),wire));}
  Ok((Self{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),wire:ManuallyDrop::new(Some(wire)),cursor:ManuallyDrop::new(None),typed:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),typed_admission:ManuallyDrop::new(None),typed_source:ManuallyDrop::new(None),output:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(Arc::clone(factory))),factory_close:None,retirement:Default::default(),phase:Phase::Admit,closing:false,faulted:false,taking_owned:false,take_reply:None,take_pending:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 fn typed_ref(&self)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,M>,ValueError>{Ok(self.typed_source.as_ref().ok_or_else(||invalid("original binary typed source is absent"))?.try_borrow()?.project(0,|authority:&FactoryOwnedSourceAuthority<M>|authority.get()))}
 fn source(&self)->Result<&Vec<u8>,ValueError>{Ok(self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?.get())}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body);}if self.faulted{return Err(invalid("faulted original binary receiving requires cleanup"));}
  Ok(match self.phase{
   Phase::Admit=>RetirementDemand{depth:1,..Default::default()},
   Phase::Receive=>self.cursor.as_ref().ok_or_else(||invalid("original binary cursor is absent"))?.demands(self.source()?,body)?,
   Phase::Take=>RetirementDemand{depth:1,..Default::default()},
   Phase::CloseCursor=>{let cursor=self.cursor.as_ref().ok_or_else(||invalid("original binary cursor is absent"))?;if cursor.terminal_is_empty(){close_header::<Option<M::Receiving>>()}else{cursor.close_demands(body)?}},
   Phase::Authority=>header::<FactoryOwnedSourceAuthority<M>>(),
   Phase::TypedSource=>{let admission=self.typed_admission.as_ref().ok_or_else(||invalid("original binary typed source admission is absent"))?;if admission.phase()=="ready"{header::<Option<RetainedCloneSource<FactoryOwnedSourceAuthority<M>>>>()}else if admission.phase()=="projectionStart"{admission.owned_projection_demand()?}else{admission.next_demand()?}},
   Phase::TypedProjection=>retained_alias_demand(self.typed_ref()?)?,
   Phase::Ready=>header::<Option<RetainedOwnedProjection<M>>>(),
  })
 }
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{let mut work=0;let result=self.advance_turn(control,body,maximum_work_units,g,&mut work);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work})}
 fn advance_turn(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,work:&mut u128)->Result<RetainedCloneStep,ValueError>{
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary receiving changed its decoder ledger"));}
  if self.closing{return Err(invalid("original binary receiving closes through its close step"));}
  let d=self.demands(body)?;if maximum_work_units==0||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  control.checkpoint()?;
  let child=RetainedCloneGrant{maximum_items:1,..g};
  let mut p=if self.phase==Phase::Receive{
   let wire=self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?;let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;let turn=cursor.step(wire.get(),control,body,g);*work=turn.work;let p=turn.result.map_err(refused)?;if cursor.is_ready(){self.phase=Phase::Take;}p
  }else{
   let phase=self.phase;let quote=match phase{Phase::Admit|Phase::TypedSource|Phase::TypedProjection=>maximum_work_units,_=>1};
   let Some((result,spent))=control.with_allocated_effect(d,quote,|_,effect|{
    let result=self.advance_local(phase,d,child,effect);
    let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
    if d.capacity_bytes!=0&&progress.retained_capacity_bytes!=0&&effect.actual().is_none(){*effect.actual()=Some(progress.retained_capacity_bytes);}
    let result=match result{Ok(progress) if effect.performed_work()==0&&progress!=RetainedCloneProgress::default()=>pay(effect,1).map(|()|progress),other=>other};
    (result,effect.performed_work())
   })? else{return Ok(RetainedCloneStep::Progress(Default::default()))};
   *work=spent;result?
  };
  p.copied_items=p.copied_items.max(1);semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original binary receiving")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn advance_local(&mut self,phase:Phase,d:RetirementDemand,child:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  Ok(match phase{
   Phase::Admit=>{let wire=self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?;let bytes=wire.get();let turn=M::admit_receiving(bytes,(0,bytes.len()),child);pay(effect,turn.work)?;match turn.result?{Some((cursor,p))=>{*self.cursor=Some(cursor);self.phase=Phase::Receive;p},None=>Default::default()}},
   Phase::Receive=>return Err(invalid("original binary receive step runs under its own funding")),
   Phase::Take=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;let turn=cursor.take_into(&mut self.typed,child);pay(effect,turn.work)?;let p=turn.result?;if self.typed.is_some(){self.phase=Phase::CloseCursor;}p},
   Phase::CloseCursor=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;if cursor.terminal_is_empty(){if let Some(cursor)=self.cursor.take(){std::mem::forget(cursor);}self.phase=Phase::Authority;unit()}else{polled(cursor.close_step(&mut effect.nested(child)))?}},
   Phase::Authority=>{let authority=FactoryOwnedSourceAuthority::new(self.typed.take().ok_or_else(||invalid("original binary typed value is absent"))?,Arc::clone(self.factory.as_ref().ok_or_else(||invalid("original binary factory is absent"))?));*self.typed_admission=Some(RetainedCloneSource::<FactoryOwnedSourceAuthority<M>>::admission_owned(authority,()));self.phase=Phase::TypedSource;unit()},
   Phase::TypedSource=>{let admission=self.typed_admission.as_mut().ok_or_else(||invalid("original binary typed admission is absent"))?;if admission.phase()=="projectionStart"{admission.project_owned(&mut effect.nested(child)).result?.progress()}else if admission.phase()=="ready"{let step=admission.take_output(&mut self.typed_source,&mut effect.nested(child)).result?;if admission.terminal_is_empty(){self.typed_admission.take();self.phase=Phase::TypedProjection;}step.progress()}else{admission.advance(&mut effect.nested(child)).result?.progress()}},
   Phase::TypedProjection=>{let Some((output,p))=retained_alias(self.typed_ref()?,&mut effect.nested(child))? else{return Ok(Default::default())};*self.output=Some(output);self.phase=Phase::Ready;p},
   Phase::Ready=>Default::default(),
  })
 }
 pub fn ready(&self)->bool{!self.taking_owned&&!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 pub fn owned_handoff_pending(&self)->bool{self.taking_owned&&!self.closing&&!self.faulted}
 pub fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{let result=self.take_ready(g);let work=u128::from(matches!(result,Ok(Some(_))));RetainedWorkOutcome{result,work}}
 fn take_ready(&mut self,g:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(g,d){return Ok(None);}let output=self.output.take().ok_or_else(||invalid("original binary receiving output already transferred"))?;self.begin_close();Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.faulted||self.phase!=Phase::Ready||(!self.taking_owned&&self.output.is_none()){return Err(invalid("original binary owned handoff requires its completed output"))}
  if !self.taking_owned{return Ok(header::<bool>())}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)}}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<FactoryOwnedSourceAuthority<M>>>>())}else{source.next_take_demand(body)}}
  if self.authority.is_some(){let d=FactoryOwnedSourceAuthority::<M>::transfer_original_demand();return Ok(RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Option<M>>(),..d})}
  Err(invalid("original binary owned handoff lost its unique authority"))
 }
 /// ⛽️ Quotes the exact work of the next owned handoff turn, including its unit payment.
 pub fn take_owned_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.take_owned_demands(body)?;
  if !self.taking_owned{return Ok(1)}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(1)}else{output.close_work_demand(body)?.checked_add(1).ok_or_else(||invalid("original binary handoff work overflow"))}}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_take_work_demand(body)?.checked_add(1).ok_or_else(||invalid("original binary handoff work overflow"))}}
  Ok(1)
 }
 pub fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let result=self.take_owned_turn(output,control,body,effect);RetainedWorkOutcome{result,work:effect.performed_work()}}
 fn take_owned_turn(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let g=effect.grant();
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary owned handoff changed its decoder ledger"))}if output.is_some(){return Err(invalid("original binary owned handoff destination is occupied"))}
  let d=self.take_owned_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;pay(effect,1)?;control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.taking_owned{self.taking_owned=true;return Ok(unit())}
  if let Some(projection)=self.output.as_mut(){if projection.terminal_is_empty(){self.output.take();return Ok(unit())}return Ok(RetainedCloneStep::Progress(polled(projection.close_step(g.maximum_copy_bytes,effect))?))}
  if let Some(source)=self.typed_source.as_mut(){
   if source.terminal_is_empty(){self.typed_source.take();return Ok(unit())}
   if self.take_reply.is_none(){let reply=OriginalEffectReply::borrow(&mut self.take_reply)?;let done=if self.take_pending{source.recover_take_authority_into(&mut *self.authority,body,effect,reply)?}else{source.take_authority_into(&mut *self.authority,body,effect,reply)?};drop(done);}
   return match self.take_reply.take(){None=>{self.take_pending=true;Ok(RetainedCloneStep::Progress(Default::default()))},Some(result)=>{self.take_pending=false;Ok(RetainedCloneStep::Progress(result.map_err(|failure|failure.into_value_error())?.progress()))}};
  }
  let mut authority=self.authority.take().ok_or_else(||invalid("original binary owned handoff requires its unique authority"))?;
  let(mut value,mut factory,mut slot)=(None,None,None);
  let transferred=OriginalEffectReply::borrow(&mut slot).and_then(|reply|authority.transfer_original_into(&mut value,&mut factory,effect,reply).map(drop));
  let settled=transferred.and_then(|()|slot.take().ok_or_else(||invalid("original binary pair transfer reply is pending")).and_then(|result|result));
  match(settled,value,factory){
   (Ok(RetainedCloneStep::Complete(mut p)),Some(value),Some(factory))=>{drop(authority);*output=Some(value);self.factory_close=Some(FactoryAuthority::new(factory));self.closing=true;p.copied_bytes+=size_of::<Option<M>>();Ok(RetainedCloneStep::Complete(p))},
   (Ok(step),None,None)=>{*self.authority=Some(authority);Ok(RetainedCloneStep::Progress(step.progress()))},
   (Err(error),..)=>{*self.authority=Some(authority);Err(error)},
   _=>{*self.authority=Some(authority);Err(invalid("original binary pair transfer changed its slots"))},
  }
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<M>(false,&None,body);}
  if let Some(admission)=self.typed_admission.as_ref(){return admission.next_return_demand();}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(close_header::<Option<M::Receiving>>())}else{cursor.close_demands(body)};}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)};}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(close_header::<Option<RetainedCloneSource<FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)};}
  if self.typed.is_some(){return self.retirement.next_owner_demands::<M>(true,&None,body);}
  if self.authority.is_some(){return self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(true,&None,body);}
  if let Some(factory)=self.factory_close.as_ref(){return if factory.terminal_is_empty(){Ok(close_header::<FactoryAuthority>())}else{factory.demands(body)};}if self.factory.is_some(){return Ok(close_header::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>());}Ok(Default::default())
 }
 pub fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let g=effect.grant();
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary cleanup changed its decoder ledger").into());}
   if !self.closing{return Err(invalid("original binary cleanup was not started").into());}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   let d=self.close_demands(g.maximum_copy_bytes)?;
   if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}
   let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
   if !self.retirement.is_empty(){return Ok(RetirementPollExit::resume(self.retirement.step_granted(g.maximum_copy_bytes,effect))?);}
   if let Some(admission)=self.typed_admission.as_mut(){if admission.phase()=="ready"{let turn=admission.take_output(&mut self.typed_source,effect);return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((authority,()))=returned{*self.authority=Some(authority);}if admission.terminal_is_empty(){self.typed_admission.take();}return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
   if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){if let Some(cursor)=self.cursor.take(){std::mem::forget(cursor);}return Ok(unit());}return Ok(RetirementPollExit::resume(cursor.close_step(effect))?);}
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();return Ok(unit());}return Ok(RetirementPollExit::resume(owner.close_step(g.maximum_copy_bytes,effect))?);}};}
   close!(output);close!(typed_source);close!(wire);
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);if let Some(step)=turn.result?{return Ok(step);}}};}
   owned!(typed);owned!(authority);
   if self.factory_close.is_none(){if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return Ok(unit());}}
   if let Some(factory)=self.factory_close.as_mut(){
    if factory.terminal_is_empty(){self.factory_close=None;return Ok(unit());}
    return Ok(RetirementPollExit::resume(factory.step(g.maximum_copy_bytes,effect))?);
   }
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  RetirementPollExit::settle(effect,result)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.wire.is_none()&&self.cursor.is_none()&&self.typed.is_none()&&self.authority.is_none()&&self.typed_admission.is_none()&&self.typed_source.is_none()&&self.output.is_none()&&self.factory.is_none()&&self.factory_close.is_none()&&self.retirement.is_empty()&&!self.take_pending}
}
impl<M:OpBinaryReceiving+Sync> Drop for OriginalBinaryReceiving<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original binary receiving requires funded closure of all source and typed owners");}}
impl<M:OpBinaryReceiving+Sync+Send+'static> ArtifactOriginalOperationReceiving<M> for OriginalBinaryReceiving<M> where M::Receiving:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{self.advance(control,body,maximum_work_units,grant)}
 fn ready(&self)->bool{self.ready()}
 fn owned_handoff_pending(&self)->bool{self.owned_handoff_pending()}
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{self.take(grant)}
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.take_owned_demands(body)}
 fn take_owned_work_demand(&self,body:usize,_ceiling:u64)->Result<u64,ValueError>{self.take_owned_work_demand(body)}
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.take_owned_into(output,control,body,effect)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close_step(control,effect)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
fn demands<M:OpBinaryReceiving+Sync>()->RetirementDemand{let d=OriginalBinaryReceiving::<M>::constructor_demand();RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Box<dyn ArtifactOriginalOperationReceiving<M>>>(),capacity_bytes:size_of::<OriginalBinaryReceiving<M>>(),..d}}
fn admit<M:OpBinaryReceiving+Sync+Send+'static>(wire:&mut Option<RetainedOwnedProjection<Vec<u8>>>,output:&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> where M::Receiving:Send{
 if output.is_some()||wire.is_none(){return Err(invalid("original binary receiver requires its retained wire and empty output"))}let d=demands::<M>();if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;
 match OriginalBinaryReceiving::admit(wire.take().ok_or_else(||invalid("original binary receiver lost its wire"))?,factory,control,grant){Ok((receiver,_))=>{*output=Some(Box::new(receiver));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})},Err((error,original))=>{*wire=Some(original);Err(error)}}
}
impl<M:OpBinaryReceiving+Sync+Send+'static> OriginalOperationReceivingAdmission<M> where M::Receiving:Send{
 /// 🔢️ The admission of a vocabulary whose persisted operations are binary-encoded.
 pub fn binary(batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer<M>)->Self{Self{batch_retirement,demands:demands::<M>,admit:admit::<M>}}
}
