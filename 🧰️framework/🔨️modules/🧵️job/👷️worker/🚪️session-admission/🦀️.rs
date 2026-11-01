//! 🚪️ One original caller owns every real session birth and its distinct output handoff.
use super::*;
use semio_framework_job_worker_authority::OriginalWorkerJobAdmission;
use semio_framework_value::RetainedWorkOutcome;
mod sealed{pub trait Sealed{}}
pub trait AdmittedWorkerJobSession<J:InteractiveJob+'static>:Sized+sealed::Sealed{
 fn from_session(session:WorkerJobSession<J>)->Self;
 fn begin_close(&mut self);
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
}
impl<J:InteractiveJob+'static> sealed::Sealed for WorkerJobSession<J>{}
impl<J:InteractiveJob+'static> AdmittedWorkerJobSession<J> for WorkerJobSession<J>{
 fn from_session(session:WorkerJobSession<J>)->Self{session}
 fn begin_close(&mut self){let _=WorkerJobSession::begin_close(self);}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{WorkerJobSession::close_step(self,cx)}
 fn terminal_is_empty(&self)->bool{WorkerJobSession::terminal_is_empty(self)}
}
impl<J:InteractiveJob+'static> sealed::Sealed for MountedWorkerJobSession<J>{}
impl<J:InteractiveJob+'static> AdmittedWorkerJobSession<J> for MountedWorkerJobSession<J>{
 fn from_session(session:WorkerJobSession<J>)->Self{Self{session,ticket:None,checked_out:None}}
 fn begin_close(&mut self){MountedWorkerJobSession::begin_close(self)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{MountedWorkerJobSession::close_step(self,cx)}
 fn terminal_is_empty(&self)->bool{MountedWorkerJobSession::terminal_is_empty(self)}
}
impl<J:InteractiveJob+'static> sealed::Sealed for BatchJobSession<J>{}
impl<J:InteractiveJob+'static> AdmittedWorkerJobSession<J> for BatchJobSession<J>{
 fn from_session(session:WorkerJobSession<J>)->Self{Self{session,ticket:None,checked_out:None}}
 fn begin_close(&mut self){BatchJobSession::begin_close(self)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{BatchJobSession::close_step(self,cx)}
 fn terminal_is_empty(&self)->bool{BatchJobSession::terminal_is_empty(self)}
}
pub struct WorkerJobSessionAdmission<J:InteractiveJob+'static,S:AdmittedWorkerJobSession<J>>{
 admission:OriginalWorkerJobAdmission<J>,authority:Option<OriginalWorkerJobAuthority<J>>,born:Option<S>,frame:Option<session_return::SessionHandle<J>>,node:Option<Box<WorkerJobRetirementNode<J>>>,slot:Option<usize>,scan:usize,binding:Option<OriginalSourceBinding>,stage:u8,closing:bool,
}
impl<J:InteractiveJob+'static,S:AdmittedWorkerJobSession<J>> Default for WorkerJobSessionAdmission<J,S>{fn default()->Self{Self::new()}}
impl<J:InteractiveJob+'static,S:AdmittedWorkerJobSession<J>> WorkerJobSessionAdmission<J,S>{
 pub const fn new()->Self{Self{admission:OriginalWorkerJobAdmission::new(),authority:None,born:None,frame:None,node:None,slot:None,scan:0,binding:None,stage:0,closing:false}}
 pub fn terminal_is_empty(&self)->bool{self.authority.is_none()&&self.born.is_none()&&self.frame.is_none()&&self.node.is_none()&&self.slot.is_none()&&self.binding.is_none()&&!self.closing&&self.admission.terminal_is_empty()}
 pub fn phase(&self)->&'static str{if self.closing{return "returning"}match self.stage{0=>"source",1=>"authority",3=>"slot",4=>"frame",5=>"node",6=>"handoff",7=>"admissionClose",8=>"output",_=>"empty"}}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session birth requires its same original caller source").into())}Ok(())}
 fn enter(cx:&StepContext<'_>,capacity:usize,release:usize,closing:bool)->bool{let g=cx.retained_grant();!(if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})&&g.maximum_items>0&&g.maximum_depth>0&&g.maximum_capacity_bytes>=capacity&&g.maximum_release_bytes>=release}
 fn post(cx:&mut StepContext<'_>,p:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(p)),work:1})?;Ok(())}
 fn metadata(cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{Self::post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})}
 pub fn advance(&mut self,job:&mut Option<J>,params:&mut Option<BatchJobParams>,output:&mut Option<S>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.stage==9&&self.terminal_is_empty(){return Ok(output.is_some())}self.validate(cx)?;
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session birth cannot advance during paid close").into())}
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session receiver already owns its output").into())}
  if !Self::enter(cx,0,0,false){return Ok(false)}
  match self.stage{
   0=>{self.binding=Some(OriginalSourceBinding::borrow(cx));self.stage=1;Self::metadata(cx)?;}
   1=>{if self.authority.is_some(){self.stage=3;Self::metadata(cx)?;}else if self.admission.ready(){self.admission.take_ready(&mut self.authority,cx)?;}else{self.admission.advance(job,params,cx)?;}}
   3=>{let index=self.scan;let admitted=retirement_slots_and_nodes::reserve_worker_job_retirement_slot_at(index);if admitted{self.slot=Some(index);self.stage=4;}else{self.scan=(index+1)%WORKER_JOB_SESSION_SLOTS;}Self::metadata(cx)?;}
   4=>{let generation=self.authority.as_ref().ok_or_else(||ValueError::invariant("authority absent"))?.source_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"session frame requires its retained ready authority"))?.1;let turn=session_return::SessionHandle::admit_empty(generation,cx.retained_grant());let result=turn.result.map(|born|match born{Some((frame,p))=>{self.frame=Some(frame);self.stage=5;RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())});receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;}
   5=>{let bytes=std::mem::size_of::<WorkerJobRetirementNode<J>>();if !Self::enter(cx,bytes,0,false){return Ok(false)}let layout=std::alloc::Layout::new::<WorkerJobRetirementNode<J>>();let raw=unsafe{std::alloc::alloc(layout)}.cast::<WorkerJobRetirementNode<J>>();if raw.is_null(){receive_original_retained_work(cx,RetainedWorkOutcome{result:Err::<RetainedCloneStep,_>(ValueError::literal(ValueRefusalKind::AllocationFailed,"worker retirement node allocation refused")),work:1})?;}else{unsafe{raw.write(WorkerJobRetirementNode{header:WorkerJobRetirementHeader{slot:self.slot.ok_or_else(||ValueError::invariant("slot absent"))?,phase:worker_job_retirement_node_phase::<J>,pump:pump_worker_job_retirement_node::<J>,destroy:destroy_worker_job_retirement_node::<J>},inner:None});self.node=Some(Box::from_raw(raw));}self.stage=6;Self::post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;}}
   6=>{let frame=self.frame.take().ok_or_else(||ValueError::invariant("frame absent"))?;let generation=frame.generation;unsafe{*frame.authority.get()=Some(self.authority.take().ok_or_else(||ValueError::invariant("authority absent"))?);}frame.phase.store(SESSION_IDLE,Ordering::Release);let session=WorkerJobSession{inner:WorkerJobSessionArc(Some(frame)),generation,terminal_step_end_us:None,retirement:std::cell::UnsafeCell::new(self.node.take()),retirement_state:AtomicU8::new(0)};self.born=Some(S::from_session(session));self.slot=None;self.stage=7;Self::metadata(cx)?;}
   7=>{if self.admission.terminal_is_empty(){self.stage=8;Self::metadata(cx)?;}else{self.admission.close_step(cx)?;}}
   8=>{*output=self.born.take();self.binding=None;self.stage=9;Self::metadata(cx)?;return Ok(true)}
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session birth has an unknown retained phase").into()),
  }Ok(false)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let pending=|p|InteractiveJobCloseStep::Pending{progress:p};if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}self.validate(cx)?;if !Self::enter(cx,0,0,true){return Ok(pending(Default::default()))}
  if !self.closing{if let Some(born)=self.born.as_mut(){born.begin_close();}self.closing=true;Self::metadata(cx)?;return Ok(pending(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(born)=self.born.as_mut(){if !born.terminal_is_empty(){return born.close_step(cx).map(|step|match step{WorkerJobCloseStep::Pending{progress}|WorkerJobCloseStep::Complete{progress}=>pending(progress),WorkerJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked})}self.born.take();Self::metadata(cx)?;return Ok(pending(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(authority)=self.authority.as_mut(){if !authority.terminal_is_empty(){if authority.has_original_cancel_alias_witness(cx.original_cancel_token()){let turn=authority.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant());let blocked=matches!(&turn.result,Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)));let result=match turn.result{Ok(Some(step))=>Ok(step),Ok(None)|Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)};let step=receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;return Ok(if blocked{InteractiveJobCloseStep::Blocked}else{pending(step.progress())})}return authority.close_step(cx).map(|step|match step{WorkerJobCloseStep::Pending{progress}|WorkerJobCloseStep::Complete{progress}=>pending(progress),WorkerJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked})}self.authority.take();Self::metadata(cx)?;return Ok(pending(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if self.frame.is_some(){let bytes=session_return::SessionHandle::<J>::frame_bytes();if !Self::enter(cx,0,bytes,true){return Ok(pending(Default::default()))}let frame=self.frame.take().ok_or_else(||ValueError::invariant("frame absent"))?;if !frame.is_unique(){self.frame=Some(frame);return Ok(InteractiveJobCloseStep::Blocked)}match frame.try_return(){Ok(empty)=>drop(empty),Err(frame)=>{self.frame=Some(frame);receive_original_retained_work(cx,RetainedWorkOutcome{result:Err::<RetainedCloneStep,_>(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"empty session frame still lends its original handle")),work:1})?;return Ok(InteractiveJobCloseStep::Blocked)}}let p=RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()};Self::post(cx,p)?;return Ok(pending(p))}
  if let Some(node)=self.node.as_ref(){let bytes=size_of_val(node.as_ref());if !Self::enter(cx,0,bytes,true){return Ok(pending(Default::default()))}drop(self.node.take());let p=RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()};Self::post(cx,p)?;return Ok(pending(p))}
  if let Some(slot)=self.slot{if !retirement_slots_and_nodes::release_reserved_worker_job_retirement_slot(slot){receive_original_retained_work(cx,RetainedWorkOutcome{result:Err::<RetainedCloneStep,_>(ValueError::literal(ValueRefusalKind::InvariantViolated,"session birth lost its exact reserved retirement slot")),work:1})?;return Ok(InteractiveJobCloseStep::Blocked)}self.slot=None;Self::metadata(cx)?;return Ok(pending(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if !self.admission.terminal_is_empty(){return self.admission.close_step(cx)}
  self.binding=None;self.closing=false;self.stage=9;Self::metadata(cx)?;Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})
 }
}
impl<J:InteractiveJob+'static,S:AdmittedWorkerJobSession<J>> Drop for WorkerJobSessionAdmission<J,S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"worker session admission reached Drop before its same-source paid close")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
