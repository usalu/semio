//! 🫴️ A borrowed original JSON output becomes one unique typed owner while every partial parser and receiver remains funded.
use semio_framework_pack_json::{JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy,JsonError};
use semio_framework_value::{DslValue,NativeDecodeControl,OriginalRetirementControl,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_value_native_owned::{NativeReceive,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority;
use semio_framework_value_native_owned::owned::{NativeReceivingChild,NativeLocalAuthority,NativeLocalProducer,NativeLocalOperation};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Parse,Bind,Source,Receiver,Receive,Take,CloseReceiver,CloseParser,Ready}
/// 🧳️ The transport retains the unchanged guest byte owner until this complete unique capture acknowledges it.
pub struct OriginalJsonOutputReceiving<T:NativeReceive<DslValue>+Sync>{ledger:NativeAllocationIdentity,recipient:Option<u64>,pointer:usize,length:usize,read_position:usize,parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,parsed:ManuallyDrop<Option<DslValue>>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,output:ManuallyDrop<Option<T>>,syntax:ManuallyDrop<Option<JsonError>>,action:NativeReceivingChild<ValueError>,local_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,retirement:RetainedCloneClose,phase:Phase,closing:bool,faulted:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original JSON output changed its source, decoder or ownership phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn add(mut d:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(bytes).ok_or_else(invalid)?;Ok(d)}
fn posted(mut p:RetainedCloneProgress,bytes:usize,work:u128)->Result<RetainedCloneProgress,ValueError>{if work!=0{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(bytes).ok_or_else(invalid)?;}Ok(p)}
fn refused(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(invalid).with_retained_progress(progress)}
fn done<E>(result:Result<RetainedCloneStep,E>,work:u128)->RetainedWorkPoll<RetainedCloneStep,E>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
fn progress_step(progress:RetainedCloneProgress)->RetainedCloneStep{RetainedCloneStep::Progress(progress)}
fn retirement_poll(poll:OriginalRetirementPoll)->RetainedWorkPoll<RetainedCloneStep,ValueError>{poll.map(|step|progress_step(step.progress())).map_err(OriginalRetirementFailure::into_value_error)}
impl<T:NativeReceive<DslValue>+Sync> OriginalJsonOutputReceiving<T>{
 pub fn constructor_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 pub fn admit(input:&[u8],limits:JsonReadLimits,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}let(_,parser)=JsonSourceCursor::<_,DslValue>::new(input,JsonMemberPolicy::Reject,limits)?.into_grammar();Ok((Self{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),pointer:input.as_ptr()as usize,length:input.len(),read_position:0,parser:ManuallyDrop::new(Some(parser)),parsed:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),output:ManuallyDrop::new(None),syntax:ManuallyDrop::new(None),action:Default::default(),local_authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeLocalProducer::new()))),retirement:Default::default(),phase:Phase::Parse,closing:false,faulted:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 fn validate(&self,input:&[u8],control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity()||self.pointer!=input.as_ptr()as usize||self.length!=input.len(){return Err(invalid())}Ok(())}
 fn metadata(&self)->usize{size_of::<Phase>()+size_of::<bool>()+match self.phase{Phase::Parse=>size_of::<usize>()+size_of::<Option<DslValue>>()+size_of::<Option<JsonError>>(),Phase::Bind=>size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>()+size_of::<Option<DslValue>>(),Phase::Source=>size_of::<Option<RetainedCloneSource<DslValue>>>(),Phase::Receiver=>size_of::<Option<NativeReceivingOriginal<DslValue,T>>>(),Phase::Take=>size_of::<Option<T>>(),_=>0}}
 pub fn demands(&self,input:&[u8],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(body)}if self.faulted{return Err(invalid())}if self.pointer!=input.as_ptr()as usize||self.length!=input.len(){return Err(invalid())}let body=body.saturating_sub(self.metadata());add(match self.phase{Phase::Parse=>self.parser.as_ref().ok_or_else(invalid)?.normal_step_demands(input).map_err(|error|ValueError::literal(error.kind(),"original JSON output grammar demand refused"))?,Phase::Bind|Phase::Receiver|Phase::Ready=>fixed(0),Phase::Source=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;if admission.phase()=="ready"{fixed(size_of::<Option<RetainedCloneSource<DslValue>>>())}else{admission.next_demand()?}},Phase::Receive=>self.receiver.as_ref().ok_or_else(invalid)?.demands(body,maximum_work_units)?,Phase::Take=>self.receiver.as_ref().ok_or_else(invalid)?.take_demand()?,Phase::CloseReceiver=>{let receiver=self.receiver.as_ref().ok_or_else(invalid)?;if receiver.terminal_is_empty(){fixed(size_of::<Option<NativeReceivingOriginal<DslValue,T>>>())}else{receiver.close_demands(body)?}},Phase::CloseParser=>if !self.retirement.is_empty(){self.retirement.next_owner_demands::<JsonGrammarCursor<DslValue>>(false,&None,body)?}else if self.parser.is_some(){self.retirement.next_owner_demands::<JsonGrammarCursor<DslValue>>(true,&None,body)?}else{fixed(0)}},self.metadata())}
 fn local(action:&mut NativeReceivingChild<ValueError>,authority:&mut Option<NativeLocalAuthority<ValueError>>,control:&mut NativeDecodeControl<'_>,d:RetirementDemand,g:RetainedCloneGrant,q:u64,maximum_work_units:u64,tag:u8,mut op:impl FnMut(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  action.tag=tag;let authority=authority.as_mut().ok_or_else(invalid)?;
  Ok(action.poll(control,d,g,q,maximum_work_units,authority,|control,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|op(control,effect)),effect,action)))
 }
 /// ⏯️ Performs one original grammar, source, native or cleanup frontier and forwards its exact physical receipt.
 pub fn advance(&mut self,input:&[u8],control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  match self.advance_turn(input,control,body,maximum_work_units,g){Ok(poll)=>poll,Err(error)=>done(Err(error.into()),0)}
 }
 fn advance_turn(&mut self,input:&[u8],control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  use RetainedWorkPoll::{Complete,Pending};
  let g=RetainedCloneGrant{maximum_items:g.maximum_items.min(1),..g};
  self.validate(input,control)?;
  if self.ready(){return Ok(done(Ok(RetainedCloneStep::Complete(Default::default())),0))}
  let in_flight=self.action.demand.is_some();
  let idle=||if in_flight{Pending{work:0}}else{done(Ok(progress_step(Default::default())),0)};
  let d=self.demands(input,body,maximum_work_units)?;if maximum_work_units==0||!retained_clone_grant_admits(g,d){return Ok(idle())}
  if !in_flight{control.checkpoint()?}
  let metadata=self.metadata();let inner=body.saturating_sub(metadata);
  let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(metadata),..g};
  let poll=match self.phase{
   Phase::Parse=>{
    let parser=&mut self.parser;let parsed=&mut self.parsed;let syntax=&mut self.syntax;let faulted=&mut self.faulted;let phase=&mut self.phase;let read=&mut self.read_position;
    Self::local(&mut self.action,&mut self.local_authority,control,d,g,maximum_work_units,maximum_work_units,1,|control,effect|{
     let Some(parse)=parser.as_mut()else{return done(Err(invalid()),0)};
     let result=parse.step_source(input,1,control,&mut effect.nested(child),false);
     let work=u128::from(parse.last_turn_work());*read=parse.position();let p=parse.normal_step_progress();
     match result{Ok(Some(value))=>{**parsed=Some(value);*phase=Phase::Bind},Ok(None)=>{},Err(error)=>{let kind=error.kind();**syntax=Some(error);if work!=0{*faulted=true}return done(Err(ValueError::literal(kind,"original JSON output grammar refused").with_retained_progress(p)),work)}}
     done(Ok(progress_step(p)),work)
    })?
   },
   Phase::Bind=>{let value=self.parsed.take().ok_or_else(invalid)?;*self.admission=Some(RetainedCloneSource::<DslValue>::admission_owned(value,()));self.phase=Phase::Source;done(Ok(progress_step(Default::default())),1)},
   Phase::Source=>{
    let admission=self.admission.as_mut().ok_or_else(invalid)?;
    let poll=if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,child);done(turn.result.map(|step|progress_step(step.progress())).map_err(OriginalReceivingPrimary::from),turn.work)}
     else{Self::local(&mut self.action,&mut self.local_authority,control,d,g,1,maximum_work_units,2,|_,effect|{let turn=admission.advance(&mut effect.nested(child));done(turn.result.map(|step|progress_step(step.progress())),turn.work)})?};
    if matches!(&poll,Complete(turn) if turn.result.is_ok())&&self.admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.admission.take();self.phase=Phase::Receiver}
    poll
   },
   Phase::Receiver=>{let source=self.source.take().ok_or_else(invalid)?;*self.receiver=Some(NativeReceivingOriginal::new(source));self.phase=Phase::Receive;done(Ok(progress_step(Default::default())),1)},
   Phase::Receive=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;
    let poll=receiver.advance(control,inner,maximum_work_units,child).map(|step|progress_step(step.progress()));
    if receiver.output_ready(){self.phase=Phase::Take}
    poll
   },
   Phase::Take=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;let output=&mut self.output;
    let poll=Self::local(&mut self.action,&mut self.local_authority,control,d,g,1,maximum_work_units,3,|_,effect|match receiver.take(&mut **output,&mut effect.nested(child)){Complete(turn)=>done(turn.result.map(|step|progress_step(step.progress())).map_err(refused),turn.work),Pending{..}=>done(Err(invalid()),0)})?;
    if matches!(&poll,Complete(turn) if turn.result.is_ok())&&self.output.is_some(){if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}self.phase=Phase::CloseReceiver}
    poll
   },
   Phase::CloseReceiver=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;
    if receiver.terminal_is_empty(){self.receiver.take();self.phase=Phase::CloseParser;done(Ok(progress_step(RetainedCloneProgress{copied_bytes:d.copy_bytes.saturating_sub(metadata),..Default::default()})),1)}
    else{let quote=receiver.close_work_demand(inner)?;Self::local(&mut self.action,&mut self.local_authority,control,d,g,quote,maximum_work_units,4,|_,effect|retirement_poll(receiver.close_step(&mut effect.nested(child))))?}
   },
   Phase::CloseParser=>{
    if !self.retirement.is_empty(){let quote=self.retirement.next_owner_work::<DslValue>(false,&None,inner)?;let retirement=&mut self.retirement;Self::local(&mut self.action,&mut self.local_authority,control,d,g,quote,maximum_work_units,5,|_,effect|retirement_poll(retirement.step_granted(&mut effect.nested(child))))?}
    else if self.parser.is_some(){let quote=self.retirement.next_owner_work::<JsonGrammarCursor<DslValue>>(true,&None,inner)?;let retirement=&mut self.retirement;let parser=&mut self.parser;Self::local(&mut self.action,&mut self.local_authority,control,d,g,quote,maximum_work_units,6,|_,effect|{let turn=retirement.begin_granted(parser,&mut effect.nested(child));done(turn.result.map(|step|step.unwrap_or_else(||progress_step(Default::default()))),turn.work)})?}
    else{self.phase=Phase::Ready;done(Ok(progress_step(Default::default())),1)}
   },
   Phase::Ready=>return Err(invalid()),
  };
  if self.action.failure.is_some(){self.faulted=true}
  let ready=self.ready();
  Ok(match poll{
   Pending{work}=>Pending{work},
   Complete(turn)=>{
    let work=turn.work;
    let result=match turn.result{
     Ok(step)=>posted(step.progress(),metadata,work).map(|p|if ready{RetainedCloneStep::Complete(p)}else{progress_step(p)}).map_err(OriginalReceivingPrimary::from),
     Err(mut error)=>{if work!=0{self.faulted=true;if let Ok(p)=posted(error.progress,metadata,work){error.progress=p}}Err(error)}
    };
    done(result,work)
   }
  })
 }
 pub fn phase(&self)->&'static str{match self.phase{Phase::Parse=>"parse",Phase::Bind=>"bind",Phase::Source=>"source",Phase::Receiver=>"receiver",Phase::Receive=>"receive",Phase::Take=>"take",Phase::CloseReceiver=>"closeReceiver",Phase::CloseParser=>"closeParser",Phase::Ready=>"ready"}}
 pub fn bytes_read(&self)->usize{self.read_position}
 pub fn source_bytes(&self)->usize{self.length}
 pub fn ready(&self)->bool{!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 pub fn syntax_error(&self)->Option<&JsonError>{self.syntax.as_ref()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(size_of::<Option<T>>()))}
 /// 📤️ Moves the completed owner into the caller's slot as exactly one recorded action of the caller's effect.
 pub fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let grant=effect.grant();
  let result=(||{
   if output.is_some(){return Err(invalid())}
   let d=self.take_demand()?;
   if !retained_clone_grant_admits(grant,d)||effect.maximum_work_units()==0{return Ok(progress_step(Default::default()))}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original JSON output take exceeded its work authority"))}
   *output=self.output.take();self.closing=true;
   Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
  })();
  RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body)}if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed(size_of::<Option<NativeReceivingOriginal<DslValue,T>>>()))}else{receiver.close_demands(body)}}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<Option<RetainedCloneSource<DslValue>>>()))}else{source.next_close_demand(body)}}if self.action.failure.is_some(){return self.action.failure_demand(&self.retirement,body)}macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}};}owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(output,T);owner!(syntax,JsonError);owner!(local_authority,NativeLocalAuthority<ValueError>);Ok(Default::default())}
 /// ⚖️ Quotes the work of the next cleanup action so the caller can open exactly one effect for it.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<DslValue>(false,&None,body)}if let Some(admission)=self.admission.as_ref(){return admission.return_work_demand()}if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(1)}else{receiver.close_work_demand(body)}}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_close_work_demand(body)}}if self.action.failure.is_some(){return self.action.failure_work(&self.retirement,body)}macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$ty>(true,&None,body)}};}owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(output,T);owner!(syntax,JsonError);owner!(local_authority,NativeLocalAuthority<ValueError>);Ok(0)}
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  use RetirementPollExit as X;
  let g=effect.grant();let mut work:u128=0;
  let result=(||->Result<RetainedCloneStep,X>{
   if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity()||!self.closing{return Err(invalid().into())}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(progress_step(Default::default()))}
   if !self.retirement.is_empty(){let turn=X::settled(self.retirement.step_granted(effect))?;work=turn.work;return turn.result.map_err(Into::into)}
   if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((value,()))=returned{*self.parsed=Some(value)}if admission.terminal_is_empty(){self.admission.take();}work=turn.work;return turn.result.map_err(Into::into)}
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();work=1;return Ok(progress_step(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}let turn=X::settled(owner.close_step(effect))?;work=turn.work;return turn.result.map_err(Into::into)}};}
   close!(receiver);
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();work=1;return Ok(progress_step(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}let turn=X::settled(source.close_step(effect))?;work=turn.work;return turn.result.map_err(Into::into)}
   if self.action.failure.is_some(){let turn=self.action.close_failure(&mut self.retirement,effect);work=turn.work;return turn.result.map_err(Into::into)}
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);work=turn.work;if let Some(step)=turn.result?{return Ok(step)}}};}
   owned!(parser);owned!(parsed);owned!(output);owned!(syntax);owned!(local_authority);Ok(RetainedCloneStep::Complete(Default::default()))
  })();X::finish(result,work)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.parsed.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.receiver.is_none()&&self.output.is_none()&&self.syntax.is_none()&&self.local_authority.is_none()&&self.action.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<T:NativeReceive<DslValue>+Sync> Drop for OriginalJsonOutputReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"unique original JSON output retains every parser, partial typed value and source until paid close")}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🎟️source/🦀️.rs"]mod source;
pub use source::{OriginalJsonOutputSource,OriginalJsonOutputOutcome,OriginalJsonOutputClose};
