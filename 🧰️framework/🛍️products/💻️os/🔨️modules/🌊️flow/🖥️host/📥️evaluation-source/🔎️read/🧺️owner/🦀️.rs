//! 🧺️ The funded heap owner of one read admission: its allocation is born and released against receipts, so no caller stores the large frontier inline.
use super::*;
const BYTES:usize=size_of::<FlowSourceRead>();
/// 🧺️ One boxed read admission whose birth capacity and release bytes are exact receipts of the original caller.
pub struct FlowSourceReadOwner{read:Option<Box<FlowSourceRead>>,progress:RetainedCloneProgress}
impl FlowSourceReadOwner{
 /// 📏️ The one admitted item that constructs the owner; capacity is the exact allocation.
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:BYTES,depth:1,..Default::default()}}
 pub fn admit(revision:[u8;32],cx:&mut StepContext<'_>)->Result<Option<Self>,ValueError>{
  if cx.is_cancelled()||cx.should_yield()||!cx.retained_work_can_enter(Self::birth_demand()){return Ok(None)}
  let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:BYTES,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);
  Ok(Some(Self{read:Some(Box::new(FlowSourceRead::new(revision))),progress}))
 }
 fn inner(&self)->Result<&FlowSourceRead,ValueError>{self.read.as_deref().ok_or_else(||refusal("original read owner is released"))}
 fn inner_mut(&mut self)->Result<&mut FlowSourceRead,ValueError>{self.read.as_deref_mut().ok_or_else(||refusal("original read owner is released"))}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn next_demands(&self,snapshot:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{self.inner()?.next_demands(snapshot,copy)}
 pub fn step(&mut self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,cx:&mut StepContext<'_>)->Result<FlowSourceReadStep,ValueError>{
  let read=self.inner_mut()?;let step=read.step(snapshot,revision,session,request,cx);self.progress=read.step_progress();step
 }
 pub fn admit_current_output<'a>(&'a mut self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,cx:&mut StepContext<'_>)->Result<FlowCurrentOutputRead<'a>,ValueError>{
  let read=self.read.as_deref_mut().ok_or_else(||refusal("original read owner is released"))?;read.admit_current_output(snapshot,revision,session,request,cx)
 }
 pub fn current_output<'a>(&'a self,request:&FlowSourceReadRequest<'_>)->Option<FlowCurrentOutput<'a>>{self.read.as_deref()?.current_output(request)}
 pub fn begin_close(&mut self){if let Some(read)=self.read.as_deref_mut(){read.begin_close()}}
 /// 🧾️ Close quotes the inner custody plus the exact allocation release.
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  let Some(read)=self.read.as_deref()else{return Ok(Default::default())};let mut demand=read.next_close_demands(copy)?;demand.release_bytes=demand.release_bytes.checked_add(BYTES).ok_or_else(||refusal("original read owner release overflows"))?;demand.depth=demand.depth.max(1);Ok(demand)
 }
 pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{match self.read.as_deref(){Some(read)=>read.next_close_work(body),None=>Ok(0)}}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let grant=effect.grant();
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if self.read.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   if !admitted(self.next_close_demands(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let read=self.read.as_deref_mut().ok_or_else(||refusal("original read owner is released"))?;let step=RetirementPollExit::resume(read.close_step(body,effect))?;let mut progress=step.progress();
   if read.terminal_is_empty(){self.read=None;progress.released_bytes=progress.released_bytes.checked_add(BYTES).ok_or_else(||refusal("original read owner release overflows"))?;self.progress=progress;return Ok(RetainedCloneStep::Complete(progress))}
   self.progress=progress;Ok(RetainedCloneStep::Progress(progress))
  })();
  settle_close(effect,result,&mut self.progress)
 }
 pub fn terminal_is_empty(&self)->bool{self.read.is_none()}
}
