//! 🎟️ One inline native I/O invocation retains its authentic source through semantic acknowledgement and page returns.
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptorSlot,JobOutcomeKind,JobOutcomeView,OriginalCallerSourceLoan,OriginalJobPageReservation,OriginalJobPageReturnRegistry,OriginalSuppliedJobPage,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_os_services_native_io::{NativeIoJob, NativeIoRequest, NativeIoResultStatus};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::size_of;

/// 🧳️ The enclosing fixed Node owns this carrier before first stepping its address-sensitive result descriptors.
pub struct RendererOriginalIoNode<const N:usize>{
 source:Option<OriginalCallerSourceLoan>,
 source_identity:usize,
 job:Option<NativeIoJob>,
 descriptor:JobOutcomeDescriptorSlot,
 terminal_seen:bool,
 acknowledged:bool,
 reservation:Option<OriginalJobPageReservation<N>>,
 returns:&'static OriginalJobPageReturnRegistry<N>,
 closing:bool,
}

fn invalid(detail:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,detail).into()}
fn metadata(cx:&mut StepContext<'_>,bytes:usize,closing:bool)->Result<bool,OriginalTurnFailure>{
 let grant=cx.retained_grant();
 if (if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
 cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}

impl<const N:usize> RendererOriginalIoNode<N>{
 /// 📥️ Genuine loan admission and request transfer are separate resumable turns, including one-item callbacks.
 pub fn admit(request:&mut Option<NativeIoRequest>,output:&mut Option<Self>,returns:&'static OriginalJobPageReturnRegistry<N>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if request.is_none(){return Err(invalid("native invocation requires its original request"))}
  if let Some(node)=output.as_mut(){
   node.validate(cx)?;
   if node.closing||node.job.is_some()||!std::ptr::eq(node.returns,returns){return Err(invalid("native invocation candidate already adopted its request or changed return owner"))}
   if !metadata(cx,size_of::<NativeIoRequest>()+size_of::<bool>(),false)?{return Ok(false)}
   node.job=Some(NativeIoJob::new(request.take().unwrap()));return Ok(true)
  }
  let source_identity=cx.original_source_parent()?.source_identity()?;
  if cx.retained_grant().maximum_copy_bytes<size_of::<NativeIoRequest>()+size_of::<bool>(){return Ok(false)}
  let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
  *output=Some(Self{source:Some(source),source_identity,job:None,descriptor:JobOutcomeDescriptorSlot::empty(),terminal_seen:false,acknowledged:false,reservation:None,returns,closing:false});Ok(false)
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.source.as_ref().ok_or_else(||invalid("native invocation source has returned"))?.validate_context(cx)}
 /// ▶️ Every continuation uses the same actual roots, payload ledger, opaque clock and absolute deadline.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.closing{return Err(invalid("closing native invocation cannot resume work"))}
  if !self.descriptor.is_empty(){
   let job=self.job.as_ref().ok_or_else(||invalid("native invocation lost its original job"))?;
   if let Some(descriptor)=self.descriptor.original().filter(|descriptor|!descriptor.is_acknowledged()){job.borrow_outcome(descriptor)?;}
   self.acknowledge_step(cx)?;
   if self.descriptor.is_empty()&&self.terminal_seen{self.acknowledged=true;}
   return Ok(self.acknowledged)
  }
  if self.acknowledged{return Ok(true)}
  let outcome=self.job.as_mut().ok_or_else(||invalid("native invocation lost its original job"))?.step(cx)?.map(JobOutcomeBorrow::into_descriptor);
  if let Some(descriptor)=outcome{self.terminal_seen=matches!(descriptor.kind(),JobOutcomeKind::Complete|JobOutcomeKind::Cancelled|JobOutcomeKind::Fault);assert!(self.descriptor.retain(descriptor).is_ok(),"exclusive native invocation outcome slot");}
  Ok(false)
 }
 fn acknowledge_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let demand=self.descriptor.retirement_demands();let descriptor=&mut self.descriptor;
  close_original_retained_step(cx,demand,|grant|Ok(descriptor.close_step(grant)))
 }
 /// 👁️ Scalars never extract the original payload header, path or complete failure from its retained producer.
 pub fn result_status(&self)->Option<NativeIoResultStatus>{self.acknowledged.then(||self.job.as_ref().and_then(NativeIoJob::result_status)).flatten()}
 /// 🔎️ Complete fault, checkpoint and result payloads remain borrowed from their genuine descriptor before acknowledgement.
 pub fn borrow_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{self.descriptor.original().filter(|descriptor|!descriptor.is_acknowledged()).map(|descriptor|self.job.as_ref().expect("original descriptor retains its producer").borrow_outcome(descriptor)).transpose()}
 pub fn request_backing_identity(&self)->Option<*const u8>{self.job.as_ref().and_then(NativeIoJob::retained_request_backing_identity)}
 pub fn producer_is_empty(&self)->bool{self.job.is_none()}
 pub fn outcome_is_acknowledged(&self)->bool{self.acknowledged&&self.descriptor.is_empty()}
 /// 📐️ The acknowledged producer retains its actual opened-file extent and borrowed page backing.
 pub fn known_file_length(&self)->Option<u64>{self.job.as_ref().and_then(NativeIoJob::known_file_length)}
 pub fn result_page(&self,index:usize)->Option<&[u8]>{self.acknowledged.then(||self.job.as_ref().and_then(|job|job.result_page(index))).flatten()}
 pub fn result_modified(&self,index:usize)->Option<(&std::path::Path,std::time::SystemTime)>{self.acknowledged.then(||self.job.as_ref().and_then(|job|job.result_modified(index))).flatten()}
 /// 🧭️ The original invocation validates source and acknowledgement before moving its actual selected path.
 pub fn supply_selected_path<const P:usize>(&mut self,reservation:&mut semio_framework_os_services_native_io::OriginalNativePathReservation<P>,output:&mut Option<semio_framework_os_services_native_io::OriginalSuppliedNativePath>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||!self.outcome_is_acknowledged(){return Err(invalid("selected native path requires its acknowledged original invocation"))}self.job.as_mut().unwrap().supply_result_last_path(reservation,output,cx)}
 /// 📮️ A small original page carrier is retained in the caller's output slot before its receipt can refuse publication.
 pub fn supply_next_page(&mut self,output:&mut Option<OriginalSuppliedJobPage>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.closing||!self.acknowledged||output.is_some(){return Err(invalid("native page publication requires an acknowledged result and empty output slot"))}
  let pages=match self.result_status(){Some(NativeIoResultStatus::Bytes{pages,..}|NativeIoResultStatus::Page{pages,..})=>pages,_=>return Err(invalid("native invocation does not retain a page result"))};
  if pages==0{return Ok(false)}
  if self.reservation.is_none(){self.reservation=self.returns.reserve(cx)?;return Ok(false)}
  let page=self.job.as_mut().unwrap().supply_result_last_page(self.reservation.as_mut().unwrap(),cx)?;
  if let Some(page)=page{*output=Some(page);self.reservation.take();return Ok(true)}Ok(false)
 }
 /// 🔁️ Streaming reuses the same original job and path after the preceding result's paid retirement.
 pub fn restart_read_page(&mut self,offset:u64,max_bytes:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.closing||!self.acknowledged||self.reservation.is_some(){return Err(invalid("native page restart requires its acknowledged original invocation"))}
  if self.returns.retains_source(self.source_identity){self.returns.close_returned_step(cx)?;return Ok(false)}
  let job=self.job.as_mut().unwrap();
  if job.result_status().is_some(){job.close_result_step(cx)?;return Ok(false)}
  if !job.restart_read_page(offset,max_bytes,cx)?{return Ok(false)}
  self.terminal_seen=false;self.acknowledged=false;Ok(true)
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(job)=self.job.as_mut(){job.begin_close();}}
 /// ♻️ Original output, producer, returned pages and invocation loan all close before the creating source can disappear.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
  self.source.as_ref().unwrap().validate_retirement_context(cx)?;
  if !self.closing{return Err(invalid("native invocation close requires its explicit lifecycle transition"))}
  if !self.descriptor.is_empty(){let step=self.acknowledge_step(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
  if self.reservation.is_some(){let bytes=size_of::<Option<OriginalJobPageReservation<N>>>();let admitted=metadata(cx,bytes,true)?;if admitted{self.reservation.take();}return Ok(InteractiveJobCloseStep::Pending{progress:if admitted{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}else{Default::default()}})}
  if let Some(job)=self.job.as_mut(){
   if !job.terminal_is_empty(){let step=job.close_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}
   let bytes=size_of::<bool>();let admitted=metadata(cx,bytes,true)?;if admitted{self.job.take();}return Ok(InteractiveJobCloseStep::Pending{progress:if admitted{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}else{Default::default()}})
  }
  if self.returns.retains_source(self.source_identity){let step=self.returns.close_returned_step(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
  let source=self.source.as_mut().unwrap();let step=source.close_step(cx.original_source_parent()?,cx)?;
  if source.terminal_is_empty(){self.source.take();}
  Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.job.is_none()&&self.descriptor.is_empty()&&self.reservation.is_none()}
}

impl<const N:usize> Drop for RendererOriginalIoNode<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original renderer invocation requires paid page returns and original-source closure before Drop");}}

