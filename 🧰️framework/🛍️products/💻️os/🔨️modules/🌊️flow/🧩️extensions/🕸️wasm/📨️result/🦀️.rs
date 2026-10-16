//! 📬️ The exact typed external completion is bound before returning to its original finish cursor.
use super::*;

pub struct EvaluationExternalCompletion {
 pub original:Option<EvaluationExternalResult>,
 pub binding:Option<neural_engine::retirement::RetainedDictionaryInput>,dictionary:Option<Dictionary>,active:Option<Box<dyn ErasedSnapshotRetirement>>,stage:u8,normal_progress:RetainedCloneProgress,
}
impl EvaluationExternalCompletion {
 pub(super) fn new(original:EvaluationExternalResult)->Self{Self{original:Some(original),binding:None,dictionary:None,active:None,stage:0,normal_progress:Default::default()}}
 pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_progress}
 pub(super) fn step(&mut self,grant:RetainedCloneGrant)->Result<Option<Dictionary>,EvaluationFailure>{
  self.normal_progress=Default::default();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None);}
  if self.stage==0{
   let output=std::mem::replace(&mut self.original.as_mut().unwrap().output,DslValue::Null);self.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(output));self.normal_progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.stage=1;return Ok(None);
  }
  if self.stage==1{
   let binding=self.binding.as_mut().unwrap();let result=binding.step(grant).map_err(|error|{self.normal_progress=error.retained_progress();error})?;self.normal_progress=result.progress;
   if let Some(dictionary)=result.dictionary{assert!(binding.terminal_is_empty());self.binding=None;self.dictionary=Some(dictionary);self.stage=2;}return Ok(None);
  }
  if self.active.is_some()||self.original.is_some(){let step=self.close_sources(grant).map_err(|error|{self.normal_progress=error.retained_progress();error})?;self.normal_progress=step.progress();return Ok(None);}
  self.stage=3;self.normal_progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(self.dictionary.take())
 }
 fn close_sources(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
  if self.original.is_some(){return evaluation_admit_original(&mut self.original,&mut self.active,grant);}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(binding)=&self.binding{return Ok(RetirementDemand{copy_bytes:binding.next_close_copy_byte_demand()?,capacity_bytes:binding.next_close_capacity_byte_demand(copy)?,release_bytes:binding.next_close_release_byte_demand()?,depth:binding.next_close_depth_demand()?});}
  if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
  if self.original.is_some(){return Ok(evaluation_owned_birth::<EvaluationExternalResult>());}
  if self.dictionary.is_some(){return Ok(evaluation_owned_birth::<Dictionary>());}
  Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(binding)=&mut self.binding{binding.cancel();let step=binding.close_step(grant)?;if binding.terminal_is_empty(){self.binding=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.active.is_some()||self.original.is_some(){return self.close_sources(grant);}
  if self.dictionary.is_some(){return evaluation_admit_original(&mut self.dictionary,&mut self.active,grant);}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.binding.is_none()&&self.dictionary.is_none()&&self.active.is_none()}
}
impl Drop for EvaluationExternalCompletion{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original external result dropped before terminal-empty");}}
