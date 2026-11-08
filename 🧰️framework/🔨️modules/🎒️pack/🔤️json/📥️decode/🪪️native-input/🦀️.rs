//! 🪪️ Borrowed immutable JSON becomes one native owner through the initiating decoder and funded frontiers.
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonReadLimits,JsonSourceCursor,JsonError};
use semio_framework_value::{DslValue,RetirementDemand,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,RetainedAllocationEffect,OriginalRetirementPoll,ValueError,ValueRefusalKind,OriginalRetirementControl};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value_native_owned::{NativeReceive,NativeDecodeControl,OriginalReceivingPrimary};
use semio_framework_value_native_owned::owned::{NativeReceivingChild,NativeLocalAuthority,NativeLocalProducer,NativeLocalOperation};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy)]
enum Phase{Parse,Source,Shell,Receive,Ready}
/// 🧾️ Keeps the original text borrow, grammar, native receiver and retirement owners in one bounded lifecycle.
pub struct JsonNativeInputCursor<'source,T:NativeReceive<DslValue>+Sync>{
 source:&'source str,ledger:semio_framework_value::retained_clone::NativeAllocationIdentity,recipient:Option<u64>,phase:Phase,closing:bool,faulted:bool,
 parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,
 parsed:ManuallyDrop<Option<DslValue>>,source_owner:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,
 receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,
 error:ManuallyDrop<Option<JsonError>>,action:NativeReceivingChild<ValueError>,local_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,retirement:RetainedCloneClose,
}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn add(mut demand:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(bytes).ok_or_else(||invalid("native JSON input extent overflow"))?;Ok(demand)}
fn refused(failure:OriginalReceivingPrimary)->ValueError{failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(||invalid("native JSON input construction refused")).with_retained_progress(failure.progress)}
impl<'source,T:NativeReceive<DslValue>+Sync> JsonNativeInputCursor<'source,T>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 /// 🎟️ Admits the inline cursor before source traversal, retaining the exact immutable borrow.
 pub fn admit(source:&'source str,limits:JsonReadLimits,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}
  let (_,parser)=JsonSourceCursor::<_,DslValue>::new(source,JsonMemberPolicy::Reject,limits)?.into_grammar();
  Ok(Some((Self{source,ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),phase:Phase::Parse,closing:false,faulted:false,parser:ManuallyDrop::new(Some(parser)),admission:ManuallyDrop::new(None),parsed:ManuallyDrop::new(None),source_owner:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),error:ManuallyDrop::new(None),action:Default::default(),local_authority:ManuallyDrop::new(Some(semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority::new(NativeLocalProducer::new()))),retirement:Default::default()},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn source(&self)->&'source str{self.source}
 pub fn ready(&self)->bool{!self.closing&&!self.faulted&&matches!(self.phase,Phase::Ready)}
 pub fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}if self.faulted{return Err(invalid("faulted native JSON input requires cleanup"))}
  let d=match self.phase{
   Phase::Parse=>add(self.parser.as_ref().ok_or_else(||invalid("native JSON parser absent"))?.normal_step_demands(self.source).map_err(|error|ValueError::literal(error.kind(),"native JSON grammar demand refused"))?,size_of::<Option<DslValue>>()+size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>())?,
   Phase::Source=>{let a=self.admission.as_ref().ok_or_else(||invalid("native JSON admission absent"))?;if a.phase()=="ready"{header::<Option<RetainedCloneSource<DslValue>>>()}else{a.next_demand()?}},
   Phase::Shell=>header::<Option<NativeReceivingOriginal<DslValue,T>>>(),
   Phase::Receive=>self.receiver.as_ref().ok_or_else(||invalid("native JSON receiver absent"))?.demands(body,maximum_work_units)?,
   Phase::Ready=>self.receiver.as_ref().ok_or_else(||invalid("native JSON receiver absent"))?.take_demand()?,
  };add(d,size_of::<Phase>())
 }
 fn same(&self,control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){Err(invalid("native JSON input changed its original decoder"))}else{Ok(())}}
 fn local(action:&mut NativeReceivingChild<ValueError>,authority:&mut Option<NativeLocalAuthority<ValueError>>,control:&mut NativeDecodeControl<'_>,d:RetirementDemand,g:RetainedCloneGrant,q:u64,maximum_work_units:u64,tag:u8,mut op:impl FnMut(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  action.tag=tag;let authority=authority.as_mut().ok_or_else(||invalid("native JSON local authority is absent"))?;
  Ok(action.poll(control,d,g,q,maximum_work_units,authority,|control,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|op(control,effect)),effect,action)))
 }
 /// ⏯️ Performs one original grammar, source or native frontier and forwards its exact physical receipt.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  match self.advance_turn(control,body,maximum_work_units,grant){Ok(poll)=>poll,Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 fn advance_turn(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  use RetainedWorkPoll::{Complete,Pending};
  self.same(control)?;
  if self.closing{return Err(invalid("native JSON input requires retirement authority"))}if self.ready(){return Err(invalid("native JSON input is ready for transfer"))}
  let in_flight=self.action.demand.is_some();
  let idle=||if in_flight{Pending{work:0}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})};
  let d=self.demands(body,maximum_work_units)?;if maximum_work_units==0||!retained_clone_grant_admits(g,d){return Ok(idle())}
  if !in_flight{control.checkpoint()?}
  let child=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Phase>(),..g};
  let q=if in_flight{self.action.work.unwrap_or(1)}else{match self.phase{Phase::Parse=>maximum_work_units,_=>1}};
  let poll=match self.phase{
   Phase::Parse=>{
    let capture=size_of::<Option<DslValue>>()+size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>();
    let source=self.source;let parser=&mut self.parser;let admission=&mut self.admission;let error=&mut self.error;let faulted=&mut self.faulted;let phase_slot=&mut self.phase;
    Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,1,|control,effect|{
     let parse=match parser.as_mut(){Some(parse)=>parse,None=>return Complete(RetainedWorkOutcome{result:Err(invalid("native JSON parser absent")),work:0})};
     let result=parse.step_source(source,1,control,&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-capture,..child}),false);
     let work=u128::from(parse.last_turn_work());let mut p=parse.normal_step_progress();p.copied_bytes+=capture;
     match result{Ok(Some(value))=>{**admission=Some(RetainedCloneSourceAdmission::new(value,(),|value|value));*phase_slot=Phase::Source},Ok(None)=>{},Err(e)=>{let kind=e.kind();**error=Some(e);*faulted=true;p.copied_bytes+=size_of::<Phase>();return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(kind,"native JSON input syntax refused").with_retained_progress(p)),work})}}
     Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work})
    })?
   },
   Phase::Source=>{
    let admission=self.admission.as_mut().ok_or_else(||invalid("native JSON admission absent"))?;
    let poll=if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source_owner,child);Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(OriginalReceivingPrimary::from),work:turn.work})}
     else{Self::local(&mut self.action,&mut self.local_authority,control,d,g,q,maximum_work_units,2,|_,effect|{let turn=admission.advance(&mut effect.nested(child));Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})})?};
    if matches!(&poll,Complete(turn) if turn.result.is_ok())&&self.admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.admission.take();self.phase=Phase::Shell}
    poll
   },
   Phase::Shell=>{*self.receiver=Some(NativeReceivingOriginal::new(self.source_owner.take().ok_or_else(||invalid("native JSON source absent"))?));self.phase=Phase::Receive;Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<Phase>(),..Default::default()})),work:1})},
   Phase::Receive=>{
    let receiver=self.receiver.as_mut().ok_or_else(||invalid("native JSON receiver absent"))?;
    let poll=receiver.advance(control,body,maximum_work_units,child).and_then(|step|Ok(RetainedCloneStep::Progress(step.progress())));
    if receiver.output_ready(){self.phase=Phase::Ready}
    poll
   },
   Phase::Ready=>return Err(invalid("native JSON input already ready")),
  };
  Ok(poll.and_then(|step|{let mut p=step.progress();p.copied_bytes=p.copied_bytes.checked_add(size_of::<Phase>()).ok_or_else(||OriginalReceivingPrimary::from(invalid("native JSON input receipt overflow")))?;Ok(RetainedCloneStep::Progress(p))}))
 }
 /// 📤️ Transfers the original typed result into the caller's slot while retaining funded parser and source cleanup.
 pub fn take(&mut self,control:&mut NativeDecodeControl<'_>,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let mut work=0u128;let result=(||{
   self.same(control)?;if !self.ready(){return Err(invalid("native JSON input is not ready"))}let d=self.demands(grant.maximum_copy_bytes,effect.maximum_work_units())?;
   if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;
   let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Phase>(),..grant};
   let turn=match self.receiver.as_mut().ok_or_else(||invalid("native JSON receiver absent"))?.take(output,&mut effect.nested(child)){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>return Err(invalid("native JSON take left its producer reply pending"))};work=turn.work;
   let mut p=turn.result.map_err(refused)?.progress();p.copied_bytes+=size_of::<Phase>();
   if output.is_none(){return Ok(RetainedCloneStep::Progress(p))}self.begin_close();Ok(RetainedCloneStep::Complete(p))
  })();RetainedWorkOutcome{result,work}
 }
 /// 🛑️ Marks cancellation without losing any partially admitted owner or original source.
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid("native JSON input cleanup not started"))}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body)}
  if let Some(a)=self.admission.as_ref(){return a.next_return_demand()}
  if let Some(r)=self.receiver.as_ref(){return if r.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{r.close_demands(body)}}
  if let Some(s)=self.source_owner.as_ref(){return if s.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{s.next_close_demand(body)}}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}};}
  if self.action.failure.is_some(){return self.action.failure_demand(&self.retirement,body)}
  owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(error,JsonError);owner!(local_authority,NativeLocalAuthority<ValueError>);Ok(Default::default())
 }
  /// ♻️ Releases each actual source, parser and native receiver under the same retirement issuer.
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  use RetirementPollExit as X;
  let grant=effect.grant();let mut work:u128=0;
  let result=(||->Result<RetainedCloneStep,X>{
   if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid("native JSON input cleanup changed its original decoder").into())}
   let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   if !self.retirement.is_empty(){let turn=X::settled(self.retirement.step_granted(effect))?;work=turn.work;return turn.result.map_err(Into::into)}
   if let Some(a)=self.admission.as_mut(){let mut returned=None;let turn=a.return_inputs(&mut returned,effect);if let Some((parsed,()))=returned{*self.parsed=Some(parsed)}if a.terminal_is_empty(){self.admission.take();}work=turn.work;return turn.result.map_err(Into::into)}
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}let turn=X::settled(owner.close_step(effect))?;work=turn.work;return turn.result.map_err(Into::into)}};}
   close!(receiver);close!(source_owner);
   if self.action.failure.is_some(){let turn=self.action.close_failure(&mut self.retirement,effect);work=turn.work;return turn.result.map_err(Into::into)}
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);work=turn.work;if let Some(step)=turn.result?{return Ok(step)}}};}
   owned!(parser);owned!(parsed);owned!(error);owned!(local_authority);Ok(RetainedCloneStep::Complete(Default::default()))
  })();X::finish(result,work)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.admission.is_none()&&self.parsed.is_none()&&self.source_owner.is_none()&&self.receiver.is_none()&&self.error.is_none()&&self.local_authority.is_none()&&self.action.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<T:NativeReceive<DslValue>+Sync> Drop for JsonNativeInputCursor<'_,T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native JSON input requires funded cleanup")}}
#[cfg(test)]
#[path="🧪️tests.rs"]
mod tests;
