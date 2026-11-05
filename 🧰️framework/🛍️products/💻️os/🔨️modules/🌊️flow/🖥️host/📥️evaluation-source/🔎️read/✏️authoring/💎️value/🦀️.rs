//! 💎️ Actual neural defaults retain immutable roots and copy authored text with bounded physical receipts.
use neural_engine::{Value as NeuralValue,Atom};
use super::text::FundedText;
use semio_framework_job::{OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original authoring default value changed its borrowed source")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
pub(super) struct ValueCopy{source:Option<usize>,output:ManuallyDrop<Option<NeuralValue>>,text_output:ManuallyDrop<Option<String>>,text:Option<FundedText>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl ValueCopy{
 pub(super) fn new()->Self{Self{source:None,output:ManuallyDrop::new(None),text_output:ManuallyDrop::new(None),text:None,retirement:Default::default(),phase:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);Ok(true)}
 pub(super) fn next_demands(&self,source:&NeuralValue,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){if self.phase==2{return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()+size_of::<NeuralValue>()))}else{text.close_demands(copy)}}let NeuralValue::Atom(Atom::String(source))=source else{return Err(invalid())};return text.next_demands(source,copy)}Ok(fixed(size_of::<Self>().max(size_of::<Option<NeuralValue>>())))}
 pub(super) fn step(&mut self,source:&NeuralValue,cx:&mut StepContext<'_>)->Result<Option<NeuralValue>,OriginalTurnFailure>{
  if self.closing||self.source.is_some_and(|original|original!=source as*const _ as usize){return Err(invalid().into())}
  match self.phase{
   0=>{if !Self::paid(cx,size_of::<Self>())?{return Ok(None)}self.source=Some(source as*const _ as usize);if matches!(source,NeuralValue::Atom(Atom::String(_))){self.text=Some(FundedText::new());self.phase=1}else{*self.output=Some(source.clone());self.phase=3}},
   1=>{let NeuralValue::Atom(Atom::String(text))=source else{return Err(invalid().into())};let cursor=self.text.as_mut().unwrap();if cursor.ready(){cursor.take_into(&mut self.text_output,cx)?;if self.text_output.is_some(){cursor.begin_close();self.phase=2}}else{cursor.advance(text,cx)?;}},
   2=>{let text=self.text.as_mut().unwrap();if !text.terminal_is_empty(){let demand=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|text.close_step(grant))?;return Ok(None)}if Self::paid(cx,size_of::<Option<FundedText>>()+size_of::<NeuralValue>())?{self.text=None;*self.output=Some(NeuralValue::Atom(Atom::String(self.text_output.take().ok_or_else(invalid)?)));self.phase=3}},
   3=>{if Self::paid(cx,size_of::<Option<NeuralValue>>())?{self.phase=4;return Ok(self.output.take())}},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,value:NeuralValue){assert_eq!(self.phase,4);*self.output=Some(value);self.phase=3}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<NeuralValue>(true,&None,copy)}if self.text_output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(Default::default())}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(text)=self.text.as_mut(){if text.terminal_is_empty(){let d=fixed(size_of::<Option<FundedText>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.text=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(d))),work:1}}return text.close_step(grant)}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  macro_rules! owner{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(output);owner!(text_output);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.output.is_none()&&self.text_output.is_none()&&self.retirement.is_empty()}
}
impl Drop for ValueCopy{fn drop(&mut self){assert!(self.terminal_is_empty(),"original authoring default value requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.text_output)}}}}
