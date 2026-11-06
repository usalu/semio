//! 🪪️ Borrowed immutable JSON becomes one native owner through the initiating decoder and funded frontiers.
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonReadLimits,JsonSourceCursor,JsonError};
use semio_framework_value::{DslValue,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,OriginalRetirementControl};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value_native_owned::{NativeReceive,NativeDecodeControl,OriginalReceivingPrimary};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy)]
enum Phase{Parse,Source,Shell,Receive,Ready}
/// 🧾️ Keeps the original text borrow, grammar, native receiver and retirement owners in one bounded lifecycle.
pub struct JsonNativeInputCursor<'source,T:NativeReceive<DslValue>+Sync>{
 source:&'source str,ledger:u64,recipient:Option<u64>,phase:Phase,closing:bool,faulted:bool,
 parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,
 parsed:ManuallyDrop<Option<DslValue>>,source_owner:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,
 receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,
 error:ManuallyDrop<Option<JsonError>>,retirement:RetainedCloneClose,
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
  Ok(Some((Self{source,ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),phase:Phase::Parse,closing:false,faulted:false,parser:ManuallyDrop::new(Some(parser)),admission:ManuallyDrop::new(None),parsed:ManuallyDrop::new(None),source_owner:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),error:ManuallyDrop::new(None),retirement:Default::default()},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn source(&self)->&'source str{self.source}
 pub fn ready(&self)->bool{!self.closing&&!self.faulted&&matches!(self.phase,Phase::Ready)}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}if self.faulted{return Err(invalid("faulted native JSON input requires cleanup"))}
  let d=match self.phase{
   Phase::Parse=>add(self.parser.as_ref().ok_or_else(||invalid("native JSON parser absent"))?.normal_step_demands(self.source).map_err(|error|ValueError::literal(error.kind(),"native JSON grammar demand refused"))?,size_of::<Option<DslValue>>()+size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>())?,
   Phase::Source=>{let a=self.admission.as_ref().ok_or_else(||invalid("native JSON admission absent"))?;if a.phase()=="ready"{header::<Option<RetainedCloneSource<DslValue>>>()}else{a.next_demand()?}},
   Phase::Shell=>header::<Option<NativeReceivingOriginal<DslValue,T>>>(),
   Phase::Receive=>self.receiver.as_ref().ok_or_else(||invalid("native JSON receiver absent"))?.demands(body)?,
   Phase::Ready=>self.receiver.as_ref().ok_or_else(||invalid("native JSON receiver absent"))?.take_demand()?,
  };add(d,size_of::<Phase>())
 }
 fn same(&self,control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){Err(invalid("native JSON input changed its original decoder"))}else{Ok(())}}
 /// ⏯️ Performs one original grammar, source or native frontier and forwards its exact physical receipt.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   self.same(control)?;let d=self.demands(body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.closing{return Err(invalid("native JSON input requires retirement authority"))}if self.ready(){return Err(invalid("native JSON input is ready for transfer"))}
   control.checkpoint()?;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Phase>(),..grant};
   let mut progress=match self.phase{
    Phase::Parse=>{
     let capture=size_of::<Option<DslValue>>()+size_of::<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>();
     let parser=self.parser.as_mut().ok_or_else(||invalid("native JSON parser absent"))?;
     let result=parser.step_source(self.source,1,control,RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-capture,..child},false);
     work=parser.last_turn_work();let mut p=parser.normal_step_progress();p.copied_bytes+=capture;
     match result{Ok(Some(value))=>{*self.admission=Some(RetainedCloneSourceAdmission::new(value,(),|value|value));self.phase=Phase::Source},Ok(None)=>{},Err(error)=>{let kind=error.kind();*self.error=Some(error);self.faulted=true;p.copied_bytes+=size_of::<Phase>();return Err(ValueError::literal(kind,"native JSON input syntax refused").with_retained_progress(p))}}p
    },
    Phase::Source=>{
     let admission=self.admission.as_mut().ok_or_else(||invalid("native JSON admission absent"))?;
     let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source_owner,child)}else{admission.advance(child)};
     let turn=control.receive_retained_work(child,turn);work=turn.work;let p=turn.result.map_err(refused)?.progress();
     if admission.terminal_is_empty(){self.admission.take();self.phase=Phase::Shell}p
    },
    Phase::Shell=>{*self.receiver=Some(NativeReceivingOriginal::new(self.source_owner.take().ok_or_else(||invalid("native JSON source absent"))?));self.phase=Phase::Receive;work=1;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<Phase>(),..Default::default()}},
    Phase::Receive=>{let receiver=self.receiver.as_mut().ok_or_else(||invalid("native JSON receiver absent"))?;let turn=receiver.advance(control,body,child);work=turn.work;let p=turn.result.map_err(refused)?.progress();if receiver.output_ready(){self.phase=Phase::Ready}p},
    Phase::Ready=>return Err(invalid("native JSON input already ready")),
   };progress.copied_bytes+=size_of::<Phase>();Ok(RetainedCloneStep::Progress(progress))
  })();RetainedWorkOutcome{result,work}
 }
 /// 📤️ Transfers the original typed result while retaining funded parser and source cleanup.
 pub fn take(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{
  let mut work=0;let result=(||{
   self.same(control)?;if !self.ready(){return Err(invalid("native JSON input is not ready"))}let d=self.demands(grant.maximum_copy_bytes)?;
   if !retained_clone_grant_admits(grant,d){return Ok(None)}control.checkpoint()?;
   let turn=self.receiver.as_mut().ok_or_else(||invalid("native JSON receiver absent"))?.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Phase>(),..grant});work=turn.work;
   let Some((value,mut p))=turn.result?else{return Ok(None)};p.copied_bytes+=size_of::<Phase>();self.begin_close();Ok(Some((value,p)))
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
  owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(error,JsonError);Ok(Default::default())
 }
 /// ♻️ Releases each actual source, parser and native receiver under the same retirement issuer.
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid("native JSON input cleanup changed its original decoder"))}
   let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result}
   if let Some(a)=self.admission.as_mut(){let mut returned=None;let turn=a.return_inputs(&mut returned,grant);if let Some((parsed,()))=returned{*self.parsed=Some(parsed)}if a.terminal_is_empty(){self.admission.take();}work=turn.work;return turn.result}
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}let turn=owner.close_step(grant);work=turn.work;return turn.result}};}
   close!(receiver);close!(source_owner);
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);work=turn.work;if let Some(step)=turn.result?{return Ok(step)}}};}
   owned!(parser);owned!(parsed);owned!(error);Ok(RetainedCloneStep::Complete(Default::default()))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.admission.is_none()&&self.parsed.is_none()&&self.source_owner.is_none()&&self.receiver.is_none()&&self.error.is_none()&&self.retirement.is_empty()}
}
impl<T:NativeReceive<DslValue>+Sync> Drop for JsonNativeInputCursor<'_,T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native JSON input requires funded cleanup")}}
#[cfg(test)]
#[path="🧪️tests.rs"]
mod tests;
