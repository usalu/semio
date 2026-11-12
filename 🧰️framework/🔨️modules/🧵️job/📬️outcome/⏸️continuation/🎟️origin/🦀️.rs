//! 🪢️ Actual source custody and its preclaimed receiver have disjoint borrows throughout worker execution.
use super::{frame::OriginalReturnFrame,capsule::{InteractiveJobStep,OriginalJobReturn,OriginalJobContinuation,OriginalJobOutcomeReturn,OriginalJobOutcome},source::{OriginalReturnClaimCredit,OriginalReturnWitness,OriginalReturnPhase,OriginalReturnBodyIdentity,claim_original_sized_return,receive_original_return_witness}};
use crate::{OriginalCallerSourceParent,OriginalCallerSourceLoan,StepContext,OriginalTurnFailure,InteractiveJobCloseStep,JobOutcomeDescriptor};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
type OriginIdentity=(usize,Option<u64>,fn()->Option<u64>,u64);
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn identity(cx:&StepContext<'_>)->Result<OriginIdentity,ValueError>{Ok((cx.original_source_parent()?.source_identity()?,cx.original_clock_binding(),cx.clock_source(),cx.original_absolute_deadline_us()))}
struct OriginalReturnSourceOwner{parent:Option<OriginalCallerSourceParent>,pending:Option<OriginalCallerSourceLoan>,returning:Option<OriginalCallerSourceLoan>}
impl OriginalReturnSourceOwner{const fn empty()->Self{Self{parent:None,pending:None,returning:None}}fn terminal_is_empty(&self)->bool{self.parent.is_none()&&self.pending.is_none()&&self.returning.is_none()}}

/// 🎟️ See `README.md`.
pub struct OriginalJobReturnOrigin{source:OriginalReturnSourceOwner,receiver:OriginalJobReturnReceiver}
impl OriginalJobReturnOrigin{
 pub const fn empty()->Self{Self{source:OriginalReturnSourceOwner::empty(),receiver:OriginalJobReturnReceiver::empty()}}
 pub fn parent(&self)->Result<&OriginalCallerSourceParent,ValueError>{self.source.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return receiver has no admitted live source alias"))}
 pub fn receiver(&mut self)->Result<&mut OriginalJobReturnReceiver,ValueError>{if self.source.parent.is_none()||!self.receiver.source_ready{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original return receiver requires its paid source handoff"))}Ok(&mut self.receiver)}
 /// 🏡️ See `README.md`.
 pub fn source_parent(&self)->Result<&OriginalCallerSourceParent,ValueError>{if !self.receiver.source_ready{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original source requires its paid durable handoff"))}self.source.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original source handoff no longer owns its payer"))}
 pub fn context_parts(&mut self)->Result<(&OriginalCallerSourceParent,&mut OriginalJobReturnReceiver),ValueError>{if !self.receiver.source_ready{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"worker view requires the actual admitting source to be frozen first"))}let parent=self.source.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"worker callback requires its retained original source"))?;Ok((parent,&mut self.receiver))}
 pub fn original_clock(&self)->Result<(fn()->Option<u64>,u64),ValueError>{if !self.receiver.source_ready{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original clock requires the actual frozen source"))}let(_,_,now,deadline)=self.receiver.source.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"frozen source clock disappeared"))?;Ok((now,deadline))}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()&&self.receiver.terminal_is_empty()}
 pub fn body_ready(&self)->bool{self.receiver.source_ready&&!self.receiver.closing&&self.receiver.credit.as_ref().is_some_and(OriginalReturnClaimCredit::has_unused_capture)&&self.receiver.frame.as_ref().is_some_and(|frame|frame.owner.original().is_some_and(|payload|payload.callback.is_none()&&payload.qualification.is_none()&&payload.receipt_qualification.is_none()))}
 pub fn validate_close_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(())}
  if let Some(loan)=self.source.pending.as_ref().filter(|loan|!loan.terminal_is_empty()).or_else(||self.source.returning.as_ref().filter(|loan|!loan.terminal_is_empty())){loan.validate_retirement_context(cx).map_err(OriginalTurnFailure::from)}else{self.receiver.validate_source(cx)}
 }
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if !self.receiver.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.receiver.resources_empty(){return self.receiver.retirement_demands(copy)}
  if let Some(loan)=self.source.pending.as_ref().or(self.source.returning.as_ref()){if !loan.terminal_is_empty(){return Ok(loan.retirement_demands())}}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }

 pub fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.receiver.closing||cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  if self.source.parent.is_none(){
   if self.source.returning.is_some(){return Ok(false)}
   if self.source.pending.is_none(){if cx.original_return_budget_is_borrowed(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"standalone original return requires its genuine durable payer or a scoped parent relend").into())}cx.original_source_parent()?.admit_loan(&mut self.source.pending,cx)?;return Ok(false)}
   self.source.pending.as_ref().ok_or_else(||ValueError::invariant("pending absent"))?.validate_context(cx)?;if !cx.retained_work_can_enter(RetirementDemand{depth:1,..Default::default()}){return Ok(false)}let source=identity(cx)?;self.source.parent=Some(self.source.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?.into_borrowed_parent());self.receiver.source=Some(source);crate::receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(metadata())),work:1})?;return Ok(false)
  }
  self.receiver.prepare(cx)
 }
 pub fn enter_body<'claim>(&'claim mut self,cx:&StepContext<'_>)->Result<Option<OriginalJobReturnClaim<'claim>>,OriginalTurnFailure>{self.receiver.enter_body(cx)}
 pub fn continuation(&self)->Option<&OriginalJobContinuation>{self.receiver.continuation()}
 pub fn receive_continuation(&mut self,cx:&mut StepContext<'_>,input:&mut Option<OriginalJobContinuation>)->Result<bool,OriginalTurnFailure>{self.receiver.receive_continuation(cx,input)}
 pub fn receive_outcome(&mut self,cx:&mut StepContext<'_>,input:&mut Option<OriginalJobOutcome>)->Result<bool,OriginalTurnFailure>{self.receiver.receive_outcome(cx,input)}
 pub fn outcome(&self)->Option<&JobOutcomeDescriptor>{self.receiver.outcome.as_ref()}
 /// 🔔️ See `README.md`.
 pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.receiver.acknowledge_outcome(cx)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
  self.validate_close_context(cx)?;
  if !self.receiver.closing{let source=identity(cx)?;return crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.receiver.source=Some(source);self.receiver.closing=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}})}
  if !self.receiver.resources_empty(){return self.receiver.close_resources(cx)}
  if let Some(loan)=self.source.pending.as_mut(){if !loan.terminal_is_empty(){return loan.close_step(cx.original_source_parent()?,cx)}return crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.source.pending.take();self.receiver.source=None;self.receiver.source_ready=false;self.receiver.closing=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1}})}
  if self.source.parent.is_some(){return crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{let parent=self.source.parent.take().expect("parent absent");self.source.returning=Some(match parent.into_source_loan(){Ok(loan)=>loan,Err(parent)=>{self.source.parent=Some(parent);return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiver source entered unrelated retirement")),work:0}}});RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}})}
  let loan=self.source.returning.as_mut().ok_or_else(||ValueError::invariant("retained original source return"))?;if !loan.terminal_is_empty(){return loan.close_step(cx.original_source_parent()?,cx)}crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.source.returning.take();self.receiver.source=None;self.receiver.source_ready=false;self.receiver.closing=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1}})
 }
}
impl Drop for OriginalJobReturnOrigin{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original receiving owner requires same-source funded close");}}

/// 📬️ See `README.md`.
pub struct OriginalJobReturnReceiver{source_ready:bool,source:Option<OriginIdentity>,closing:bool,frame:Option<Box<OriginalReturnFrame>>,credit:Option<OriginalReturnClaimCredit>,witness:Option<OriginalReturnWitness>,continuation:Option<OriginalJobContinuation>,outcome:Option<JobOutcomeDescriptor>}
impl OriginalJobReturnReceiver{
 const fn empty()->Self{Self{source_ready:false,source:None,closing:false,frame:None,credit:None,witness:None,continuation:None,outcome:None}}
 fn resources_empty(&self)->bool{self.frame.is_none()&&self.credit.is_none()&&self.witness.is_none()&&self.continuation.is_none()&&self.outcome.is_none()}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&!self.source_ready&&!self.closing&&self.resources_empty()}
 fn validate_source(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let actual=parent.source_identity()?;if self.source.is_none_or(|(source,clock,now,deadline)|source!=actual||clock!=cx.original_clock_binding()||!std::ptr::fn_addr_eq(now,cx.clock_source())||deadline!=cx.original_absolute_deadline_us()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original return receiver requires its actual admitting source").into())}Ok(())}
 pub fn prepare(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(cx)?;if self.closing||self.continuation.is_some()||self.outcome.is_some()||self.credit.as_ref().is_some_and(|credit|!credit.has_unused_capture())||cx.is_cancelled()||cx.should_yield(){return Ok(false)}if cx.original_return_budget_is_borrowed(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"standalone original receiver requires its actual durable payer").into())}
  if !cx.admit_original_return_table()?{return Ok(false)}
  if self.frame.is_none(){let demand=RetirementDemand{capacity_bytes:size_of::<OriginalReturnFrame>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(demand){return Ok(false)}let layout=std::alloc::Layout::new::<OriginalReturnFrame>();let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<OriginalReturnFrame>());cx.consume_fuel(1);let Some(pointer)=pointer else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original return frame allocation refused").into())};self.frame=Some(unsafe{pointer.as_ptr().write(OriginalReturnFrame::empty());Box::from_raw(pointer.as_ptr())});self.source_ready=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})?;return Ok(false)}claim_original_sized_return(cx,&mut self.credit,self.frame.as_ref().ok_or_else(||ValueError::invariant("return frame absent"))?.as_ref() as *const _ as usize,0,0)
 }
 pub fn enter_body<'claim>(&'claim mut self,cx:&StepContext<'_>)->Result<Option<OriginalJobReturnClaim<'claim>>,OriginalTurnFailure>{self.validate_source(cx)?;if self.closing||cx.is_cancelled()||cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0||!self.credit.as_ref().is_some_and(OriginalReturnClaimCredit::has_unused_capture)||!self.frame.as_ref().is_some_and(|frame|frame.owner.original().is_some_and(|payload|payload.callback.is_none()&&payload.qualification.is_none()&&payload.receipt_qualification.is_none())){return Ok(None)}Ok(Some(OriginalJobReturnClaim{receiver:self,identity:OriginalReturnBodyIdentity::capture(cx)}))}
 pub fn continuation(&self)->Option<&OriginalJobContinuation>{self.continuation.as_ref()}
 pub fn outcome(&self)->Option<&JobOutcomeDescriptor>{self.outcome.as_ref()}
 pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(cx)?;if self.closing||self.continuation.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original outcome acknowledgement requires its live receiving place").into())}
  let Some(outcome)=self.outcome.as_ref()else{return Ok(true)};let acknowledged=outcome.is_acknowledged();let depth=if acknowledged{1}else{2};
  crate::close_original_retained_step(cx,RetirementDemand{depth,..Default::default()},|grant|{
   if !acknowledged{let Some(outcome)=self.outcome.as_mut()else{return RetainedWorkOutcome{result:Err(ValueError::invariant("outcome absent")),work:0}};let turn=outcome.acknowledge(semio_framework_value_retained_clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
   self.outcome.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1}
  })?;Ok(self.outcome.is_none())
 }

 pub fn receive_continuation(&mut self,cx:&mut StepContext<'_>,input:&mut Option<OriginalJobContinuation>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(cx)?;let Some(capsule)=input.as_mut()else{return Ok(true)};capsule.validate(cx)?;if self.continuation.is_some()||self.outcome.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiver already owns its returned result").into())}let expected=1+usize::from(capsule.witness.is_some());cx.preflight_original_reserved_return(RetainedCloneProgress{copied_items:expected,..Default::default()},expected as u64)?;let credit=self.credit.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original continuation requires its pre-body receiving claim"))?;receive_original_return_witness(cx,credit,&mut capsule.witness)?;if self.witness.is_none(){self.witness=Some(credit.issue_return(OriginalReturnPhase::Receive,0)?);}receive_original_return_witness(cx,credit,&mut self.witness)?;assert!(credit.terminal_is_empty(),"both original return witnesses received once");self.credit.take();self.continuation=input.take();Ok(true)
 }
 pub fn receive_outcome(&mut self,cx:&mut StepContext<'_>,input:&mut Option<OriginalJobOutcome>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(cx)?;let Some(returned)=input.as_mut()else{return Ok(true)};if self.continuation.is_some()||self.outcome.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiver already owns its returned result").into())}let credit=self.credit.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original outcome requires its pre-body receiving claim"))?;receive_original_return_witness(cx,credit,&mut returned.witness)?;assert!(credit.terminal_is_empty(),"normal original return received once");self.credit.take();self.outcome=returned.outcome.take();input.take();Ok(true)
 }
 fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(capsule)=self.continuation.as_ref(){return if capsule.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{capsule.retirement_demands(copy)}}
  if let Some(outcome)=self.outcome.as_ref(){return Ok(RetirementDemand{depth:if outcome.is_acknowledged(){1}else{2},..Default::default()})}
  if self.credit.as_ref().is_some_and(|credit|!credit.has_unused_capture())&&!self.witness.as_ref().is_some_and(OriginalReturnWitness::releases_unused_capture){return Ok(Default::default())}
  if self.credit.is_some()||self.witness.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(frame)=self.frame.as_ref(){return if frame.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<OriginalReturnFrame>(),depth:1,..Default::default()})}else{frame.demand(copy)}}
  Ok(Default::default())
 }
 fn close_resources(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.validate_source(cx)?;if let Some(capsule)=self.continuation.as_mut(){if !capsule.terminal_is_empty(){return capsule.close_step(cx).map(|step|match step{InteractiveJobCloseStep::Complete{progress}|InteractiveJobCloseStep::Pending{progress}=>InteractiveJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked})}return crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.continuation.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}})}
  if let Some(outcome)=self.outcome.as_ref(){let acknowledged=outcome.is_acknowledged();let depth=if acknowledged{1}else{2};return crate::close_original_retained_step(cx,RetirementDemand{depth,..Default::default()},|grant|{if !acknowledged{let Some(outcome)=self.outcome.as_mut()else{return RetainedWorkOutcome{result:Err(ValueError::invariant("outcome absent")),work:0}};let turn=outcome.acknowledge(semio_framework_value_retained_clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}self.outcome.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}})}
  if self.credit.as_ref().is_some_and(|credit|!credit.has_unused_capture())&&!self.witness.as_ref().is_some_and(OriginalReturnWitness::releases_unused_capture){return Ok(InteractiveJobCloseStep::Blocked)}if self.credit.is_some()||self.witness.is_some(){if self.witness.is_none(){cx.preflight_original_reserved_return(metadata(),1)?;let credit=self.credit.as_mut().ok_or_else(||ValueError::invariant("credit absent"))?;self.witness=Some(if credit.has_unused_capture(){credit.issue_success_return(0)?}else{credit.issue_return(OriginalReturnPhase::Receive,0)?});}let before=cx.retained_progress();receive_original_return_witness(cx,self.credit.as_mut().ok_or_else(||ValueError::invariant("credit absent"))?,&mut self.witness)?;let after=cx.retained_progress();if self.credit.as_ref().is_some_and(OriginalReturnClaimCredit::terminal_is_empty){self.credit.take();}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}
  if let Some(frame)=self.frame.as_mut(){if !frame.terminal_is_empty(){let body=cx.retained_grant().maximum_copy_bytes;let demand=frame.demand(body)?;return crate::close_original_effect_step(cx,demand,|effect|frame.close_step(body,effect))}return crate::close_original_retained_step(cx,RetirementDemand{release_bytes:size_of::<OriginalReturnFrame>(),depth:1,..Default::default()},|_|{self.frame.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:size_of::<OriginalReturnFrame>(),..metadata()})),work:1}})}Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
}

