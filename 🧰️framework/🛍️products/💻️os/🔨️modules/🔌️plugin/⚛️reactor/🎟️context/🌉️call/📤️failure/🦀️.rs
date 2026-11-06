//! 📤️ Entry borrows its whole original failure while bounded Pack windows become paid fault pages.
use semio_framework_job::OriginalDomainProjection;
use semio_framework_job::{receive_original_retained_work, OriginalTurnFailure, StepContext, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView};
use semio_framework_job_payload::{RetainedPayloadBuilder, JobPayloadStream};
use semio_framework_job_fault_publication::OriginalFaultPayloadStep;
use semio_os_kernel::record::BorrowedProjectedPackCursor;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

/// 🏛️ The enclosing Entry admission owns this fixed window and metadata before the same cause is lent.
pub(super) struct RetainedOriginalFailurePublication{builder:RetainedPayloadBuilder,cursor:BorrowedProjectedPackCursor,buffer:[u8;64],binding:Option<(usize,semio_framework_job::OriginalSourceBinding)>,length:usize,offset:usize,delivered:bool,closing:bool}
impl RetainedOriginalFailurePublication{
 pub(super) fn new()->Self{Self{builder:RetainedPayloadBuilder::new(JobPayloadStream::Fault),cursor:Default::default(),buffer:[0;64],binding:None,length:0,offset:0,delivered:false,closing:false}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Entry failure publication requires its same immutable cause and caller")}
 fn identity(original:&impl OriginalDomainProjection,cx:&StepContext<'_>)->(usize,semio_framework_job::OriginalSourceBinding){(original.projection_identity(),semio_framework_job::OriginalSourceBinding::borrow(cx))}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub(super) fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.builder.terminal_is_empty()&&!self.closing}
 pub(super) fn advance_demands(&self,original:&impl OriginalDomainProjection,cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.binding.is_some_and(|bound|bound!=Self::identity(original,cx)){return Err(Self::invalid())}
  if self.delivered{return Ok(Default::default())}if self.binding.is_none(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of_val(&self.binding),depth:1,..Default::default()})}
  if !self.builder.is_initialized(){return Self::nested(self.builder.initialization_demand())}
  if self.offset<self.length{return Self::nested(self.builder.append_demand(cx.retained_grant())?)}
  if self.length>0{return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<usize>(),depth:1,..Default::default()})}
  if !self.cursor.is_complete(){return Self::nested(RetirementDemand{copy_bytes:self.cursor.next_minimum_copy_bytes().checked_add(std::mem::size_of::<usize>()).ok_or_else(Self::invalid)?,capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:self.cursor.next_advance_depth_demand()?,..Default::default()})}
  if self.builder.published().is_none(){return Self::nested(RetainedPayloadBuilder::seal_demand())}
  Ok(RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:2,..Default::default()})
 }
 /// 🪙️ Every encoding, copy, page, seal and descriptor action consumes the same original context once.
 pub(super) fn advance<'a>(&'a mut self,original:&OriginalTurnFailure,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{self.advance_with_cursor(original,cx,0)}
 /// 📤️ Funds the caller's descriptor slot on the exact fault-admission turn.
 pub(super) fn advance_with_cursor<'a>(&'a mut self,original:&OriginalTurnFailure,cx:&mut StepContext<'_>,cursor:usize)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  self.advance_projection_with_cursor(original,PublicationCause::Turn(original),cx,cursor)
 }
 pub(super) fn advance_input_with_cursor<'a>(&'a mut self,view:&crate::original_input::OriginalActorInputFailureView<'_>,cx:&mut StepContext<'_>,cursor:usize)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  let authority=if let Some(v)=view.outcome.and_then(|v|v.receiving_refusal.as_ref()).or_else(||view.settlement.and_then(|v|v.receiving_refusal.as_ref())){PublicationCause::Turn(v)}else if let Some(v)=view.outcome.and_then(|v|v.value.as_ref().err()){PublicationCause::Receiving(v)}else if let Some(v)=view.settlement.and_then(|v|v.value.as_ref().err()){PublicationCause::Value(v)}else{return Err(Self::invalid().into())};self.advance_projection_with_cursor(view,authority,cx,cursor)
 }
 fn advance_projection_with_cursor<'a>(&'a mut self,original:&impl OriginalDomainProjection,authority:PublicationCause<'_>,cx:&mut StepContext<'_>,cursor:usize)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  let mut demand=self.advance_demands(original,cx)?;if self.cursor.is_complete()&&self.length==0&&self.builder.published().is_some(){demand.copy_bytes=demand.copy_bytes.checked_add(cursor).ok_or_else(Self::invalid)?;}if self.delivered||(if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()})||!Self::permits(cx.retained_grant(),demand){return Ok(None)}let grant=cx.retained_grant();let own_items=if self.length>0&&self.offset>=self.length{2}else if self.builder.is_initialized()&&self.offset>=self.length&&(self.builder.published().is_some()||!self.cursor.is_complete()){2}else{1};if grant.maximum_items<own_items{return Ok(None)}cx.consume_fuel(1);
  if self.binding.is_none(){cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of_val(&self.binding),..Default::default()})?;self.binding=Some(Self::identity(original,cx));return Ok(None)}
  if !self.builder.is_initialized(){let turn=if cx.original_retirement_active(){self.builder.advance_initialization(&authority.borrow(cx)?)}else{self.builder.advance_initialization(cx)};receive_original_retained_work(cx,turn)?;return Ok(None)}
  if self.offset<self.length{let turn=if cx.original_retirement_active(){self.builder.append_original(&mut authority.borrow(cx)?,&self.buffer[..self.length],&mut self.offset)}else{self.builder.append_original(cx,&self.buffer[..self.length],&mut self.offset)};receive_original_retained_work(cx,turn)?;return Ok(None)}
  if self.length>0{cx.consume_retained(RetainedCloneProgress{copied_items:2,copied_bytes:2*std::mem::size_of::<usize>(),..Default::default()})?;self.length=0;self.offset=0;return Ok(None)}
  if !self.cursor.is_complete(){let result=self.cursor.advance_intrinsic(original,1,&mut self.buffer,RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<usize>(),maximum_depth:grant.maximum_depth-1,..grant});match result{Ok(step)=>{self.length=step.written_bytes;cx.consume_retained(step.progress)?;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<usize>(),..Default::default()})?},Err(error)=>{self.length=error.written_bytes;let _=cx.consume_retained(error.reason.retained_progress());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<usize>(),..Default::default()})?;return Err(error.reason.into())}}return Ok(None)}
  if self.builder.published().is_none(){let turn=if cx.original_retirement_active(){self.builder.seal(&authority.borrow(cx)?)}else{self.builder.seal(cx)};receive_original_retained_work(cx,turn)?;return Ok(None)}let outcome=if cx.original_retirement_active(){JobOutcomeBorrow::admit_fault_with_cursor_retiring(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?}else{JobOutcomeBorrow::admit_fault_with_cursor(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?};if outcome.is_some(){cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<bool>(),..Default::default()})?;self.delivered=true}Ok(outcome)
 }
 pub(super) fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{if !self.delivered||self.closing{return Err(Self::invalid())}descriptor.fault(self.builder.published().ok_or_else(Self::invalid)?)}
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()})}if !self.cursor.terminal_is_empty(){return Self::nested(self.cursor.retirement_demands()?)}if !self.builder.terminal_is_empty(){return Self::nested(self.builder.retirement_demands()?)}Ok(RetirementDemand{copy_bytes:std::mem::size_of_val(&self.binding)+2*std::mem::size_of::<usize>()+2*std::mem::size_of::<bool>(),depth:1,..Default::default()})}
 /// ♻️ Symbols and original publication pages finish before Entry may move the whole cause into retirement.
 pub(super) fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|(_,binding)|!binding.matches(cx)){return Err(Self::invalid().into())}if self.terminal_is_empty(){return Ok(true)}
  cx.with_original_retirement(|cx|{
   if self.closing&&!self.cursor.terminal_is_empty(){let demand=Self::nested(self.cursor.retirement_demands()?)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,1,|grant|semio_framework_value::RetainedWorkOutcome{result:self.cursor.close(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress)).map_err(|error|error.reason),work:1})?;return Ok(false)}
   if self.closing&&!self.builder.terminal_is_empty(){let demand=Self::nested(self.builder.retirement_demands()?)?;let work=self.builder.retirement_work()?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|self.builder.close_step_granted(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}))?;return Ok(false)}
   let final_step=self.closing&&self.length==0&&self.offset==0&&!self.delivered;let items=if final_step{2}else{1};let bytes=if final_step{std::mem::size_of_val(&self.binding)+std::mem::size_of::<bool>()}else if !self.closing||self.delivered&&self.length==0&&self.offset==0{std::mem::size_of::<bool>()}else{std::mem::size_of::<usize>()};let progress=RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()<items as u64||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return Ok(false)}cx.retained_progress().checked_add(progress)?;
   if !self.closing{self.closing=true}else if self.length!=0{self.length=0}else if self.offset!=0{self.offset=0}else if self.delivered{self.delivered=false}else{self.binding=None;self.closing=false}let posted=cx.consume_retained(progress);cx.consume_fuel(items as u64);posted?;Ok(self.terminal_is_empty())
  })
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if !Self::permits(grant,demand)||self.closing&&self.cursor.terminal_is_empty()&&self.builder.terminal_is_empty()&&grant.maximum_items<5{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<bool>(),..Default::default()}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if !self.cursor.terminal_is_empty(){return self.cursor.close(child).map(|step|RetainedCloneStep::Progress(step.progress)).map_err(|error|error.reason)}if !self.builder.terminal_is_empty(){return self.builder.close_step_granted(child).result.map(|step|RetainedCloneStep::Progress(step.progress()))}
  self.binding=None;self.length=0;self.offset=0;self.delivered=false;self.closing=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:5,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}

#[derive(Clone,Copy)]enum PublicationCause<'a>{Turn(&'a OriginalTurnFailure),Receiving(&'a semio_framework_job::OriginalReceivingPrimary),Value(&'a ValueError)}
impl PublicationCause<'_>{fn borrow<'a,'b>(&'a self,cx:&'a mut StepContext<'b>)->Result<OriginalFaultPayloadStep<'a,'b>,ValueError>{match self{Self::Turn(v)=>OriginalFaultPayloadStep::borrow_failure(cx,v),Self::Receiving(v)=>OriginalFaultPayloadStep::borrow_receiving(cx,v),Self::Value(v)=>OriginalFaultPayloadStep::borrow_value(cx,v)}}}
impl Drop for RetainedOriginalFailurePublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Entry failure publication requires funded terminal closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
