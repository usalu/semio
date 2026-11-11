//! 🎟️ A preborn native carrier retains its actual return Origin until semantic and physical settlement.
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,InteractiveStage,JobOutcomeBorrow,JobOutcomeView,OriginalJobContinuation,OriginalJobContinuationView,OriginalJobOutcome,OriginalJobReturn,OriginalJobReturnOrigin,OriginalJobPageReservation,OriginalJobPageReturnRegistry,OriginalSourceBinding,OriginalSuppliedJobPage,OriginalTurnFailure,StepContext,close_original_retained_step,drive_step,receive_original_retained_work};
use semio_os_services_native_io::{NativeIoJob,NativeIoRequest,NativeIoResultStatus};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};

/// 🚦️ Semantic completion is distinct from a still-owned original continuation or cancellation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RendererOriginalIoStep{Pending,Complete,Cancelled,Continuation}

/// 🧳️ The enclosing Node owns this allocation-free carrier before any source or job is admitted.
pub struct RendererOriginalIoNode<const N:usize>{
 origin:OriginalJobReturnOrigin,
 binding:Option<OriginalSourceBinding>,
 source_identity:Option<usize>,
 job:Option<NativeIoJob>,
 returned_outcome:Option<OriginalJobOutcome>,
 returned_continuation:Option<OriginalJobContinuation>,
 terminal_seen:bool,
 acknowledged:bool,
 reservation:Option<OriginalJobPageReservation<N>>,
 returns:&'static OriginalJobPageReturnRegistry<N>,
 closing:bool,
}
fn invalid(detail:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,detail).into()}
fn control_allowed(cx:&StepContext<'_>,closing:bool)->bool{!(if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})&&cx.retained_work_can_enter(RetirementDemand{depth:1,..Default::default()})}
fn post_control(cx:&mut StepContext<'_>)->Result<RetainedCloneProgress,OriginalTurnFailure>{receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}).map(|step|step.progress())}
fn span(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let(Some(copied_items),Some(copied_bytes),Some(retained_capacity_bytes),Some(released_bytes))=(after.copied_items.checked_sub(before.copied_items),after.copied_bytes.checked_sub(before.copied_bytes),after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes),after.released_bytes.checked_sub(before.released_bytes))else{return Err(invalid("native original receiving receipt regressed"))};Ok(RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes})
}
impl<const N:usize> RendererOriginalIoNode<N>{
 /// 🪹️ A cold carrier creates no source, allocation, grant or work.
 pub const fn new(returns:&'static OriginalJobPageReturnRegistry<N>)->Self{Self{origin:OriginalJobReturnOrigin::empty(),binding:None,source_identity:None,job:None,returned_outcome:None,returned_continuation:None,terminal_seen:false,acknowledged:false,reservation:None,returns,closing:false}}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx))||self.source_identity.is_some_and(|identity|parent.source_identity().ok()!=Some(identity)){return Err(invalid("native invocation changed its original source or clock"))}self.origin.validate_close_context(cx)}
 /// 📥️ The actual Origin and its prepaid receiver are ready before the original request moves.
 pub fn admit(&mut self,request:&mut Option<NativeIoRequest>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing{return Err(invalid("closing native invocation cannot admit a request"))}
  if self.job.is_some(){return if request.is_none(){Ok(true)}else{Err(invalid("native invocation already retains its original request"))}}
  if request.is_none(){return Err(invalid("native invocation requires its original request"))}
  if !self.origin.body_ready(){let identity=cx.original_source_parent()?.source_identity()?;let binding=OriginalSourceBinding::borrow(cx);let result=self.origin.admit(cx);if !self.origin.terminal_is_empty()&&self.binding.is_none(){self.binding=Some(binding);self.source_identity=Some(identity)}result?;return Ok(false)}
  if !control_allowed(cx,false){return Ok(false)}self.job=Some(NativeIoJob::new(request.take().ok_or_else(||ValueError::invariant("request absent"))?));post_control(cx)?;Ok(true)
 }
 fn receive_return(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneProgress,OriginalTurnFailure>{let before=cx.retained_progress();if self.returned_continuation.is_some(){self.origin.receive_continuation(cx,&mut self.returned_continuation)?;}else if self.returned_outcome.is_some(){self.origin.receive_outcome(cx,&mut self.returned_outcome)?;}span(before,cx.retained_progress())}
 /// ▶️ A real preclaimed receiver captures and receives every original callback before semantic settlement.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<RendererOriginalIoStep,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing{return Err(invalid("closing native invocation cannot resume work"))}
  if self.returned_continuation.is_some()||self.returned_outcome.is_some(){self.receive_return(cx)?;return Ok(if self.origin.continuation().is_some(){RendererOriginalIoStep::Continuation}else{RendererOriginalIoStep::Pending})}
  if self.origin.continuation().is_some(){return Ok(RendererOriginalIoStep::Continuation)}
  if let Some(descriptor)=self.origin.outcome(){if !descriptor.is_acknowledged(){self.job.as_ref().ok_or_else(||invalid("native invocation lost its original job"))?.borrow_outcome(descriptor)?;}let done=self.origin.acknowledge_outcome(cx)?;if done&&self.terminal_seen{self.acknowledged=true;}return Ok(if self.acknowledged{RendererOriginalIoStep::Complete}else{RendererOriginalIoStep::Pending})}
  if self.acknowledged{return Ok(RendererOriginalIoStep::Complete)}if cx.is_cancelled(){return Ok(RendererOriginalIoStep::Cancelled)}
  if !self.origin.body_ready(){self.origin.admit(cx)?;return Ok(RendererOriginalIoStep::Pending)}
  let mut verdict=None;let returned=drive_step(self.job.as_mut().ok_or_else(||invalid("native invocation lost its original job"))?,cx,self.origin.receiver()?,"renderer.original-native-io",InteractiveStage::InteractiveStep,&mut verdict)?;
  match returned{OriginalJobReturn::Preparing|OriginalJobReturn::Pending=>{},OriginalJobReturn::Outcome(reply)=>{self.terminal_seen=matches!(reply.original(),Some(JobOutcomeBorrow::Complete{..}|JobOutcomeBorrow::Cancelled{..}|JobOutcomeBorrow::Fault{..}));self.returned_outcome=Some(reply.into_descriptor());},OriginalJobReturn::Continuation(value)=>self.returned_continuation=Some(value)}
  Ok(RendererOriginalIoStep::Pending)
 }
 /// 👁️ Complete callback and qualification causes remain borrowed from their actual retained carrier.
 pub fn borrow_continuation(&self)->Option<OriginalJobContinuationView<'_>>{self.origin.continuation().or(self.returned_continuation.as_ref()).map(OriginalJobContinuation::view)}
 /// 🔎️ Producer payloads remain borrowed until their exact descriptor is acknowledged.
 pub fn borrow_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{self.origin.outcome().filter(|descriptor|!descriptor.is_acknowledged()).map(|descriptor|self.job.as_ref().ok_or_else(||ValueError::invariant("original descriptor retains its producer"))?.borrow_outcome(descriptor)).transpose()}
 pub fn result_status(&self)->Option<NativeIoResultStatus>{self.acknowledged.then(||self.job.as_ref().and_then(NativeIoJob::result_status)).flatten()}
 pub fn request_backing_identity(&self)->Option<*const u8>{self.job.as_ref().and_then(NativeIoJob::retained_request_backing_identity)}
 pub fn producer_is_empty(&self)->bool{self.job.is_none()}
 pub fn outcome_is_acknowledged(&self)->bool{self.acknowledged&&self.origin.outcome().is_none()&&self.returned_outcome.is_none()}
 pub fn known_file_length(&self)->Option<u64>{self.job.as_ref().and_then(NativeIoJob::known_file_length)}
 pub fn result_page(&self,index:usize)->Option<&[u8]>{self.acknowledged.then(||self.job.as_ref().and_then(|job|job.result_page(index))).flatten()}
 pub fn result_modified(&self,index:usize)->Option<(&std::path::Path,std::time::SystemTime)>{self.acknowledged.then(||self.job.as_ref().and_then(|job|job.result_modified(index))).flatten()}
 /// 🧭️ The actual selected path moves only after the original descriptor settlement.
 pub fn supply_selected_path<const P:usize>(&mut self,reservation:&mut semio_os_services_native_io::OriginalNativePathReservation<P>,output:&mut Option<semio_os_services_native_io::OriginalSuppliedNativePath>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||!self.outcome_is_acknowledged(){return Err(invalid("selected native path requires its acknowledged original invocation"))}self.job.as_mut().unwrap().supply_result_last_path(reservation,output,cx)}
 /// 📮️ Caller-owned page output survives publication refusal and its reservation clears in a separate paid turn.
 pub fn supply_next_page(&mut self,output:&mut Option<OriginalSuppliedJobPage>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing||!self.acknowledged{return Err(invalid("native page publication requires an acknowledged live result"))}
  if output.is_some(){let reservation=self.reservation.as_mut().ok_or_else(||invalid("native supplied page requires its retained reservation"))?;if !self.job.as_mut().ok_or_else(||ValueError::invariant("job absent"))?.supply_result_last_page(reservation,output,cx)?{return Ok(false)}close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.reservation.take();RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;return Ok(self.reservation.is_none())}
  let pages=match self.result_status(){Some(NativeIoResultStatus::Bytes{pages,..}|NativeIoResultStatus::Page{pages,..})=>pages,_=>return Err(invalid("native invocation does not retain a page result"))};if pages==0{return Ok(false)}
  if !self.reservation.as_ref().is_some_and(OriginalJobPageReservation::ready){self.returns.reserve(&mut self.reservation,cx)?;return Ok(false)}self.job.as_mut().ok_or_else(||ValueError::invariant("job absent"))?.supply_result_last_page(self.reservation.as_mut().ok_or_else(||ValueError::invariant("reservation absent"))?,output,cx)?;Ok(false)
 }
 /// 🔁️ Streaming preserves the actual path, Origin and finite source across paid result retirement.
 pub fn restart_read_page(&mut self,offset:u64,max_bytes:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing||!self.acknowledged||self.reservation.is_some(){return Err(invalid("native page restart requires its acknowledged original invocation"))}
  if self.returns.retains_source(self.source_identity.ok_or_else(||ValueError::invariant("source_identity absent"))?){self.returns.close_returned_step(cx)?;return Ok(false)}let job=self.job.as_mut().ok_or_else(||ValueError::invariant("job absent"))?;if job.result_status().is_some(){job.close_result_step(cx)?;return Ok(false)}if !job.restart_read_page(offset,max_bytes,cx)?{return Ok(false)}self.terminal_seen=false;self.acknowledged=false;Ok(true)
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(job)=self.job.as_mut(){job.begin_close();}}
 /// ♻️ Returned capsules, descriptors, producer, World pages, actual Origin and caller header close in order.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}self.validate(cx)?;if !self.closing{return Err(invalid("native invocation close requires its explicit lifecycle transition"))}
  if self.returned_continuation.is_some()||self.returned_outcome.is_some(){return self.receive_return(cx).map(|progress|InteractiveJobCloseStep::Pending{progress})}
  if self.origin.continuation().is_some()||self.origin.outcome().is_some(){let step=self.origin.close_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}
  if self.reservation.is_some(){if !control_allowed(cx,true){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.reservation.take();return post_control(cx).map(|progress|InteractiveJobCloseStep::Pending{progress})}
  if let Some(job)=self.job.as_mut(){if !job.terminal_is_empty(){let step=job.close_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}if !control_allowed(cx,true){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.job.take();return post_control(cx).map(|progress|InteractiveJobCloseStep::Pending{progress})}
  if self.source_identity.is_some_and(|identity|self.returns.retains_source(identity)){let step=self.returns.close_returned_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}
  if !self.origin.terminal_is_empty(){let step=self.origin.close_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}
  if !control_allowed(cx,true){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.binding=None;self.source_identity=None;post_control(cx).map(|progress|InteractiveJobCloseStep::Complete{progress})
 }
 pub fn terminal_is_empty(&self)->bool{self.origin.terminal_is_empty()&&self.binding.is_none()&&self.source_identity.is_none()&&self.job.is_none()&&self.returned_outcome.is_none()&&self.returned_continuation.is_none()&&self.reservation.is_none()}
}
impl<const N:usize> Drop for RendererOriginalIoNode<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original renderer invocation requires genuine received outcomes and paid Source closure")}}
