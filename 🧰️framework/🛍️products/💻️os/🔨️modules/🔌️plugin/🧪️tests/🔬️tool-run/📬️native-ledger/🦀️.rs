//! 🧪️ The mounted read-only fixture publishes real canonical preview pages before the next producer step.
use super::*;
use std::mem::size_of;
use semio_framework_job::{JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,OriginalTurnFailure,InteractiveJobCloseStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
struct Source{writer:ToolRunTickWriter,progress:Option<ToolRunProgress>,counter:Option<ToolRunCounter>}
semio_framework_value::artifact_retire_struct!(Source{writer,progress,counter});
pub struct OriginalReadonlyPreview{source:std::mem::ManuallyDrop<Source>,close:Option<ControlledRetirement<Source>>,emission:OriginalToyEmission,index:usize,position:usize,phase:u8,closing:bool,source_live:bool}
impl OriginalReadonlyPreview{
 pub fn new(identity:ToolRunIdentity)->Self{Self{source:std::mem::ManuallyDrop::new(Source{writer:ToolRunTickWriter::new(identity),progress:Some(ToolRunProgress{identity,sequence:0,state:ToolRunState::Running,stage:0,completed:8760,total:Some(8760),counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()}),counter:None}),close:None,emission:OriginalToyEmission::new(),index:0,position:0,phase:0,closing:false,source_live:true}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.emission.terminal_is_empty(){return self.emission.retirement_demands()}if let Some(close)=self.close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{close.selected_demand(body)}}Ok(RetirementDemand{copy_bytes:if self.source_live{size_of::<Source>()+size_of::<ControlledRetirement<Source>>()}else{0},depth:usize::from(self.source_live),..Default::default()})}
}
impl semio_framework_job::InteractiveJob for OriginalReadonlyPreview{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  let grant=cx.retained_grant();if cx.should_yield(){return Ok(None)}
  match self.phase{
   0=>{if self.index==9{self.phase=1;return Ok(None)}if self.source.counter.is_none(){let d=RetirementDemand{copy_bytes:size_of::<ToolRunCounter>(),depth:1,..Default::default()};if cx.retained_work_can_enter(d){self.source.counter=Some(ToolRunCounter{counter:self.index as u16,value:[0,8760,2,0,0,4358948,6067258,1600000,8760][self.index]});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);}return Ok(None)}let source=&mut*self.source;let step=ToolRunTickWriter::admit_original_progress_counter(source.progress.as_mut().unwrap(),&mut source.counter,grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if matches!(step,RetainedCloneStep::Complete(_)){self.index+=1;}}
   1=>{let source=&mut*self.source;let step=source.writer.admit_original_progress(&mut source.progress,grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;}}
   2=>{let result=self.source.writer.admit_original_payload(&[17,23,41,67],&mut self.position,grant)?;cx.consume_retained(result.progress())?;cx.consume_fuel(result.progress().copied_items as u64);if self.position==4{self.phase=3;}}
   3=>{let result=self.emission.advance_preview(&self.source.writer,cx)?;if result.is_some(){self.phase=4;}return Ok(result)}
   4=>{if self.emission.close_in_context(cx)?{self.phase=5;}}
   _=>return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into),
  }Ok(None)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{self.emission.borrow_outcome(descriptor)}
 fn begin_close(&mut self){self.closing=true;}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(4096)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(4096)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(4096)?.depth)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let d=self.demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|grant|{if !self.emission.terminal_is_empty(){return self.emission.close_step(grant)}if let Some(close)=self.close.as_mut(){if !close.terminal_is_empty(){return close.step(grant)}self.close=None;}else if self.source_live{let source=unsafe{std::mem::ManuallyDrop::take(&mut self.source)};match ControlledRetirement::new(source){Ok(close)=>{self.close=Some(close);self.source_live=false;},Err((error,source))=>{self.source=std::mem::ManuallyDrop::new(source);return Err(error)}}}else{return Ok(RetainedCloneStep::Complete(Default::default()))}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})}
 fn terminal_is_empty(&self)->bool{self.closing&&!self.source_live&&self.close.is_none()&&self.emission.terminal_is_empty()}
}
impl Drop for OriginalReadonlyPreview{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original read-only fixture requires paid close");}}
