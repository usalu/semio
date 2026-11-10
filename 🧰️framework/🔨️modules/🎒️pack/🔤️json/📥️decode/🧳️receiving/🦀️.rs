//! 🧳️ Original JSON operation bytes become typed native projections without a cold conversion.
use crate::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor,JsonGrammarCursor,JsonMemberPolicy,JsonError};
use semio_framework_value::{DslValue, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, ArtifactOwnedValueRetirementFactory, FactoryAuthority, retained_clone::{RetainedCloneSource, RetainedCloneSourceAdmission, RetainedOwnedProjection, RetainedOwnedProjectionAdmission, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceivingFailure, native_decoding::OriginalReceivingPrimary,};
use semio_framework_value::retained_clone::{OriginalEffectAction,OriginalEffectReply,OriginalProjectionFrame};
use semio_framework_value_native_owned::owned::{NativeReceivingChild, NativeLocalAuthority, NativeLocalProducer, NativeLocalOperation};
use semio_framework_value_native_owned::NativeAllocationIdentity;
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Phase{Header,Parse,Source,Shell,Receive,TakeReceived,CloseReceived,Authority,TypedSource,TypedProjection,CanonicalProjection,CanonicalBirth,Canonical,Ready}

pub struct OriginalJsonReceiving<M:NativeReceive<DslValue>+Sync>{
 ledger:NativeAllocationIdentity,recipient:Option<u64>,header:&'static[u8],pub wire:ManuallyDrop<Option<RetainedOwnedProjection<Vec<u8>>>>,
 parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,parsed:ManuallyDrop<Option<DslValue>>,
 parsed_source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,source_admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,typed_admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M,FactoryOwnedSourceAuthority<M>>>>,receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,M>>>,
 typed:ManuallyDrop<Option<M>>,authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,typed_source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,typed_alias:ManuallyDrop<Option<RetainedOwnedProjectionAdmission<M>>>,canonical_alias:ManuallyDrop<Option<RetainedOwnedProjectionAdmission<dyn ArtifactCanonicalJsonTree>>>,typed_frame:ManuallyDrop<Option<OriginalProjectionFrame<M,FactoryOwnedSourceAuthority<M>>>>,frame_action:ManuallyDrop<Option<OriginalEffectAction<RetainedCloneStep,ValueError>>>,
 pub output:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,canonical_projection:ManuallyDrop<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>,canonical:ManuallyDrop<Option<ArtifactCanonicalJsonTreeCursor>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:Option<FactoryAuthority>,
 error:ManuallyDrop<Option<JsonError>>,action:NativeReceivingChild<ValueError>,local_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,retirement:RetainedCloneClose,source:for<'a>fn(&'a M)->&'a dyn ArtifactCanonicalJsonTree,offset:usize,pub phase:Phase,closing:bool,faulted:bool,taking_owned:bool,
}
fn refused(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(||invalid("original JSON native construction refused")).with_retained_progress(progress)}
fn settled<V>(poll:semio_framework_value::OriginalRetirementPoll<V>)->Result<(V,u128),ValueError>{match poll{
 semio_framework_value::RetainedWorkPoll::Pending{..}=>Err(invalid("original JSON child close left its producer reply pending inside the step")),
 semio_framework_value::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>Ok((step,turn.work)),Err(failure)=>{let progress=failure.retained_progress();let(primary,work,receipt,..)=failure.into_parts();Err(primary.or(work).or(receipt).unwrap_or_else(||invalid("original JSON child close refused")).with_retained_progress(progress))}}}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn close_header<T>()->RetirementDemand{let _=std::marker::PhantomData::<T>;RetirementDemand{depth:1,..Default::default()}}
