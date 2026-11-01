//! ⏸️ The actual return arm owns its prepaid frame and uninserted native results until a genuine receiver admits them.
use super::{frame::OriginalReturnFrame,source::OriginalReturnWitness};
use crate::{JobOutcomeBorrow,JobOutcomeDescriptor,OriginalTurnFailure,StepContext,InteractiveJobCloseStep};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};

#[must_use]
pub enum InteractiveJobStep<'job>{Outcome(Option<JobOutcomeBorrow<'job>>),Continuation(OriginalTurnFailure)}
impl<'job> InteractiveJobStep<'job>{pub fn from_callback(result:Result<Option<JobOutcomeBorrow<'job>>,OriginalTurnFailure>)->Self{match result{Ok(outcome)=>Self::Outcome(outcome),Err(failure)=>Self::Continuation(failure)}}}
#[must_use]
pub enum OriginalJobReturn<'job>{Preparing,Pending,Outcome(OriginalJobOutcomeReturn<'job>),Continuation(OriginalJobContinuation)}
#[must_use]
pub struct OriginalJobOutcomeReturn<'job>{outcome:Option<JobOutcomeBorrow<'job>>,witness:Option<OriginalReturnWitness>}
#[must_use]
pub struct OriginalJobOutcome{pub(super) outcome:Option<JobOutcomeDescriptor>,pub(super) witness:Option<OriginalReturnWitness>}
impl<'job> OriginalJobOutcomeReturn<'job>{
 pub(super) fn captured(outcome:Option<JobOutcomeBorrow<'job>>,witness:OriginalReturnWitness)->Self{Self{outcome,witness:Some(witness)}}
 pub fn original(&self)->Option<&JobOutcomeBorrow<'job>>{self.outcome.as_ref()}
 pub fn into_descriptor(self)->OriginalJobOutcome{OriginalJobOutcome{outcome:self.outcome.map(JobOutcomeBorrow::into_descriptor),witness:self.witness}}
}
/// 👁️ Immutable original causes and descriptor stay in their actual continuation owner.
pub struct OriginalJobContinuationView<'a>{pub callback:Option<&'a OriginalTurnFailure>,pub qualification:Option<&'a OriginalTurnFailure>,pub receipt_qualification:Option<&'a OriginalTurnFailure>,pub descriptor:Option<&'a JobOutcomeDescriptor>}
#[must_use]
pub struct OriginalJobContinuation{pub(super) frame:Option<Box<OriginalReturnFrame>>,pub(super) callback:Option<OriginalTurnFailure>,pub(super) qualification:Option<OriginalTurnFailure>,pub(super) receipt_qualification:Option<OriginalTurnFailure>,pub(super) descriptor:Option<JobOutcomeDescriptor>,pub(super) witness:Option<OriginalReturnWitness>,pub(super) source:Option<(usize,Option<u64>,fn()->Option<u64>,u64)>,inserted:bool}
impl OriginalJobContinuation{
 /// 🪟️ Borrows complete original causes both before and after their paid frame insertion.
 pub fn view(&self)->OriginalJobContinuationView<'_>{OriginalJobContinuationView{callback:self.callback(),qualification:self.qualification(),receipt_qualification:self.receipt_qualification.as_ref().or_else(||self.frame.as_ref()?.owner.original()?.receipt_qualification.as_ref()),descriptor:self.descriptor.as_ref().or_else(||self.frame.as_ref()?.descriptor.as_ref())}}
 pub(super) fn captured(frame:Box<OriginalReturnFrame>,callback:Option<OriginalTurnFailure>,qualification:Option<OriginalTurnFailure>,receipt_qualification:Option<OriginalTurnFailure>,descriptor:Option<JobOutcomeDescriptor>,witness:OriginalReturnWitness,source:(usize,Option<u64>,fn()->Option<u64>,u64))->Self{Self{frame:Some(frame),callback,qualification,receipt_qualification,descriptor,witness:Some(witness),source:Some(source),inserted:false}}
 pub(crate) fn callback(&self)->Option<&OriginalTurnFailure>{self.callback.as_ref().or_else(||self.frame.as_ref()?.owner.original()?.callback.as_ref())}
 pub(crate) fn qualification(&self)->Option<&OriginalTurnFailure>{self.qualification.as_ref().or_else(||self.frame.as_ref()?.owner.original()?.qualification.as_ref())}
 pub(crate) fn terminal_is_empty(&self)->bool{self.frame.is_none()&&self.callback.is_none()&&self.qualification.is_none()&&self.receipt_qualification.is_none()&&self.descriptor.is_none()&&self.witness.is_none()&&self.source.is_none()}
 pub(super) fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let source=parent.source_identity()?;if self.source.is_none_or(|(original,clock,now,deadline)|original!=source||clock!=cx.original_clock_binding()||!std::ptr::fn_addr_eq(now,cx.clock_source())||deadline!=cx.original_absolute_deadline_us()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned original continuation requires its actual admitting source").into())}Ok(())}
 pub(super) fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if self.witness.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned continuation must redeem its held return before insertion or close"))}
  if !self.inserted{return Ok(RetirementDemand{depth:1,..Default::default()})}
  let frame=self.frame.as_ref().ok_or_else(||ValueError::invariant("prepaid original continuation frame"))?;if !frame.terminal_is_empty(){return frame.demand(copy)}
  Ok(RetirementDemand{release_bytes:size_of::<OriginalReturnFrame>(),depth:1,..Default::default()})
 }
 pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}self.validate(cx)?;if self.witness.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned continuation must redeem its held return before insertion or close").into())}
  if !self.inserted{return crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{let frame=self.frame.as_mut().expect("prepaid original continuation frame");let payload=frame.owner.original_mut().expect("uninserted original continuation");assert!(payload.callback.is_none()&&payload.qualification.is_none()&&payload.receipt_qualification.is_none()&&frame.descriptor.is_none(),"one genuine continuation insertion");payload.callback=self.callback.take();payload.qualification=self.qualification.take();payload.receipt_qualification=self.receipt_qualification.take();frame.descriptor=self.descriptor.take();self.inserted=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})}
  let frame=self.frame.as_mut().ok_or_else(||ValueError::invariant("frame absent"))?;if !frame.terminal_is_empty(){let demand=frame.demand(cx.retained_grant().maximum_copy_bytes)?;return crate::close_original_retained_step(cx,demand,|grant|frame.close_step(grant))}
  crate::close_original_retained_step(cx,RetirementDemand{release_bytes:size_of::<OriginalReturnFrame>(),depth:1,..Default::default()},|_|{self.frame.take();self.source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:size_of::<OriginalReturnFrame>(),..Default::default()})),work:1}})
 }
}
impl Drop for OriginalJobContinuation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned original continuation retains its exact results or paid frame");}}
