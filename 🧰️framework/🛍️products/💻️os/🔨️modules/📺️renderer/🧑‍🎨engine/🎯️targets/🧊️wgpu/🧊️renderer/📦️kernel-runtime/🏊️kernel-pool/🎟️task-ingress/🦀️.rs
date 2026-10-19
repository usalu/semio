//! 🎟️ Persistent finite-task inputs and born candidates retain their authentic producer until exact close.
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,OriginalCallerSourceLoan,OriginalSourceBinding,OriginalConstructionDemand,OriginalTurnFailure,StepContext,admit_original_construction,close_original_retained_step};
use semio_framework_value::{NativeDecodeControl,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};

/// 📥️ A concrete producer input declares genuine bounded close instead of erasing opaque Future captures.
pub(crate) trait OriginalKernelTaskInput:Send+'static{
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
}

/// 🪴️ Inline caller custody precedes source admission and every constructor-owned allocation.
pub(crate) struct OriginalKernelTaskProducer<P:OriginalKernelTaskInput,J:InteractiveJob+'static>{input:ManuallyDrop<Option<P>>,candidate:ManuallyDrop<Option<J>>,source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,binding:Option<OriginalSourceBinding>,committed:bool,failed:bool,closing:bool}
fn refused(detail:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,detail).into()}
impl<P:OriginalKernelTaskInput,J:InteractiveJob+'static> OriginalKernelTaskProducer<P,J>{
 /// 🎫️ The initiating caller keeps its inline input slot and actual source alive until admission or original cleanup.
 pub(crate) fn new(input:P,cx:&StepContext<'_>)->Self{Self::from_binding(input,OriginalSourceBinding::borrow(cx))}
 /// 🪪️ Inline metadata captured before entering the actual decoder constructor grants no construction or execution authority.
 pub(crate) fn from_binding(input:P,binding:OriginalSourceBinding)->Self{Self{input:ManuallyDrop::new(Some(input)),candidate:ManuallyDrop::new(None),source:ManuallyDrop::new(None),binding:Some(binding),committed:false,failed:false,closing:false}}
 pub(crate) fn input(&self)->Option<&P>{self.input.as_ref()}
 pub(crate) fn candidate(&self)->Option<&J>{self.candidate.as_ref()}
 pub(crate) fn committed(&self)->bool{self.committed&&!self.failed&&!self.closing}
 /// 🏭️ The infallible constructor takes original input only inside its actual receiving gate; failed receipts keep the born candidate.
 pub(crate) fn admit(&mut self,cx:&mut StepContext<'_>,demand:OriginalConstructionDemand,construct:impl FnOnce(P,&mut NativeDecodeControl<'_>)->(J,RetainedCloneProgress))->Result<bool,OriginalTurnFailure>{
  if self.closing||self.failed{return Err(refused("finite task producer is closing or retains a refused constructor"))}self.validate_binding(cx)?;
  if self.source.is_none(){
   if cx.is_cancelled()||cx.should_yield()||!demand.fits(cx.retained_grant()){return Ok(false)}
   let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};*self.source=Some(source);return Ok(false)
  }
  self.validate(cx,false)?;if self.committed{return Ok(true)}if self.candidate.is_some()||self.input.is_none(){return Err(refused("finite task producer lost its original constructor frontier"))}
  let candidate=&mut*self.candidate;let input=&mut*self.input;
  match admit_original_construction(cx,candidate,demand,|control|{let(job,progress)=construct(input.take().unwrap(),control);Ok((job,progress))}){Ok(true)=>{self.committed=true;Ok(true)},Ok(false)=>Ok(false),Err(error)=>{self.failed=true;Err(error)}}
 }
 fn validate_binding(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.binding.is_none_or(|binding|!binding.matches(cx)){return Err(refused("finite task producer refused another original source or clock"))}Ok(())}
 fn validate(&self,cx:&StepContext<'_>,retirement:bool)->Result<(),OriginalTurnFailure>{
  self.validate_binding(cx)?;let source=self.source.as_ref().ok_or_else(||refused("finite task producer lost its original source loan"))?;if retirement{source.validate_retirement_context(cx)?;}else{source.validate_context(cx)?;}Ok(())
 }
 /// ▶️ Only a receipted candidate runs, borrowing its actual unchanged original caller.
 pub(crate) fn run<R>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut J,&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.validate(cx,false)?;if !self.committed(){return Err(refused("finite task candidate is not receipted for execution"))}run(self.candidate.as_mut().unwrap(),cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(candidate)=self.candidate.as_mut(){candidate.begin_close();}}
 /// ♻️ Input or candidate closes before returning the genuine source; a child Complete is never whole-producer completion.
 pub(crate) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}self.validate_binding(cx)?;if self.source.is_some(){self.validate(cx,true)?;}self.begin_close();
  if let Some(input)=self.input.as_mut(){if !input.terminal_is_empty(){let step=input.close_step(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}let demand=RetirementDemand{copy_bytes:size_of::<Option<P>>(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{self.input.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}
  if let Some(candidate)=self.candidate.as_mut(){if !candidate.terminal_is_empty(){let step=candidate.close_step(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}let demand=RetirementDemand{copy_bytes:size_of::<Option<J>>(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{self.candidate.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}
  if let Some(source)=self.source.as_mut(){let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){self.source.take();}return Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})}Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.candidate.is_none()&&self.source.is_none()}
}
impl<P:OriginalKernelTaskInput,J:InteractiveJob+'static> Drop for OriginalKernelTaskProducer<P,J>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"finite task producer requires original paid input/candidate/source close");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