fn hop(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->RetainedWorkPoll<RetainedCloneStep,ValueError>{
 let started=effect.performed_work();let result=run(effect).map(|step|RetainedCloneStep::Progress(step.progress()));
 RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work().saturating_sub(started)})
}
fn settle<T>(answer:Result<Option<Result<T,ValueError>>,ValueError>)->Result<T,ValueError>{answer?.unwrap_or_else(||Err(invalid("alias reply pending")))}
fn alias_bound<T:?Sized+Sync>(admission:&Option<RetainedOwnedProjectionAdmission<T>>)->bool{admission.as_ref().is_some_and(|admission|admission.borrow().is_ok())}
fn alias_quote<T:?Sized+Sync>(admission:&Option<RetainedOwnedProjectionAdmission<T>>,child:semio_framework_value::retained_clone::RetainedCloneRef<'_,T>)->Result<(RetirementDemand,u64),ValueError>{
 match admission.as_ref().filter(|_|alias_bound(admission)){Some(admission)=>Ok((admission.take_demand()?,admission.take_work_demand()?)),None=>{let fresh=RetainedOwnedProjectionAdmission::<T>::default();let mut demand=fresh.demand(child)?;demand.depth=demand.depth.max(1);Ok((demand,fresh.work_demand(child)?))}}
}
/// 🔌️ Runs one reply-shaped producer into a private slot and settles it as a synchronous turn.
fn relay<T,F>(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,produce:impl for<'r> FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>,OriginalEffectReply<'r,T,F>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,T,F>,ValueError>)->Result<Option<Result<T,F>>,ValueError>{
 let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;drop(produce(effect,reply)?);}Ok(slot.take())
}
/// ♻️ Starts or advances one native receiver close under the borrowed original effect.
pub fn close_native_receiver<M:NativeReceive<DslValue>+Sync>(receiver:&mut NativeReceivingOriginal<DslValue,M>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>{
 use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
 let started=effect.performed_work();
 let answered=if receiver.is_closing(){relay(effect,|effect,reply|RetirementCursor::close_into(receiver,body,effect,reply)).map(|answer|answer.map(|result|result.map(|step|match step{RetirementStep::Progress(progress,_)|RetirementStep::ChildProgress(_,progress,_)=>RetainedCloneStep::Progress(progress),_=>RetainedCloneStep::Progress(Default::default())})))}else{relay(effect,|effect,reply|receiver.begin_close_into(effect,reply))};
 let work=effect.performed_work().saturating_sub(started);
 match answered{Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work}),Ok(None)=>RetainedWorkPoll::Pending{work},Ok(Some(Ok(step)))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work}),Ok(Some(Err(failure)))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(failure_error(failure)),work})}
}
fn failure_error(failure:semio_framework_value::OriginalRetirementFailure)->ValueError{let progress=failure.retained_progress();let(primary,work,receipt,..)=failure.into_parts();primary.or(work).or(receipt).unwrap_or_else(||invalid("original JSON receiver close refused")).with_retained_progress(progress)}
/// 📤️ Hands the typed authority back from its source through the borrowed original reply.
fn take_authority<M:Send+Sync+'static>(source:&mut RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>,authority:&mut Option<FactoryOwnedSourceAuthority<M>>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
 match relay(effect,|effect,reply|source.take_authority_into(authority,body,effect,reply))?{Some(Ok(step))=>Ok(step),Some(Err(failure))=>Err(failure_error(failure)),None=>Err(invalid("original JSON source handback left its reply pending"))}
}
/// 📤️ Deposits the typed value and its issuer into the caller's slots through the borrowed original reply.
fn transfer_original<M:Send+Sync+'static>(authority:&mut FactoryOwnedSourceAuthority<M>,value:&mut Option<M>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
 match relay(effect,|effect,reply|authority.transfer_original_into(value,factory,effect,reply))?{Some(result)=>result,None=>Err(invalid("original JSON authority transfer left its reply pending"))}
}
fn phase(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(size_of::<Phase>()).ok_or_else(||invalid("original JSON receiving extent overflow"))?;Ok(d)}
impl<M:NativeReceive<DslValue>+Sync> OriginalJsonReceiving<M>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 fn canonical_child(&self)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,dyn ArtifactCanonicalJsonTree>,ValueError>{Ok(self.output.as_ref().ok_or_else(||invalid("original JSON typed projection is absent"))?.borrow()?.project(0,self.source))}
 pub fn admit(wire:RetainedOwnedProjection<Vec<u8>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,header:&'static[u8],source:for<'a>fn(&'a M)->&'a dyn ArtifactCanonicalJsonTree,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<Vec<u8>>)>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err((invalid("original JSON receiving shell was not funded"),wire));}
  Ok((Self{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),header,wire:ManuallyDrop::new(Some(wire)),parser:ManuallyDrop::new(Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject))),parsed:ManuallyDrop::new(None),parsed_source:ManuallyDrop::new(None),source_admission:ManuallyDrop::new(None),typed_admission:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),typed:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),typed_source:ManuallyDrop::new(None),typed_alias:ManuallyDrop::new(None),canonical_alias:ManuallyDrop::new(None),typed_frame:ManuallyDrop::new(None),frame_action:ManuallyDrop::new(None),output:ManuallyDrop::new(None),canonical_projection:ManuallyDrop::new(None),canonical:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(Arc::clone(factory))),factory_close:None,error:ManuallyDrop::new(None),action:Default::default(),local_authority:ManuallyDrop::new(Some(semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority::new(NativeLocalProducer::new()))),retirement:Default::default(),source,offset:header.len(),phase:Phase::Header,closing:false,faulted:false,taking_owned:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body);}if self.faulted{return Err(invalid("faulted original JSON receiving requires cleanup"));}
  phase(match self.phase{
   Phase::Header=>RetirementDemand{copy_bytes:self.header.len(),depth:1,..Default::default()},
   Phase::Parse=>{let input=self.wire.as_ref().ok_or_else(||invalid("original JSON wire is absent"))?.borrow()?;let mut d=self.parser.as_ref().unwrap().normal_step_demands(input.get().get(self.header.len()..).ok_or_else(||invalid("original JSON wire header is truncated"))?).map_err(|error|ValueError::literal(error.kind(),"original JSON grammar demand refused"))?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<DslValue>>()).ok_or_else(||invalid("original JSON result capture overflow"))?;d},
   Phase::Source=>{let admission=self.source_admission.as_ref().ok_or_else(||invalid("original JSON source admission is absent"))?;if admission.phase()=="projectionStart"{admission.owned_projection_demand()?}else if admission.phase()=="ready"{admission.take_output_demand()}else{admission.next_demand()?}},
   Phase::Shell=>header::<Option<NativeReceivingOriginal<DslValue,M>>>(),
   Phase::Receive=>self.receiver.as_ref().unwrap().demands(body,u64::MAX)?,
   Phase::TakeReceived=>self.receiver.as_ref().unwrap().take_demand()?,
   Phase::CloseReceived=>{let receiver=self.receiver.as_ref().unwrap();if receiver.terminal_is_empty(){header::<Option<NativeReceivingOriginal<DslValue,M>>>()}else if receiver.is_closing(){receiver.close_demands(body)?}else{receiver.begin_close_demand()?}},
   Phase::Authority=>header::<FactoryOwnedSourceAuthority<M>>(),
   Phase::TypedSource=>{let admission=self.typed_admission.as_ref().ok_or_else(||invalid("original JSON typed source admission is absent"))?;if let Some(stage)=admission.projection_stage(&self.typed_frame)?{admission.projection_stage_quote(&self.typed_frame,stage)?.0}else if admission.phase()=="ready"{admission.take_output_demand()}else{admission.next_demand()?}},
   Phase::TypedProjection=>alias_quote(&self.typed_alias,self.typed_source.as_ref().ok_or_else(||invalid("original JSON typed source is absent"))?.try_borrow()?)?.0,
   Phase::CanonicalProjection=>alias_quote(&self.canonical_alias,self.canonical_child()?)?.0,
   Phase::CanonicalBirth=>ArtifactCanonicalJsonTreeCursor::constructor_demand(),
   Phase::Canonical=>{let mut d=self.canonical.as_ref().unwrap().next_demand()?;d.copy_bytes=d.copy_bytes.checked_add(64+size_of::<usize>()).ok_or_else(||invalid("original JSON canonical comparison extent overflow"))?;d},
   Phase::Ready=>header::<Option<RetainedOwnedProjection<M>>>(),
  })
 }
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value::native_decoding::owned::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  match self.advance_turn(control,witness,body,maximum_work_units,g){Ok(poll)=>poll,Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 fn local(action:&mut NativeReceivingChild<ValueError>,authority:&mut Option<NativeLocalAuthority<ValueError>>,control:&mut NativeDecodeControl<'_>,d:RetirementDemand,g:RetainedCloneGrant,q:u64,maximum_work_units:u64,tag:u8,mut op:impl FnMut(&mut NativeDecodeControl<'_>,&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,ValueError>{
  action.tag=tag;let authority=authority.as_mut().ok_or_else(||invalid("original JSON local authority is absent"))?;
  Ok(action.poll(control,d,g,q,maximum_work_units,authority,|control,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|op(control,effect)),effect,action)))
 }
 fn advance_turn(&mut self,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value::native_decoding::owned::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,ValueError>{
  use RetainedWorkPoll::{Complete,Pending};
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original JSON receiving changed its decoder ledger"));}
  if self.closing{return Err(invalid("original JSON receiving closes through its close step"));}
  let in_flight=self.action.demand.is_some();
  let idle=||if in_flight{Pending{work:0}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})};
  let d=self.demands(body)?;if maximum_work_units==0||!retained_clone_grant_admits(g,d){return Ok(idle());}
  if !in_flight{control.checkpoint()?;if !matches!(self.phase,Phase::Parse|Phase::Receive){control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0&&matches!(self.phase,Phase::CanonicalBirth){control.charge(d.capacity_bytes)?;}}}
  let child=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Phase>(),..g};let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  let pure=|step:RetainedCloneProgress|Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(step)),work:1});
  let q=if in_flight{self.action.work.unwrap_or(1)}else{match self.phase{Phase::Parse|Phase::Canonical=>maximum_work_units,Phase::Source=>match self.source_admission.as_ref(){Some(admission) if admission.phase()=="projectionStart"=>1,Some(admission) if admission.phase()!="ready"=>admission.advance_work_demand()?,_=>1},Phase::TypedSource=>match self.typed_admission.as_ref(){Some(admission) if admission.projection_stage(&self.typed_frame)?.is_some()=>admission.projection_stage_quote(&self.typed_frame,admission.projection_stage(&self.typed_frame)?.unwrap_or(1))?.1,Some(admission) if admission.phase()!="ready"=>admission.advance_work_demand()?,_=>1},Phase::TypedProjection=>alias_quote(&self.typed_alias,self.typed_source.as_ref().ok_or_else(||invalid("original JSON typed source is absent"))?.try_borrow()?)?.1,Phase::CanonicalProjection=>alias_quote(&self.canonical_alias,self.canonical_child()?)?.1,Phase::CloseReceived=>match self.receiver.as_ref(){Some(receiver) if !receiver.terminal_is_empty()=>if receiver.is_closing(){receiver.close_work_demand(body)?}else{receiver.begin_close_work_demand()?},_=>1},_=>1}};if !in_flight&&q>maximum_work_units{return Ok(idle());}
  let header_length=self.header.len();let phase=self.phase;
  let poll=match phase{
   Phase::Header=>{let input=self.wire.as_ref().ok_or_else(||invalid("original JSON wire is absent"))?.borrow()?;if input.get().get(..header_length)!=Some(self.header){self.faulted=true;return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original JSON operation wire header differs").with_retained_progress(unit()));}self.phase=Phase::Parse;let mut p=unit();p.copied_bytes-=size_of::<Phase>();pure(p)},
   Phase::Parse=>{
    let wire=&self.wire;let parser=&mut self.parser;let source_admission=&mut self.source_admission;let error=&mut self.error;let faulted=&mut self.faulted;let phase_slot=&mut self.phase;
    Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,1,|control,effect|{
     let input=match wire.as_ref().ok_or_else(||ValueError::invariant("wire absent")).and_then(|wire|wire.borrow()){Ok(input)=>input,Err(e)=>return Complete(RetainedWorkOutcome{result:Err(e),work:0})};
     let parse=match parser.as_mut(){Some(parse)=>parse,None=>return Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("parser absent")),work:0})};
     let bytes=match input.get().get(header_length..){Some(bytes)=>bytes,None=>return Complete(RetainedWorkOutcome{result:Err(invalid("original JSON wire header is truncated")),work:0})};
     let result=parse.step_source(bytes,1,control,effect,false);let work=u128::from(parse.last_turn_work());let mut p=parse.normal_step_progress();
     match result{Ok(Some(value))=>{**source_admission=Some(RetainedCloneSource::<DslValue>::admission_owned(value,()));*phase_slot=Phase::Source;},Ok(None)=>{},Err(e)=>{let kind=e.kind();**error=Some(e);*faulted=true;p.copied_bytes+=size_of::<Phase>();return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(kind,"original JSON syntax refused").with_retained_progress(p)),work});}}
     match p.copied_bytes.checked_add(size_of::<Option<DslValue>>()){Some(bytes)=>{p.copied_bytes=bytes;Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work})},None=>Complete(RetainedWorkOutcome{result:Err(invalid("original JSON parser receipt overflow")),work})}
    })?
   },
   Phase::Source=>{let admission=self.source_admission.as_mut().ok_or_else(||ValueError::invariant("source_admission absent"))?;
    let ready=admission.phase()=="ready"||self.action.tag==9;let projecting=admission.phase()=="projectionStart"||self.action.tag==16;let parsed=&mut self.parsed_source;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,if projecting{16}else if ready{9}else{2},|_,effect|{let turn=if projecting{admission.project_owned(&mut effect.nested(child))}else if ready{admission.take_output(parsed,&mut effect.nested(child))}else{admission.advance(&mut effect.nested(child))};Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})})?;
    if ready{if let Complete(turn)=&poll{if turn.result.is_ok()&&self.source_admission.as_ref().is_some_and(RetainedCloneSourceAdmission::terminal_is_empty){self.source_admission.take();self.phase=Phase::Shell;}}}poll},
   Phase::Shell=>{let source=self.parsed_source.take().ok_or_else(||invalid("original JSON parsed source is absent"))?;*self.receiver=Some(NativeReceivingOriginal::new(source));self.phase=Phase::Receive;let mut p=unit();p.copied_bytes-=size_of::<Phase>();pure(p)},
   Phase::Receive=>{let owner=self.receiver.as_mut().ok_or_else(||ValueError::invariant("receiver absent"))?;let poll=owner.advance(control,witness,body,maximum_work_units,g);if owner.output_ready(){self.phase=Phase::TakeReceived;}poll},
   Phase::TakeReceived=>{let receiver=self.receiver.as_mut().ok_or_else(||ValueError::invariant("receiver absent"))?;let typed=&mut self.typed;let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,3,|_,effect|receiver.take(typed,&mut effect.nested(child)).map_err(refused))?;
    if let Complete(turn)=&poll{if turn.result.is_ok(){self.phase=Phase::CloseReceived;}}poll},
   Phase::CloseReceived=>{if self.receiver.as_ref().ok_or_else(||ValueError::invariant("receiver absent"))?.terminal_is_empty(){self.receiver.take();self.phase=Phase::Authority;let mut p=unit();p.copied_bytes-=size_of::<Phase>();pure(p)}else{let receiver=self.receiver.as_mut().ok_or_else(||ValueError::invariant("receiver absent"))?;Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,4,|_,effect|close_native_receiver(receiver,body,&mut effect.nested(child)))?}},
   Phase::Authority=>{let authority=FactoryOwnedSourceAuthority::new(self.typed.take().ok_or_else(||ValueError::invariant("typed absent"))?,Arc::clone(self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?));*self.typed_admission=Some(RetainedCloneSourceAdmission::new(authority,()));self.phase=Phase::TypedSource;let mut p=unit();p.copied_bytes-=size_of::<Phase>();pure(p)},
   Phase::TypedSource=>{let admission=self.typed_admission.as_mut().ok_or_else(||ValueError::invariant("typed_admission absent"))?;
    let stage=if self.action.demand.is_some()&&(11..=15).contains(&self.action.tag){self.action.tag-10}else{admission.projection_stage(&self.typed_frame)?.unwrap_or(0)};
    if stage!=0{let frame=&mut self.typed_frame;let action=&mut self.frame_action;Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,stage+10,|_,effect|admission.project_staged(frame,action,stage,&mut effect.nested(child)))?}else{
    let ready=admission.phase()=="ready"||self.action.tag==10;let typed=&mut self.typed_source;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,if ready{10}else{5},|_,effect|{let turn=if ready{admission.take_output(typed,&mut effect.nested(child))}else{admission.advance(&mut effect.nested(child))};Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})})?;
    if ready{if let Complete(turn)=&poll{if turn.result.is_ok()&&self.typed_admission.as_ref().is_some_and(RetainedCloneSourceAdmission::terminal_is_empty){self.typed_admission.take();self.phase=Phase::TypedProjection;}}}poll}},
   Phase::TypedProjection=>{let source=self.typed_source.as_ref().ok_or_else(||ValueError::invariant("typed_source absent"))?;let publish=alias_bound(&self.typed_alias);let alias=&mut self.typed_alias;let output=&mut self.output;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,if publish{17}else{6},|_,effect|{let effect=&mut effect.nested(child);hop(effect,|effect|if publish{settle(alias.as_mut().ok_or_else(||invalid("alias admission absent")).and_then(|alias|relay(effect,|effect,reply|alias.take(output,effect,reply))))}else{let alias=alias.get_or_insert_with(Default::default);settle(relay(effect,|effect,reply|alias.admit(source.try_borrow()?,effect,reply)))})})?;
    if publish{if let Complete(turn)=&poll{if turn.result.is_ok(){if alias.as_ref().is_some_and(RetainedOwnedProjectionAdmission::terminal_is_empty){alias.take();}self.phase=Phase::CanonicalProjection;}}}poll},
   Phase::CanonicalProjection=>{let publish=alias_bound(&self.canonical_alias);let reference=self.output.as_ref().ok_or_else(||ValueError::invariant("output absent"))?.borrow()?.project(0,self.source);let alias=&mut self.canonical_alias;let slot=&mut self.canonical_projection;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,if publish{18}else{7},|_,effect|{let effect=&mut effect.nested(child);hop(effect,|effect|if publish{settle(alias.as_mut().ok_or_else(||invalid("alias admission absent")).and_then(|alias|relay(effect,|effect,reply|alias.take(slot,effect,reply))))}else{let alias=alias.get_or_insert_with(Default::default);settle(relay(effect,|effect,reply|alias.admit(reference,effect,reply)))})})?;
    if publish{if let Complete(turn)=&poll{if turn.result.is_ok(){if alias.as_ref().is_some_and(RetainedOwnedProjectionAdmission::terminal_is_empty){alias.take();}self.phase=Phase::CanonicalBirth;}}}poll},
   Phase::CanonicalBirth=>{let source=self.canonical_projection.take().ok_or_else(||invalid("original JSON canonical projection is absent"))?;match ArtifactCanonicalJsonTreeCursor::admit(source,child){Ok((cursor,p))=>{*self.canonical=Some(cursor);self.phase=Phase::Canonical;pure(p)},Err((error,source))=>{*self.canonical_projection=Some(source);return Err(error);}}},
   Phase::Canonical=>{
    let copy=64+size_of::<usize>();let canonical_child=RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-copy,..child};
    let canonical=self.canonical.as_mut().ok_or_else(||invalid("original JSON canonical cursor is absent"))?;let wire=&self.wire;let offset=&mut self.offset;let faulted=&mut self.faulted;let phase_slot=&mut self.phase;
    Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,8,|_,effect|{
     let mut bytes=[0;64];let step=match canonical.advance(&mut bytes,&mut effect.nested(canonical_child)){Ok(step)=>step,Err(e)=>return Complete(RetainedWorkOutcome{result:Err(e),work:0})};let work=u128::from(canonical.last_turn_work());
     if step.written_bytes==0&&matches!(&step.ownership,RetainedCloneStep::Progress(p) if *p==RetainedCloneProgress::default()){eprintln!("[DEBUG] mutapply5 zero canonical step cursor={:?} child={canonical_child:?} grant={:?} work={:?}",canonical.next_demand(),effect.grant(),effect.remaining_work_units());return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work})}
     let result=(||{let input=wire.as_ref().ok_or_else(||invalid("original JSON wire is absent"))?.borrow()?;let next=offset.checked_add(step.written_bytes).ok_or_else(||invalid("original JSON canonical offset overflow"))?;let equal=input.get().get(*offset..next)==Some(&bytes[..step.written_bytes]);*offset=next;let mut p=step.ownership.progress();p.copied_bytes=p.copied_bytes.checked_add(step.written_bytes+size_of::<usize>()).ok_or_else(||invalid("original JSON canonical receipt overflow"))?;if !equal||canonical.is_complete()&&*offset!=input.get().len(){*faulted=true;p.copied_bytes+=size_of::<Phase>();return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original JSON operation is not canonically encoded").with_retained_progress(p));}if canonical.is_complete(){*phase_slot=Phase::Ready;}Ok(RetainedCloneStep::Progress(p))})();
     Complete(RetainedWorkOutcome{result,work})
    })?
   },
   Phase::Ready=>return Ok(Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})),
  };
  Ok(poll.and_then(|step|{let mut p=step.progress();if p==RetainedCloneProgress::default()&&self.phase==phase{return Ok(step);}p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(size_of::<Phase>()).ok_or_else(||invalid("original JSON receiving receipt overflow"))?;semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original JSON receiving")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}))
 }
 pub fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{let result=self.take_ready(g);let work=u128::from(matches!(result,Ok(Some(_))));RetainedWorkOutcome{result,work}}
 fn take_ready(&mut self,g:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(g,d){return Ok(None);}let output=self.output.take().ok_or_else(||invalid("original JSON receiving output already transferred"))?;self.begin_close();Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn owned_handoff_pending(&self)->bool{self.taking_owned&&!self.closing&&!self.faulted}
 pub fn ready(&self)->bool{!self.taking_owned&&!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 pub fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.faulted||self.phase!=Phase::Ready||(!self.taking_owned&&self.output.is_none()){return Err(invalid("original JSON owned handoff requires its completed canonical output"))}
  if !self.taking_owned{return Ok(header::<bool>())}
  if let Some(canonical)=self.canonical.as_ref(){return if canonical.terminal_is_empty(){Ok(header::<Option<ArtifactCanonicalJsonTreeCursor>>())}else{canonical.next_demand()}}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)}}
  if let Some(projection)=self.canonical_projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>())}else{projection.close_demand(body)}}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_take_demand(body)}}
  if self.authority.is_some(){return Ok(FactoryOwnedSourceAuthority::<M>::transfer_original_demand())}
  Err(invalid("original JSON owned handoff lost its unique authority"))
 }
 pub fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let started=effect.performed_work();let result=self.take_owned_turn(output,control,body,effect);RetainedWorkOutcome{result,work:effect.performed_work().saturating_sub(started)}}
 fn take_owned_turn(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let g=effect.grant();
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original JSON owned handoff changed its decoder ledger"))}if output.is_some(){return Err(invalid("original JSON owned handoff destination is occupied"))}
  let d=self.take_owned_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.taking_owned{self.taking_owned=true;if let Some(canonical)=self.canonical.as_mut(){canonical.begin_close()}{Self::pay(effect)?;return Ok(unit())}}
  if let Some(canonical)=self.canonical.as_mut(){if canonical.terminal_is_empty(){self.canonical.take();{Self::pay(effect)?;return Ok(unit())}}let step=canonical.advance(&mut[],effect)?;return Ok(step.ownership)}
  if let Some(projection)=self.output.as_mut(){if projection.terminal_is_empty(){self.output.take();{Self::pay(effect)?;return Ok(unit())}}return Ok(settled(projection.close_step(body,effect))?.0)}
  if let Some(projection)=self.canonical_projection.as_mut(){if projection.terminal_is_empty(){self.canonical_projection.take();{Self::pay(effect)?;return Ok(unit())}}return Ok(settled(projection.close_step(body,effect))?.0)}
  if let Some(source)=self.typed_source.as_mut(){if source.terminal_is_empty(){self.typed_source.take();{Self::pay(effect)?;return Ok(unit())}}let authority=&mut *self.authority;return Ok(RetainedCloneStep::Progress(take_authority(source,authority,body,effect)?.progress()))}
  let mut factory=None;let step=transfer_original(self.authority.as_mut().ok_or_else(||invalid("original JSON owned handoff requires its unique authority"))?,output,&mut factory,effect)?;match(factory,step){(Some(factory),RetainedCloneStep::Complete(p))=>{self.authority.take();self.factory_close=Some(FactoryAuthority::new(factory));self.closing=true;Ok(RetainedCloneStep::Complete(p))},(None,step)=>Ok(RetainedCloneStep::Progress(step.progress())),_=>Err(invalid("original JSON owned handoff transferred a partial pair"))}
 }

 pub fn take_original_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.faulted||self.phase!=Phase::Ready||self.closing{return Err(invalid("original JSON owned output is not available"));}
  if !self.taking_owned{return Ok(header::<bool>());}
  if let Some(c)=self.canonical.as_ref(){return if c.terminal_is_empty(){Ok(header::<Option<ArtifactCanonicalJsonTreeCursor>>())}else{c.next_demand()};}
  if let Some(c)=self.canonical_projection.as_ref(){return if c.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>())}else{c.close_demand(body)};}
  if let Some(c)=self.output.as_ref(){return if c.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{c.close_demand(body)};}
  if let Some(s)=self.typed_source.as_ref(){return if s.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{let mut d=s.next_take_demand(body)?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<FactoryOwnedSourceAuthority<M>>>()).ok_or_else(||invalid("original JSON source transfer header overflow"))?;Ok(d)};}
  if self.authority.is_some(){return Ok(FactoryOwnedSourceAuthority::<M>::transfer_original_demand());}
  Err(invalid("original JSON owned output lost its source authority"))
 }
 pub fn take_original(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(RetainedCloneStep,Option<(M,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)>)>{let started=effect.performed_work();let result=self.take_original_turn(control,body,effect);RetainedWorkOutcome{result,work:effect.performed_work().saturating_sub(started)}}
 fn take_original_turn(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(RetainedCloneStep,Option<(M,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)>),ValueError>{
  let g=effect.grant();
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original JSON owned transfer changed its decoder ledger"));}let d=self.take_original_demand(body)?;if !retained_clone_grant_admits(g,d){return Ok((RetainedCloneStep::Progress(Default::default()),None));}control.checkpoint()?;control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.taking_owned{self.taking_owned=true;if let Some(c)=self.canonical.as_mut(){c.begin_close();}{Self::pay(effect)?;return Ok((unit(),None))};}
  if let Some(c)=self.canonical.as_mut(){if c.terminal_is_empty(){self.canonical.take();{Self::pay(effect)?;return Ok((unit(),None))};}let s=c.advance(&mut[],effect)?;return Ok((RetainedCloneStep::Progress(s.ownership.progress()),None));}
  if let Some(c)=self.canonical_projection.as_mut(){if c.terminal_is_empty(){self.canonical_projection.take();{Self::pay(effect)?;return Ok((unit(),None))};}let(step,_)=settled(c.close_step(body,effect))?;return Ok((RetainedCloneStep::Progress(step.progress()),None));}
  if let Some(c)=self.output.as_mut(){if c.terminal_is_empty(){self.output.take();{Self::pay(effect)?;return Ok((unit(),None))};}let(step,_)=settled(c.close_step(body,effect))?;return Ok((RetainedCloneStep::Progress(step.progress()),None));}
  if let Some(s)=self.typed_source.as_mut(){if s.terminal_is_empty(){self.typed_source.take();{Self::pay(effect)?;return Ok((unit(),None))};}let copied=2*size_of::<Option<FactoryOwnedSourceAuthority<M>>>();let narrowed=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copied,..g};let authority=&mut *self.authority;let mut p=take_authority(s,authority,body,&mut effect.nested(narrowed))?.progress();p.copied_bytes=p.copied_bytes.checked_add(copied).ok_or_else(||invalid("original JSON source transfer receipt overflow"))?;return Ok((RetainedCloneStep::Progress(p),None));}
  let mut value=None;let mut factory=None;let step=transfer_original(self.authority.as_mut().ok_or_else(||invalid("original JSON owned authority vanished"))?,&mut value,&mut factory,effect)?;match(value,factory,step){(Some(value),Some(factory),RetainedCloneStep::Complete(mut p))=>{self.authority.take();self.closing=true;self.begin_close();p.copied_bytes=d.copy_bytes;Ok((RetainedCloneStep::Complete(p),Some((value,factory))))},(None,None,step)=>Ok((RetainedCloneStep::Progress(step.progress()),None)),_=>Err(invalid("original JSON owned transfer produced a partial pair"))}
 }

 /// 💰️ Performs the one paid unit of a handoff turn that delegates to no nested owner.
 fn pay(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(),ValueError>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original JSON owned handoff exceeded its work authority"))}Ok(())}
 pub fn begin_close(&mut self){self.closing=true;if let Some(canonical)=self.canonical.as_mut(){canonical.begin_close();}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
  if let Some(admission)=self.typed_admission.as_ref(){return admission.next_return_demand();}
  if let Some(admission)=self.source_admission.as_ref(){return admission.next_return_demand();}
  if let Some(alias)=self.canonical_alias.as_ref(){return if alias.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjectionAdmission<dyn ArtifactCanonicalJsonTree>>>())}else{alias.close_demand(body)};}
  if let Some(alias)=self.typed_alias.as_ref(){return if alias.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjectionAdmission<M>>>())}else{alias.close_demand(body)};}
  if let Some(canonical)=self.canonical.as_ref(){return if canonical.terminal_is_empty(){Ok(close_header::<Option<ArtifactCanonicalJsonTreeCursor>>())}else{canonical.next_demand()};}
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(close_header::<Option<NativeReceivingOriginal<DslValue,M>>>())}else if receiver.is_closing(){receiver.close_demands(body)}else{receiver.begin_close_demand()};}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)};}
  if let Some(output)=self.canonical_projection.as_ref(){return if output.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>())}else{output.close_demand(body)};}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(close_header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  if let Some(source)=self.parsed_source.as_ref(){return if source.terminal_is_empty(){Ok(close_header::<Option<RetainedCloneSource<DslValue>>>())}else{source.next_close_demand(body)};}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)};}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body);}};}
  owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(typed,M);owner!(authority,FactoryOwnedSourceAuthority<M>);owner!(error,JsonError);owner!(local_authority,NativeLocalAuthority<ValueError>);
  if let Some(factory)=self.factory_close.as_ref(){return if factory.terminal_is_empty(){Ok(close_header::<FactoryAuthority>())}else{factory.demands(body)};}if self.factory.is_some(){return Ok(close_header::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>());}Ok(Default::default())
 }
 pub fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetirementPollExit as X;
  let g=effect.grant();let body=g.maximum_copy_bytes;
  let result=(||->Result<RetainedCloneStep,X>{
   if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original JSON cleanup changed its decoder ledger").into());}
   if !self.closing{return Err(invalid("original JSON cleanup was not started").into());}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   let d=self.close_demands(body)?;
   if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}
   let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
   if !self.retirement.is_empty(){return X::resume(self.retirement.step_granted(body,effect));}
   if let Some(admission)=self.typed_admission.as_mut(){if admission.phase()=="ready"{let turn=admission.take_output(&mut self.typed_source,effect);return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((authority,()))=returned{*self.authority=Some(authority);}if admission.terminal_is_empty(){self.typed_admission.take();}return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}
   if let Some(admission)=self.source_admission.as_mut(){if admission.phase()=="ready"{let turn=admission.take_output(&mut self.parsed_source,effect);return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((parsed,()))=returned{*self.parsed=Some(parsed);}if admission.terminal_is_empty(){self.source_admission.take();}return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}
   if let Some(canonical)=self.canonical.as_mut(){
    if canonical.terminal_is_empty(){self.canonical.take();X::pay(effect,1)?;return Ok(unit());}
    return canonical.advance(&mut[],effect).map(|step|step.ownership).map_err(Into::into);
   }
   if let Some(receiver)=self.receiver.as_mut(){
    if receiver.terminal_is_empty(){self.receiver.take();X::pay(effect,1)?;return Ok(unit());}
    return match close_native_receiver(receiver,body,effect){RetainedWorkPoll::Pending{work}=>Err(X::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(Into::into)};
   }
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();X::pay(effect,1)?;return Ok(unit());}return X::resume(owner.close_step(body,effect));}};}
   close!(canonical_alias);close!(typed_alias);close!(output);close!(canonical_projection);close!(typed_source);close!(parsed_source);close!(wire);
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);if let Some(step)=turn.result?{return Ok(step);}}};}
   owned!(parser);owned!(parsed);owned!(typed);owned!(authority);owned!(error);owned!(local_authority);
   if self.factory_close.is_none(){if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));X::pay(effect,1)?;return Ok(unit());}}
   if let Some(factory)=self.factory_close.as_mut(){
    if factory.terminal_is_empty(){self.factory_close=None;X::pay(effect,1)?;return Ok(unit());}
    return X::resume(factory.step(body,effect));
   }
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  X::settle(effect,result)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.wire.is_none()&&self.parser.is_none()&&self.parsed.is_none()&&self.parsed_source.is_none()&&self.source_admission.is_none()&&self.typed_admission.is_none()&&self.receiver.is_none()&&self.typed.is_none()&&self.authority.is_none()&&self.typed_source.is_none()&&self.typed_alias.is_none()&&self.canonical_alias.is_none()&&self.typed_frame.is_none()&&self.frame_action.is_none()&&self.output.is_none()&&self.canonical_projection.is_none()&&self.canonical.is_none()&&self.factory.is_none()&&self.factory_close.is_none()&&self.error.is_none()&&self.local_authority.is_none()&&self.action.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<M:NativeReceive<DslValue>+Sync> Drop for OriginalJsonReceiving<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original JSON receiving requires funded closure of all source and typed owners");}}

#[cfg(test)]
#[path="🧪️tests/🧰️support/🦀️.rs"]
mod test_support;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🎟️close/🧪️tests/🦀️.rs"]
mod close_work_tests;