/// 🎟️ See `README.md`.
pub struct OriginalJobReturnClaim<'receiver>{receiver:&'receiver mut OriginalJobReturnReceiver,identity:OriginalReturnBodyIdentity}
impl OriginalJobReturnClaim<'_>{
 pub fn qualify(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{super::source::qualify_original_return_credit(self.receiver.credit.as_ref().ok_or_else(||ValueError::invariant("live claimed original body"))?,cx,&self.identity)}
 pub fn finish<'job>(self,cx:&StepContext<'_>,result:InteractiveJobStep<'job>,receipt_qualification:Option<OriginalTurnFailure>)->OriginalJobReturn<'job>{let qualification=self.qualify(cx).err();let(callback,descriptor)=match result{InteractiveJobStep::Outcome(None)if qualification.is_none()&&receipt_qualification.is_none()=>return OriginalJobReturn::Pending,InteractiveJobStep::Outcome(outcome)if qualification.is_none()&&receipt_qualification.is_none()=>{let witness=self.receiver.credit.as_mut().expect("unique normal original return").issue_success_return(0).expect("unused genuine normal receiving claim");return OriginalJobReturn::Outcome(OriginalJobOutcomeReturn::captured(outcome,witness))},InteractiveJobStep::Outcome(outcome)=>(None,outcome.map(crate::JobOutcomeBorrow::into_descriptor)),InteractiveJobStep::Continuation(failure)=>(Some(failure),None)};let witness=self.receiver.credit.as_mut().expect("unique original return claim").issue_return(OriginalReturnPhase::Capture,0).expect("unused genuine one-shot capture");let frame=self.receiver.frame.take().expect("same prepaid original return frame");OriginalJobReturn::Continuation(OriginalJobContinuation::captured(frame,callback,qualification,receipt_qualification,descriptor,witness,self.receiver.source.expect("actual original return source")))}
}
