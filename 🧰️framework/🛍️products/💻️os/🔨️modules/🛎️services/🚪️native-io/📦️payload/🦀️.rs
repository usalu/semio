//! 📦️ Native byte chunks retain their original copy cursor through funded payload assembly.
use semio_framework_job_payload::{JobPayloadStep,JobPayloadStream,RetainedJobPayload,RetainedPayloadBuilder};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub(super) struct NativePayloadPump{builder:RetainedPayloadBuilder,source:Option<(usize,usize)>,cursor:usize,finished:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native payload chunk changed its retained source or phase")}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl NativePayloadPump{
 pub(super) fn new(stream:JobPayloadStream)->Self{Self{builder:RetainedPayloadBuilder::new(stream),source:None,cursor:0,finished:false}}
 pub(super) fn copy_cursor(&self)->usize{self.cursor}
 pub(super) fn chunk_complete(&self)->bool{self.source.is_some_and(|(_,length)|self.cursor==length)}
 pub(super) fn append_demand(&self,bytes:&[u8],g:RetainedCloneGrant)->Result<RetirementDemand,ValueError>{
  if self.finished{return Err(invalid())}
  let Some((pointer,length))=self.source else{return Ok(RetirementDemand{depth:1,..Default::default()})};
  if pointer!=bytes.as_ptr()as usize||length!=bytes.len()||self.cursor>length{return Err(invalid())}
  if self.cursor==length{return Ok(Default::default())}
  if !self.builder.is_initialized(){return Ok(self.builder.initialization_demand())}
  self.builder.append_demand(g)
 }
 pub(super) fn append_next(&mut self,bytes:&[u8],cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  let g=cx.retained_grant();let d=match self.append_demand(bytes,g){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if cx.is_cancelled()||cx.should_yield()||!permits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.source.is_none(){self.source=Some((bytes.as_ptr()as usize,bytes.len()));return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  if self.cursor==bytes.len(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !self.builder.is_initialized(){let turn=self.builder.advance_initialization(cx);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  self.builder.append_original(cx,bytes,&mut self.cursor)
 }
 pub(super) fn clear_chunk_demand(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub(super) fn clear_chunk(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some((_,length))=self.source else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if self.cursor!=length{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if !permits(g,self.clear_chunk_demand()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.source=None;self.cursor=0;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub(super) fn finish_demand(&self)->RetirementDemand{if !self.builder.is_initialized(){self.builder.initialization_demand()}else{RetainedPayloadBuilder::seal_demand()}}
 pub(super) fn finish_next(&mut self,output:&mut Option<RetainedJobPayload>,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  if self.finished{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if self.source.is_some()||output.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if !self.builder.is_initialized(){let turn=self.builder.advance_initialization(cx);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  if self.builder.published().is_none(){let turn=self.builder.seal(cx);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  let turn=self.builder.take_finished(output,cx.retained_grant());if output.is_some(){self.finished=true}turn
 }
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if !self.builder.terminal_is_empty(){self.builder.retirement_demands()}else{Ok(RetirementDemand{depth:usize::from(self.source.is_some()),..Default::default()})}}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.builder.terminal_is_empty(){let turn=self.builder.close_step_granted(g);return RetainedWorkOutcome{result:turn.result.map(|step|if self.source.is_some(){RetainedCloneStep::Progress(step.progress())}else{step}),work:turn.work}}
  if self.source.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !permits(g,RetirementDemand{depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.source=None;self.cursor=0;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.builder.terminal_is_empty()}
}
impl Drop for NativePayloadPump{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native payload pump requires funded closure")}}
#[cfg(test)]
#[path="tests.rs"]
mod tests;
