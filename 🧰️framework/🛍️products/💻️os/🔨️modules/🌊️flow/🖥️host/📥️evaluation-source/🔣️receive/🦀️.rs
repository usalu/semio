//! 🔣️ Original borrowed JSON becomes typed Flow fields under the initiating native decoder.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value::{DslValue,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value_native_owned::native_decoding::owned::NativeReceive;
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonReadSource,JsonError};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceivingEffect,NativeReceivingEffectError,NativeReceivingEffectPhase,NativeReceivingEffectPoll};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow JSON receiver changed its funded source frontier")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn fits(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&retained_clone_grant_admits(cx.retained_grant(),d)}
fn receiving_fault<E:NativeReceivingEffectError>(error:E)->ValueError{let progress=error.receiving_progress();error.cause().cloned().unwrap_or_else(||ValueError::invariant("native receiving failed without a typed cause")).with_retained_progress(progress)}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; the turn is charged exactly the work it performed.
fn effected(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 if !fits(cx,d){return Ok(false)}
 let grant=cx.retained_grant();
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 result?;Ok(true)
}
/// 🫴️ One native receiving turn of a polled cursor under the context's receiving scope.
fn receive<E:NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,u64,RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 if !fits(cx,d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut polled=None;
 cx.with_native_receiving(&mut out,|native,grant|{polled=Some(run(native,maximum,grant));Ok(())})?;
 match polled{Some(RetainedWorkPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(RetainedWorkPoll::Pending{work})=>{cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));Ok(())},None=>Ok(())}
}
/// 🫴️ One native receiving turn of an effect-shaped cursor; an unstarted frame is cancelled so the turn retries intact.
fn receive_effect<E:NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 if !fits(cx,d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut frame=NativeReceivingEffect::<E>::default();let mut done=None;
 cx.with_native_receiving(&mut out,|native,_|{let polled=native.receive_effect(d,maximum,&mut frame,run)?;if matches!(polled,NativeReceivingEffectPoll::Pending{work:0})&&frame.phase()==NativeReceivingEffectPhase::Granted{frame.cancel_unstarted(native)?}done=Some(polled);Ok(())})?;
 match done{Some(NativeReceivingEffectPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(NativeReceivingEffectPoll::Pending{work:0})|None=>Ok(()),Some(NativeReceivingEffectPoll::Pending{..})=>Err(invalid().into())}
}
/// ♻️ Pays the receiver close start once, then closes it through its reply-shaped retirement cursor.
fn receiver_close<P:NativeReceive<DslValue>>(receiver:&mut NativeReceivingOriginal<DslValue,P>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 use semio_framework_value::retained_clone::OriginalEffectReply;
 use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
 let mut slot=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut slot).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop)).map(|()|None)}else{let mut stepped=None;OriginalEffectReply::borrow(&mut stepped).and_then(|reply|RetirementCursor::close_into(receiver,body,effect,reply).map(drop)).map(|()|stepped.take())};
 let work=effect.performed_work();
 match(ran,slot.take()){
  (Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(_),Some(settled))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled,work}),
  (Ok(Some(settled)),None)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(ValueError::invariant("original JSON receiver close produced a foreign child step").into())}),work}),
  (Ok(None),None)=>RetainedWorkPoll::Pending{work},
 }
}
struct JsonOwners<T:NativeReceive<DslValue>>{
 parser:Option<JsonGrammarCursor<DslValue>>,parsed:Option<DslValue>,admission:Option<RetainedCloneSourceAdmission<DslValue,DslValue>>,source:Option<RetainedCloneSource<DslValue>>,receiver:Option<NativeReceivingOriginal<DslValue,T>>,output:Option<T>,error:Option<JsonError>,
}
pub struct JsonField<T:NativeReceive<DslValue>>{owners:ManuallyDrop<JsonOwners<T>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:NativeReceive<DslValue>> JsonField<T>{
 pub fn new()->Self{Self{owners:ManuallyDrop::new(JsonOwners{parser:Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject)),parsed:None,admission:None,source:None,receiver:None,output:None,error:None}),retirement:Default::default(),phase:0,closing:false}}
 pub fn demands(&self,source:&(impl JsonReadSource+?Sized),copy:usize,work:u64)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(copy)}Ok(match self.phase{
  0=>self.owners.parser.as_ref().ok_or_else(invalid)?.normal_step_demands(source).map_err(|error|ValueError::literal(error.kind(),"original composed widget grammar demand refused"))?,
  1=>{let admission=self.owners.admission.as_ref().ok_or_else(invalid)?;if admission.phase()=="ready"{admission.take_output_demand()}else{admission.next_demand()?}},
  2=>self.owners.receiver.as_ref().ok_or_else(invalid)?.demands(copy,work)?,
  3=>self.owners.receiver.as_ref().ok_or_else(invalid)?.take_demand()?,
  _=>fixed(),
 })}
 pub fn step(&mut self,source:&(impl JsonReadSource+?Sized),cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let d=self.demands(source,cx.retained_grant().maximum_copy_bytes,cx.fuel_remaining())?;
  let owners=&mut *self.owners;
  match self.phase{
   0=>{
    let parser=owners.parser.as_mut().ok_or_else(invalid)?;let mut stepped=None;
    receive_effect(cx,d,|control,effect|{stepped=Some(parser.step_source(source,1,control,effect,false));RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneStep::Progress(parser.normal_step_progress())),work:effect.performed_work()}})?;
    match stepped{
     Some(Ok(Some(value)))=>{owners.admission=Some(RetainedCloneSource::admission_owned(value,()));self.phase=1},
     Some(Ok(None))|None=>{},
     Some(Err(JsonError::Native(error)))=>return Err(error.into()),
     Some(Err(error))=>{let kind=error.kind();owners.error=Some(error);return Err(ValueError::literal(kind,"original composed widget JSON refused").into())},
    }
   },
   1=>{
    let admission=owners.admission.as_mut().ok_or_else(invalid)?;
    let ready=admission.phase()=="ready";
    let ran=if ready{effected(cx,d,|effect|admission.take_output(&mut owners.source,effect).result.map_err(Into::into))?}else{effected(cx,d,|effect|admission.advance(effect).result.map_err(Into::into))?};
    if ran&&ready&&admission.terminal_is_empty(){owners.admission=None;owners.receiver=Some(NativeReceivingOriginal::new(owners.source.take().ok_or_else(invalid)?));self.phase=2}
   },
   2=>{
    let receiver=owners.receiver.as_mut().ok_or_else(invalid)?;let body=cx.retained_grant().maximum_copy_bytes;
    receive(cx,d,|native,maximum,grant|{let witness=match native.source_witness(){Ok(witness)=>witness,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};receiver.advance(native,witness,body,maximum,grant)})?;
    if receiver.output_ready(){self.phase=3}
   },
   3=>{
    let receiver=owners.receiver.as_mut().ok_or_else(invalid)?;
    let taken=effected(cx,d,|effect|match receiver.take(&mut owners.output,effect){RetainedWorkPoll::Complete(turn)=>turn.result.map_err(receiving_fault),RetainedWorkPoll::Pending{..}=>Err(invalid())})?;
    if taken&&owners.output.is_some(){self.phase=4}
   },
   _=>{if fits(cx,d){cx.consume_retained(unit())?;cx.consume_fuel(1)}},
  }
  Ok(self.phase==4)
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),..fixed()}}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=self.take_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}Ok(self.owners.output.take().map(|value|(value,RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})))}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,copy)}
  if let Some(receiver)=self.owners.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed())}else if receiver.is_closing(){receiver.close_demands(copy)}else{receiver.begin_close_demand()}}
  if let Some(admission)=self.owners.admission.as_ref(){return admission.next_return_demand()}
  if let Some(source)=self.owners.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed())}else{source.next_close_demand(copy)}}
  macro_rules! owned{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owned!(parser,JsonGrammarCursor<DslValue>);owned!(parsed,DslValue);owned!(output,T);owned!(error,JsonError);Ok(fixed())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   if !self.closing||!retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.retirement.is_empty(){let turn=crate::poll_outcome(self.retirement.step_granted(grant.maximum_copy_bytes,effect));work=turn.work;return turn.result}
   if let Some(receiver)=self.owners.receiver.as_mut(){if receiver.terminal_is_empty(){self.owners.receiver=None;work=1;return Ok(RetainedCloneStep::Progress(unit()))}let turn=crate::poll_outcome(receiver_close(receiver,grant.maximum_copy_bytes,effect));work=turn.work;return turn.result}
   if let Some(admission)=self.owners.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);work=turn.work;let terminal=admission.terminal_is_empty();if let Some((value,()))=returned{self.owners.parsed=Some(value)}if terminal{self.owners.admission=None}return turn.result.map_err(Into::into)}
   if let Some(source)=self.owners.source.as_mut(){if source.terminal_is_empty(){self.owners.source=None;work=1;return Ok(RetainedCloneStep::Progress(unit()))}let turn=crate::poll_outcome(source.close_step(grant.maximum_copy_bytes,effect));work=turn.work;return turn.result}
   macro_rules! owned{($field:ident)=>{if self.owners.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.owners.$field,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}}}
   owned!(parser);owned!(parsed);owned!(output);owned!(error);Ok(RetainedCloneStep::Complete(Default::default()))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_empty()&&self.owners.parser.is_none()&&self.owners.parsed.is_none()&&self.owners.admission.is_none()&&self.owners.source.is_none()&&self.owners.receiver.is_none()&&self.owners.output.is_none()&&self.owners.error.is_none()}
}
impl<T:NativeReceive<DslValue>> Drop for JsonField<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original composed JSON field must close its grammar and receiving custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}


#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod tests;
