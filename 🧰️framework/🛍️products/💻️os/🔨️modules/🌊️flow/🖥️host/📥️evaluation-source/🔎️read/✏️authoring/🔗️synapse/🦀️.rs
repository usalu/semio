//! 🔗️ Original synapse text fields are copied individually before the mutation handoff.
use super::text::FundedText;
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_framework_job::{OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original synapse fields changed their retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub struct SynapseWork{fields:ManuallyDrop<[Option<String>;5]>,output:ManuallyDrop<Option<SynapseSpec>>,text:Option<FundedText>,retirement:RetainedCloneClose,slot:usize,phase:u8,closing:bool}
impl SynapseWork{
 pub fn new()->Self{Self{fields:ManuallyDrop::new([None,None,None,None,None]),output:ManuallyDrop::new(None),text:None,retirement:Default::default(),slot:0,phase:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub fn next_demands(&self,fields:[&str;5],copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()+size_of::<usize>()))}else if self.fields[self.slot].is_some(){text.close_demands(copy)}else{text.next_demands(fields[self.slot],copy)}}Ok(fixed(if self.phase==0&&self.slot==5{size_of::<SynapseSpec>()+size_of::<[Option<String>;5]>()}else if self.phase==1{size_of::<Option<SynapseSpec>>()}else{size_of::<FundedText>()}))}
 pub fn step(&mut self,fields:[&str;5],cx:&mut StepContext<'_>)->Result<Option<SynapseSpec>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}match self.phase{
   0=>{if self.slot==5{if Self::paid(cx,size_of::<SynapseSpec>()+size_of::<[Option<String>;5]>())?{*self.output=Some(SynapseSpec{id:self.fields[0].take().ok_or_else(invalid)?,from:self.fields[1].take().ok_or_else(invalid)?,to:self.fields[2].take().ok_or_else(invalid)?,from_port:self.fields[3].take().ok_or_else(invalid)?,to_port:self.fields[4].take().ok_or_else(invalid)?});self.phase=1}return Ok(None)}
    if self.text.is_none(){if Self::paid(cx,size_of::<FundedText>())?{self.text=Some(FundedText::new())}return Ok(None)}
    let text=self.text.as_mut().unwrap();if self.fields[self.slot].is_none(){if text.ready(){text.take_into(&mut self.fields[self.slot],cx)?;if self.fields[self.slot].is_some(){text.begin_close()}}else{text.advance(fields[self.slot],cx)?;}return Ok(None)}
    if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<FundedText>>()+size_of::<usize>())?{self.text=None;self.slot+=1}},
   1=>if Self::paid(cx,size_of::<Option<SynapseSpec>>())?{self.phase=2;return Ok(self.output.take())},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:SynapseSpec){assert_eq!(self.phase,2);*self.output=Some(output);self.phase=1}
 pub fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<SynapseSpec>(true,&None,copy)}self.retirement.next_owner_demands::<String>(self.fields.iter().any(Option::is_some),&None,copy)}
 pub fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if let Some(text)=self.text.as_mut(){if text.terminal_is_empty(){let d=fixed(size_of::<Option<FundedText>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.text=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}return text.close_step(g)}
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}
  if let Some(field)=self.fields.iter_mut().find(|field|field.is_some()){let turn=self.retirement.begin_granted(field,effect);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.fields.iter().all(Option::is_none)&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for SynapseWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original synapse text requires terminal custody")}}
