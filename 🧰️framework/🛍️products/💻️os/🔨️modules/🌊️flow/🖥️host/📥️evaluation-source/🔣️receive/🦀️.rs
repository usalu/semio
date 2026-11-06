//! 🔣️ Original borrowed JSON becomes typed Flow fields under the initiating native decoder.
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value_native_owned::native_decoding::owned::{NativeReceive,NativeReceivingOriginal};
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonReadSource,JsonError};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow JSON receiver changed its funded source frontier")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
struct JsonOwners<T:NativeReceive<DslValue>>{
 parser:Option<JsonGrammarCursor<DslValue>>,parsed:Option<DslValue>,admission:Option<RetainedCloneSourceAdmission<DslValue,DslValue>>,source:Option<RetainedCloneSource<DslValue>>,receiver:Option<NativeReceivingOriginal<DslValue,T>>,output:Option<T>,error:Option<JsonError>,
}
pub struct JsonField<T:NativeReceive<DslValue>>{owners:ManuallyDrop<JsonOwners<T>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:NativeReceive<DslValue>> JsonField<T>{
 pub fn new()->Self{Self{owners:ManuallyDrop::new(JsonOwners{parser:Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject)),parsed:None,admission:None,source:None,receiver:None,output:None,error:None}),retirement:Default::default(),phase:0,closing:false}}
 pub fn demands(&self,source:&(impl JsonReadSource+?Sized),copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(copy)}Ok(match self.phase{
  0=>self.owners.parser.as_ref().unwrap().normal_step_demands(source).map_err(|error|ValueError::literal(error.kind(),"original composed widget grammar demand refused"))?,
  1=>{let admission=self.owners.admission.as_ref().unwrap();if admission.phase()=="ready"{fixed()}else{admission.next_demand()?}},
  2=>self.owners.receiver.as_ref().unwrap().demands(copy)?,
  3=>self.owners.receiver.as_ref().unwrap().take_demand()?,
  _=>fixed(),
 })}
 pub fn step(&mut self,source:&(impl JsonReadSource+?Sized),cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let d=self.demands(source,cx.retained_grant().maximum_copy_bytes)?;if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}
  let mut progress=Default::default();let mut work=0;let grant=cx.retained_grant();
  let result:Result<(),OriginalTurnFailure>=cx.with_native_receiving(|control|{
   let owners=&mut *self.owners;
   match self.phase{
    0=>{let parser=owners.parser.as_mut().unwrap();let result=parser.step_source(source,1,control,grant,false);progress=parser.normal_step_progress();work=parser.last_turn_work();match result{Ok(Some(value))=>{owners.admission=Some(RetainedCloneSource::admission_owned(value,()));self.phase=1},Ok(None)=>{},Err(error)=>{let kind=error.kind();owners.error=Some(error);return Err(ValueError::literal(kind,"original composed widget JSON refused").into())}}},
    1=>{let admission=owners.admission.as_mut().unwrap();if admission.phase()=="ready"{let turn=admission.take_output(&mut owners.source,grant);work=turn.work;progress=match turn.result{Ok(step)=>step.progress(),Err(error)=>{progress=error.retained_progress();return Err(error.into())}};if admission.terminal_is_empty(){owners.admission=None;owners.receiver=Some(NativeReceivingOriginal::new(owners.source.take().unwrap()));self.phase=2}}else{control.charge(d.capacity_bytes)?;let turn=admission.advance(grant);work=turn.work;progress=match turn.result{Ok(step)=>step.progress(),Err(error)=>{progress=error.retained_progress();return Err(error.into())}};}},
    2=>{let turn=owners.receiver.as_mut().unwrap().advance(control,grant.maximum_copy_bytes,grant);work=turn.work;progress=match turn.result{Ok(step)=>step.progress(),Err(error)=>{progress=error.progress;return Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(error),receipt_refusal:None,context_refusal:None,work_refusal:None})}};if owners.receiver.as_ref().unwrap().output_ready(){self.phase=3}},
    3=>{let turn=owners.receiver.as_mut().unwrap().take(grant);work=turn.work;let taken=match turn.result{Ok(taken)=>taken,Err(error)=>{progress=error.retained_progress();return Err(error.into())}};if let Some((output,p))=taken{progress=p;owners.output=Some(output);self.phase=4}},
    _=>{progress=unit();work=1},
   }Ok(())
  });
  let receipt=cx.receiving_issued_grant().map(|grant|cx.consume_receiving_retained(grant,progress)).transpose();cx.consume_fuel(work);
  match(result,receipt){(Err(mut error),Err(receipt))=>{error.context_refusal=Some(receipt);Err(error)},(Err(error),_)=>Err(error),(_,Err(error))=>Err(error.into()),_=>Ok(self.phase==4)}
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),..fixed()}}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=self.take_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}Ok(self.owners.output.take().map(|value|(value,RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.owners.receiver.as_mut(){receiver.begin_close()}}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,copy)}
  if let Some(receiver)=self.owners.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed())}else{receiver.close_demands(copy)}}
  if let Some(admission)=self.owners.admission.as_ref(){return admission.next_return_demand()}
  if let Some(source)=self.owners.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed())}else{source.next_close_demand(copy)}}
  macro_rules! owned{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owned!(parser,JsonGrammarCursor<DslValue>);owned!(parsed,DslValue);owned!(output,T);owned!(error,JsonError);Ok(fixed())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
  let mut work=0;let result=(||{
   if !self.closing||!retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result}
   if let Some(receiver)=self.owners.receiver.as_mut(){if receiver.terminal_is_empty(){self.owners.receiver=None;work=1;return Ok(RetainedCloneStep::Progress(unit()))}let turn=receiver.close_step(grant);work=turn.work;return turn.result}
   if let Some(admission)=self.owners.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);work=turn.work;let terminal=admission.terminal_is_empty();if let Some((value,()))=returned{self.owners.parsed=Some(value)}if terminal{self.owners.admission=None}return turn.result}
   if let Some(source)=self.owners.source.as_mut(){if source.terminal_is_empty(){self.owners.source=None;work=1;return Ok(RetainedCloneStep::Progress(unit()))}let turn=source.close_step(grant);work=turn.work;return turn.result}
   macro_rules! owned{($field:ident)=>{if self.owners.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.owners.$field,grant);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}}}
   owned!(parser);owned!(parsed);owned!(output);owned!(error);Ok(RetainedCloneStep::Complete(Default::default()))
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_empty()&&self.owners.parser.is_none()&&self.owners.parsed.is_none()&&self.owners.admission.is_none()&&self.owners.source.is_none()&&self.owners.receiver.is_none()&&self.owners.output.is_none()&&self.owners.error.is_none()}
}
impl<T:NativeReceive<DslValue>> Drop for JsonField<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original composed JSON field must close its grammar and receiving custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}


#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod tests;
