//! 📬️ The exact external completion is parsed and bound before returning to its original finish cursor.
use super::*;
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy};
use semio_framework_value::native_decoding::NativeDecodeContinuation;
use semio_framework_value::NativeDecodeControl;

pub(super) struct EvaluationExternalCompletion {
 pub(super) original:Option<EvaluationExternalResult>,parser:Option<JsonGrammarCursor<DslValue>>,continuation:Option<NativeDecodeContinuation>,
 binding:Option<neural_engine::retirement::RetainedDictionaryInput>,dictionary:Option<Dictionary>,active:Option<Box<dyn ErasedSnapshotRetirement>>,stage:u8,normal_progress:RetainedCloneProgress,
}
impl EvaluationExternalCompletion {
 pub(super) fn new(original:EvaluationExternalResult)->Self{Self{original:Some(original),parser:Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject)),continuation:None,binding:None,dictionary:None,active:None,stage:0,normal_progress:Default::default()}}
 pub(super) fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_progress}
 pub(super) fn step(&mut self,grant:RetainedCloneGrant)->Result<Option<Dictionary>,EvaluationFailure>{
  self.normal_progress=Default::default();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None);}
  if self.stage==0{
   let mut accepted=|_|true;let mut control=match self.continuation.take(){Some(original)=>NativeDecodeControl::resume(original,&mut accepted)?,None=>NativeDecodeControl::new_retained(&mut accepted)};
   let result=self.parser.as_mut().unwrap().step(&self.original.as_ref().unwrap().output_json,1,&mut control,grant);self.normal_progress=self.parser.as_ref().unwrap().normal_step_progress();self.continuation=Some(control.pause().map_err(|error|error.with_retained_progress(self.normal_progress))?);
   if let Some(value)=result?{self.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(value));self.stage=1;}return Ok(None);
  }
  if self.stage==1{
   let binding=self.binding.as_mut().unwrap();let result=binding.step(grant).map_err(|error|{self.normal_progress=error.retained_progress();error})?;self.normal_progress=result.progress;
   if let Some(dictionary)=result.dictionary{assert!(binding.terminal_is_empty());self.binding=None;self.dictionary=Some(dictionary);self.stage=2;}return Ok(None);
  }
  if self.active.is_some()||self.parser.is_some()||self.original.is_some()||self.continuation.is_some(){let step=self.close_sources(grant).map_err(|error|{self.normal_progress=error.retained_progress();error})?;self.normal_progress=step.progress();return Ok(None);}
  self.stage=3;self.normal_progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(self.dictionary.take())
 }
 fn close_sources(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
  if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant);}
  if self.original.is_some(){return evaluation_admit_original(&mut self.original,&mut self.active,grant);}
  if self.continuation.is_some(){self.continuation=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub(super) fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(binding)=&self.binding{return Ok(RetirementDemand{copy_bytes:binding.next_close_copy_byte_demand()?,capacity_bytes:binding.next_close_capacity_byte_demand(copy)?,release_bytes:binding.next_close_release_byte_demand()?,depth:binding.next_close_depth_demand()?});}
  if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
  if self.parser.is_some(){return Ok(evaluation_owned_birth::<JsonGrammarCursor<DslValue>>());}
  if self.original.is_some(){return Ok(evaluation_owned_birth::<EvaluationExternalResult>());}
  if self.dictionary.is_some(){return Ok(evaluation_owned_birth::<Dictionary>());}
  Ok(RetirementDemand{depth:usize::from(self.continuation.is_some()),..Default::default()})
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(binding)=&mut self.binding{binding.cancel();let step=binding.close_step(grant)?;if binding.terminal_is_empty(){self.binding=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.active.is_some()||self.parser.is_some()||self.original.is_some()||self.continuation.is_some(){return self.close_sources(grant);}
  if self.dictionary.is_some(){return evaluation_admit_original(&mut self.dictionary,&mut self.active,grant);}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.parser.is_none()&&self.continuation.is_none()&&self.binding.is_none()&&self.dictionary.is_none()&&self.active.is_none()}
}
impl Drop for EvaluationExternalCompletion{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original external result dropped before terminal-empty");}}
