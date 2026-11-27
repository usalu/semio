//! 🧳️ Genuine parent roots stay in one strong source while static children borrow actual incoming turns.
use super::*;
use semio_framework_value_retained_clone::{RetirementDemand,RetainedCloneStep,RetainedWorkOutcome,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use crate::original_job_continuation::{OriginalReturnSource,OriginalReturnBudgetReference};

#[path="🏠️native/🦀️.rs"]
mod native;
pub use native::{ORIGINAL_NATIVE_SOURCE_CAPACITY,OriginalNativeLease,OriginalNativeRetirement,OriginalNativeBorrow,OriginalNativeBorrowed,OriginalNativeRecipientIdentity,OriginalNativeRecipientClosed,OriginalNativeRetiredWitness};
const SOURCE_CONTROL_WORK:u64=1;
fn pay_source_close_metadata(effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<bool,(ValueError,u128)>{let remaining=effect.remaining_work_units().map_err(|error|(error,0))?;if remaining<SOURCE_CONTROL_WORK{return Ok(false)}match effect.performed(SOURCE_CONTROL_WORK){Ok(true)=>Ok(true),Ok(false)=>Err((ValueError::literal(ValueRefusalKind::WorkLimit,"Source close exceeded its original Work grant"),u128::from(SOURCE_CONTROL_WORK))),Err(error)=>Err((error,0))}}
type SourceClockOwner=semio_framework_value_retained_clone::retirement::controlled::RetainedOwnerGate<semio_framework_job_clock::OriginalSourceClock>;
type SourceClockLoan<'a>=semio_framework_value_retained_clone::retirement::controlled::RetainedOwnerGateGuard<'a,semio_framework_job_clock::OriginalSourceClock>;
fn borrow_source_clock(clock:&SourceClockOwner)->Result<SourceClockLoan<'_>,ValueError>{clock.try_lock().map_err(|_|refused("original Source clock is currently lent"))}
pub use semio_framework_job_failure::OriginalSourceCloseFailure;
static ORIGINAL_RETURN_BIRTH:std::sync::atomic::AtomicU32=std::sync::atomic::AtomicU32::new(1);
#[doc(hidden)]
pub struct SourceRoots{turns:OriginalSourceTurnIssuer,authority:JobPayloadAuthority,cancel:CancelToken,returns:OriginalReturnSource,native:native::NativeSources,clock:SourceClockOwner}
impl SourceRoots{fn issuer()->SharedIssuer<Self>{let mut issuer=SharedIssuer::owned();issuer.transfer_copy_bytes=|_|0;issuer}}
pub struct SourceRetirement{authority:JobPayloadAuthority,cancel:CancelTokenRetirement,returns:OriginalReturnSource,clock:SourceClockOwner}
impl SourceRetirement{
 fn clock_demand(&self)->Result<Option<RetirementDemand>,ValueError>{borrow_source_clock(&self.clock)?.retirement_demand()}
 fn clock_work(&self)->Result<Option<u64>,ValueError>{borrow_source_clock(&self.clock)?.retirement_work()}
 fn close_clock(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Option<semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,semio_framework_value_retained_clone::OriginalRetirementFailure>>{self.clock.get_mut().retirement_step(effect)}
 fn new(roots:SourceRoots)->Self{Self{authority:roots.authority,cancel:CancelTokenRetirement::from_token(roots.cancel),returns:roots.returns,clock:roots.clock}}
 fn terminal_is_empty(&self)->bool{self.authority.terminal_is_empty()&&self.cancel.terminal_is_empty()&&self.returns.terminal_is_empty()&&self.clock.try_lock().is_ok_and(|clock|clock.terminal_is_empty())}
 fn demands(&self)->Result<RetirementDemand,ValueError>{if let Some(demand)=self.clock_demand()?{return Ok(demand)}if !self.authority.terminal_is_empty(){Ok(self.authority.retirement_demands())}else if !self.cancel.terminal_is_empty(){self.cancel.retirement_demands()}else{self.returns.retirement_demands()}}
 fn retirement_work_demand(&self)->Result<u64,ValueError>{if let Some(work)=self.clock_work()?{return Ok(work)}if !self.authority.terminal_is_empty(){self.authority.retirement_work_demand()}else if !self.cancel.terminal_is_empty(){Ok(self.cancel.retirement_work_demand())}else{self.returns.retirement_work_demand()}}
 fn close_source(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,OriginalSourceCloseFailure>{use semio_framework_value_control_contract::RetainedWorkPoll;if let Some(poll)=self.close_clock(effect){return poll.map_err(OriginalSourceCloseFailure::Retirement)}if !self.authority.terminal_is_empty(){let turn=self.authority.close_step(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalSourceCloseFailure::Value),work:turn.work})}if !self.cancel.terminal_is_empty(){let turn=self.cancel.close_step(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalSourceCloseFailure::Cancellation),work:turn.work})}let turn=self.returns.close_step(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalSourceCloseFailure::Value),work:turn.work})}
}
impl RetireOwned for SourceRoots{
 type RetirementCursor=SourceRetirement;
 fn retirement_ready(&self)->bool{false}
 fn retirement_seed(self)->Self::RetirementCursor{SourceRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{None}
 fn controlled_retirement_supported()->bool{false}
}
impl RetirementCursor for SourceRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{self.retirement_work_demand()}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{self.demands()}
 fn close_into<'reply>(&mut self,_:usize,_:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetirementStep,semio_framework_value_retained_clone::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetirementStep,semio_framework_value_retained_clone::OriginalRetirementFailure>,ValueError>{Ok(reply.complete(Ok(if self.terminal_is_empty(){RetirementStep::Complete(0)}else{RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Source roots require their typed original close owner").into(),0)})))}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🏡️ See `README.md`.
pub struct OriginalCallerSourceParent{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,retiring:ManuallyDrop<Option<SourceRetirement>>,pending_roots:ManuallyDrop<Option<SourceRoots>>,pending_owner:ManuallyDrop<Option<semio_framework_value_shared_issuance::SharedOriginal<SourceRoots>>>,borrowed_cancel:bool,borrowed_source:bool,lending_open:bool,clock_authority:Option<(u64,fn()->Option<u64>,u64)>,resident_loans:std::sync::atomic::AtomicUsize}
impl OriginalCallerSourceParent{
 /// 🚪️ See `README.md`.
 pub fn lending_is_open(&self)->bool{self.lending_open}

 /// 🪪️ See `README.md`.
 pub fn source_identity(&self)->Result<usize,ValueError>{Ok(self.original.as_ref().ok_or_else(||refused("original source identity has entered physical retirement"))?.identity())}
 pub fn payload_authority_identity(&self)->Result<OriginalPayloadSourceIdentity,ValueError>{self.original.as_ref().and_then(|source|source.get().authority.identity()).ok_or_else(||refused("original payload source identity has entered physical retirement"))}
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original parent cancellation has entered physical retirement"))?.get().cancel)}
 pub fn payload_authority(&self)->Result<&JobPayloadAuthority,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original parent payload authority has entered physical retirement"))?.get().authority)}
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<SourceRoots>::birth_bytes(),depth:1,..Default::default()}}
/// ⛽️ Quotes the original source issuance and backing allocation attempts.
 pub fn admission_work_demand()->u64{SealedShared::<SourceRoots>::birth_work_demand()}
 pub fn borrow_context<'turn>(&'turn self,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress)->Result<StepContext<'turn>,ValueError>{if self.borrowed_cancel{return Err(refused("inherited original child context requires its live parent budget loan"))}self.borrow_context_with_return(budget,now_us,sequence,recipient,self.return_source()?.try_borrow().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return source already lends its budget"))?,None)}
 /// 🎫️ Issues one dispatch that a caller retains while an effect stays Pending across its turns.
 pub fn issue_dispatch(&self)->Result<OriginalSourceTurn,ValueError>{self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?.get().turns.issue().map_err(ValueError::from)}
 pub fn borrow_context_for_turn<'turn>(&'turn self,turn:&'turn OriginalSourceTurn,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress)->Result<StepContext<'turn>,ValueError>{if self.borrowed_cancel{return Err(refused("inherited original child context requires its live parent budget loan"))}let returns=self.return_source()?.try_borrow().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return source already lends its budget"))?;self.borrow_context_with_return(budget,now_us,sequence,recipient,returns,Some(turn))}
 pub(crate) fn borrow_context_with_return<'turn>(&'turn self,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress,returns:OriginalReturnBudgetReference<'turn>,turn:Option<&'turn OriginalSourceTurn>)->Result<StepContext<'turn>,ValueError>{let roots=self.original.as_ref().ok_or_else(||refused("original parent source has entered physical retirement"))?.get();let ledger=roots.authority.original().ok_or_else(||refused("original parent payload source has closed"))?;let mut cx=StepContext::with_payload_authority(ledger.operation,ledger.generation,budget,&roots.cancel,now_us,sequence,recipient,&roots.authority)?;cx.source_parent=Some(self);cx.source_turn=Some(match turn{Some(turn)=>{roots.turns.validate(turn).map_err(ValueError::from)?;SourceTurnReference::Borrowed(turn)},None=>SourceTurnReference::Owned(roots.turns.issue().map_err(ValueError::from)?)});cx.attach_original_return_budget(returns)?;if let Some((binding,clock,deadline))=self.clock_authority{if !std::ptr::fn_addr_eq(clock,now_us)||budget.deadline_us>deadline{return Err(refused("borrowed original source changed its clock or widened its absolute deadline"))}cx.original_clock_binding=Some(binding);cx.original_absolute_deadline=deadline;}Ok(cx)}
 /// 🔁️ Carries this Source's return stock into a verified successor tenure; identity, clock, deadline and claims stay.
 pub fn succeed_return_stock(&self,granted:RetainedCloneGrant,fuel:u64)->Result<(),ValueError>{self.return_source()?.succeed(granted,fuel)}
 pub fn return_source(&self)->Result<&OriginalReturnSource,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original return source has entered retirement"))?.get().returns)}
 /// 👁️ Reports whether the committed origin already serves this exact Source turn; births run only via the resident birth driver.
 pub(crate) fn admit_return_origin(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate_borrowed_context(cx)?;let turn=cx.original_source_turn_identity();Ok(cx.return_budget.as_ref().ok_or_else(||refused("original return source has no live budget loan"))?.get().origin().is_some_and(|origin|semio_framework_job_return_budget::OriginalReturnOrigin::source_turn(&origin)==turn))}
 pub fn validate_borrowed_context(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source parent has entered physical retirement"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original reply requires its genuine source receiver"))}Ok(())}
 /// 🕰️ Installs the system host clock into this Source's resident clock.
 pub fn admit_system_clock(&self,cx:&StepContext<'_>,host:&mut Option<semio_framework_job_clock::OriginalClockAuthority<semio_framework_job_clock::OriginalHostClock<semio_framework_job_clock::OriginalSystemClock>>>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if host.is_none(){let Some(original)=self.original.as_ref()else{return fail(refused("original Source clock entered retirement"))};*host=Some(semio_framework_job_clock::OriginalClockAuthority::new(semio_framework_job_clock::OriginalHostClock::new(&original.get().turns,semio_framework_job_clock::OriginalSystemClock::new(&original.get().turns))))}
  let context=match self.recorded_clock_context(cx){Ok(context)=>context,Err(error)=>return fail(error)};let mut clock=match self.source_clock(){Ok(clock)=>clock,Err(error)=>return fail(error)};clock.admit(&context,host,effect)
 }
 fn source_clock(&self)->Result<SourceClockLoan<'_>,ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original Source clock entered retirement"))?;borrow_source_clock(&original.get().clock)}
 /// 📏️ Quotes the actual Source clock producer before issuing Work or physical authority.
 pub fn recorded_clock_demand(&self,cx:&StepContext<'_>)->Result<semio_framework_job_clock::OriginalClockDemand,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.demand(&context)}
 /// 📨️ Drives or recovers the original Source clock reading in its resident action.
 pub fn poll_recorded_clock(&self,cx:&StepContext<'_>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<semio_framework_job_clock::OriginalClockDecisionStatus,ValueError>{let mut context=self.recorded_clock_context(cx)?;self.source_clock()?.poll_reading(&mut context,effect,cx.fuel_remaining())}
 /// 📤️ Transfers the exact recorded reading into a caller-owned slot.
 pub fn take_recorded_clock_reading(&self,cx:&StepContext<'_>,output:&mut Option<Result<semio_framework_job_clock::OriginalClockObservation,ValueError>>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.take_reading(&context,output)}
 /// 🏁️ Finishes only the exact original action after its reading transfers.
 pub fn finish_recorded_clock_reading(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.finish_reading(&context)}
 /// ↩️ Clears only a same-dispatch clock entry whose original action never started.
 pub fn cancel_unstarted_recorded_clock(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.cancel_unstarted_reading(&context)}
 /// 🎯️ Derives a deadline from the same Source clock original reading.
 pub fn recorded_clock_deadline(&self,cx:&StepContext<'_>,observation:semio_framework_job_clock::OriginalClockObservation,duration:u64)->Result<semio_framework_job_clock::OriginalClockDeadline,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.deadline(&context,observation,duration)}
 /// 🚦️ Drives the same recorded deadline decision across Pending and unwind.
 pub fn poll_recorded_clock_decision(&self,cx:&StepContext<'_>,deadline:semio_framework_job_clock::OriginalClockDeadline,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<semio_framework_job_clock::OriginalClockDecisionStatus,ValueError>{let mut context=self.recorded_clock_context(cx)?;self.source_clock()?.poll_decision(&mut context,effect,deadline,cx.original_turn_fuel_remaining())}
 /// 🛂️ Checks the actual resident clock decision against this Source wallet.
 pub fn recorded_clock_allows(&self,cx:&StepContext<'_>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.allows(&context,cx.original_turn_fuel_remaining())}
 /// 📤️ Transfers the original decision directly to the caller-owned slot.
 pub fn take_recorded_clock_decision(&self,cx:&StepContext<'_>,output:&mut Option<bool>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.take_decision(&context,cx.original_turn_fuel_remaining(),output)}
 /// 🏁️ Finishes the exact original deadline decision after transfer.
 pub fn finish_recorded_clock_decision(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.finish_decision(&context)}
 /// ↩️ Clears only a decision whose original producer never started.
 pub fn cancel_unstarted_recorded_clock_decision(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.cancel_unstarted_decision(&context)}

 /// 🕰️ Borrows the exact Source issuer and its still-owned dispatch for recorded clock context.
 pub fn recorded_clock_context<'turn>(&'turn self,cx:&'turn StepContext<'_>)->Result<semio_framework_job_clock::OriginalSystemClockContext<'turn>,ValueError>{self.validate_borrowed_context(cx)?;let roots=self.original.as_ref().ok_or_else(||refused("original Source clock entered retirement"))?.get();semio_framework_job_clock::OriginalSystemClockContext::new(&roots.turns,cx.original_source_turn()?)}
 /// 🕰️ Borrows the exact Source issuer and its still-owned dispatch for recorded clock entry.
 pub fn recorded_clock<'turn>(&'turn self,cx:&'turn StepContext<'_>)->Result<(&'turn semio_framework_job_clock::OriginalSourceClock,semio_framework_job_clock::OriginalSystemClockContext<'turn>),ValueError>{self.validate_borrowed_context(cx)?;let roots=self.original.as_ref().ok_or_else(||refused("original Source clock entered retirement"))?.get();let context=semio_framework_job_clock::OriginalSystemClockContext::new(&roots.turns,cx.original_source_turn()?)?;let guard=borrow_source_clock(&roots.clock)?;let clock=&*guard as *const semio_framework_job_clock::OriginalSourceClock;drop(guard);Ok((unsafe{&*clock},context))}
 /// 🎟️ Quotes the same counted alias action used by original loan admission.
 pub fn loan_admission_demand(&self)->Result<RetirementDemand,ValueError>{if !self.lending_open{return Err(refused("original source parent has entered loan closure"))}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if original.terminal_is_empty(){return Err(refused("original parent source has already returned"))}let mut demand=OriginalCallerSourceLoan::admission_demand();demand.copy_bytes+=SealedShared::<SourceRoots>::duplicate_copy_bytes();Ok(demand)}
 /// ⛽️ One counted shared-alias update is the actual loan admission action.
 pub fn loan_admission_work_demand(&self)->Result<u64,ValueError>{self.loan_admission_demand()?;let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;Ok(original.duplicate_work_demand()?+1)}
 /// 🎟️ The physical alias is real and its receipt already posted once it exists, so a refused receipt leaves the loan in `output` for its owner to retire through `close_step`; callers must never discard a refused admission's slot.
 pub fn admit_loan(&self,output:&mut Option<OriginalCallerSourceLoan>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(loan)=output.as_ref(){loan.validate_context(cx)?;if loan.source_identity()?!=self.source_identity()?{return Err(refused("original source loan slot belongs to another parent").into())}return Ok(true)}
  if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("original parent loan requires its actual incoming source context").into())}let demand=self.loan_admission_demand()?;if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()<self.loan_admission_work_demand()?||!semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false)}let binding=cx.original_clock_binding().ok_or_else(||refused("original source loan has no genuine bound clock authority"))?;let clock_authority=Some((binding,cx.clock_source(),cx.original_absolute_deadline_us()));let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|original.try_duplicate(effect))?else{return Ok(false)};let overrun=performed>turn.work;let mut work=turn.work;let result=match turn.result{Ok((alias,mut progress))=>{work+=1;progress.copied_bytes+=OriginalCallerSourceLoan::admission_demand().copy_bytes;*output=Some(OriginalCallerSourceLoan{source_identity:original.identity(),original:ManuallyDrop::new(Some(alias)),closing:false,clock_authority});Ok(RetainedCloneStep::Progress(progress))},Err(error)=>Err(error)};
  #[cfg(any(test,feature="testkit"))]crate::testkit::narrow_loan_receipt(cx);
  crate::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})?;if overrun{return Err(refused("original source admission performed more work than it settled").into())}Ok(output.is_some())
 }
 /// 🎟️ See `README.md`.
 pub fn admit_owned_turn<T>(&self,output:&mut Option<T>,cx:&mut StepContext<'_>,epoch:u64,make:impl FnOnce(OriginalCallerSourceParent,&StepContext<'_>,usize,u64,OriginalSourceTurn)->T)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(refused("owned original admission requires an empty caller receiver").into())}
  if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original admission requires its actual incoming source context").into())}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(false)}
  let binding=cx.original_clock_binding().ok_or_else(||refused("owned original admission lacks its bound clock authority"))?;let clock_authority=Some((binding,cx.clock_source(),cx.original_absolute_deadline_us()));let parent_identity=self.source_identity()?;let dispatch=original.get().turns.issue().map_err(ValueError::from)?;let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|original.try_duplicate(effect))?else{return Ok(false)};let overrun=performed>turn.work;let result=match turn.result{Ok((alias,progress))=>{let loan=OriginalCallerSourceLoan{source_identity:original.identity(),original:ManuallyDrop::new(Some(alias)),closing:false,clock_authority};*output=Some(make(loan.into_borrowed_parent(),cx,parent_identity,epoch,dispatch));Ok(RetainedCloneStep::Progress(progress))},Err(error)=>Err(error)};
  #[cfg(any(test,feature="testkit"))]crate::testkit::narrow_loan_receipt(cx);
  crate::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;if overrun{return Err(refused("original source admission performed more work than it settled").into())}Ok(output.is_some())
 }
 pub fn into_source_loan(mut self)->Result<OriginalCallerSourceLoan,Self>{if !self.borrowed_source||self.retiring.is_some()||self.pending_roots.is_some()||self.pending_owner.is_some(){return Err(self)}let source_identity=self.original.as_ref().expect("live borrowed source").identity();let original=self.original.take();Ok(OriginalCallerSourceLoan{source_identity,original:ManuallyDrop::new(original),closing:false,clock_authority:self.clock_authority})}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending_roots.is_none()&&self.pending_owner.is_none()&&self.retiring.is_none()}
 /// 🧹️ Quotes the original work that removes this emptied Source parent from its owner; its retirement quote is 0 once terminal because nothing remains to close, but the removal itself is paid by its owner.
 pub fn metadata_removal_work_demand()->u64{SOURCE_CONTROL_WORK}
 /// 🪺️ See `README.md`.
 pub fn original_loans_are_returned(&self)->bool{!self.borrowed_source&&self.original.as_ref().is_none_or(|original|original.original_aliases_are_returned_beyond(self.resident_loans.load(std::sync::atomic::Ordering::Acquire)))}
 /// 🌱️ Admits one loan that the resident return birth driver keeps for the life of this Source; it is not an outstanding borrower and is returned only by [`Self::release_resident_loan`].
 pub fn admit_resident_loan(&self,output:&mut Option<OriginalCallerSourceLoan>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let held=output.is_some();let admitted=self.admit_loan(output,cx);if !held&&output.is_some(){self.resident_loans.fetch_add(1,std::sync::atomic::Ordering::AcqRel);}admitted}
 /// ♻️ Records that the resident return birth driver has returned one of its loans.
 pub fn release_resident_loan(&self){self.resident_loans.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default());}
  if self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty()){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value_shared_issuance::SharedOriginal<SourceRoots>>>(),depth:1,..Default::default()})}if self.lending_open{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}if self.pending_roots.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SourceRoots>>()+size_of::<Option<SourceRetirement>>(),depth:1,..Default::default()})}
  if let Some(original)=self.original.as_ref(){return original.original_election_demand();}if let Some(owner)=self.pending_owner.as_ref(){return owner.take_value_demand();}
  self.retiring.as_ref().map_or(Ok(Default::default()),|retiring|{if retiring.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SourceRetirement>>()+size_of::<Option<(u64,fn()->Option<u64>,u64)>>(),depth:1,..Default::default()})}if let Some(demand)=retiring.clock_demand()?{return Ok(demand)}if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}retiring.demands()})
 }
 /// 📌️ True while a held return place, live claim or pending native Source still references the resident return birth, so neither it nor the Source may close.
 pub fn retirement_is_pinned(&self)->bool{self.native_sources_pending()||self.original.as_ref().is_some_and(|source|source.get().returns.retains_claims())}
 /// 🧮️ The selected parent close quote delegates every live child to its actual retirement owner.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if self.retirement_is_pinned(){return Ok(0)}if self.lending_open||self.pending_roots.is_some()||self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty()){return Ok(SOURCE_CONTROL_WORK)}if let Some(original)=self.original.as_ref(){return original.original_election_work_demand()}if let Some(owner)=self.pending_owner.as_ref(){return owner.take_value_work_demand()}let Some(retiring)=self.retiring.as_ref()else{return Ok(0)};if retiring.terminal_is_empty(){return Ok(SOURCE_CONTROL_WORK)}if let Some(work)=retiring.clock_work()?{return Ok(work)}if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty(){return retiring.cancel.return_alias_work_demand()}retiring.retirement_work_demand()}
 /// ♻️ Closes this exact Source with the caller's original performed-work effect.
 pub fn close_step(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,OriginalSourceCloseFailure>{
  use semio_framework_value_control_contract::RetainedWorkPoll;
  if self.borrowed_source{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalSourceCloseFailure::Value(refused("original borrowed parent must return its paid source loan to its issuer"))),work:0})}
  if self.borrowed_cancel&&!self.terminal_is_empty(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalSourceCloseFailure::Value(refused("child source physical close requires its still-retained original parent witness"))),work:0})}
  self.close_witnessed(effect,None)
 }
 #[doc(hidden)]
 pub fn clock_retirement_selected(&self)->Result<bool,ValueError>{if self.lending_open||self.original.is_some()||self.pending_owner.is_some()||self.pending_roots.is_some(){return Ok(false)}match self.retiring.as_ref(){Some(retiring)if !retiring.terminal_is_empty()=>Ok(borrow_source_clock(&retiring.clock)?.retirement_demand()?.is_some()),_=>Ok(false)}}
 #[doc(hidden)]
 pub fn close_clock_retirement_into<'reply>(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retained_clone::OriginalRetirementFailure>,recover:bool)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retained_clone::OriginalRetirementFailure>,ValueError>{let retiring=self.retiring.as_mut().ok_or_else(||refused("original Source clock retirement is absent"))?;if recover{retiring.clock.get_mut().recover_retirement_into(effect,reply)}else{retiring.clock.get_mut().retirement_into(effect,reply)}}
 #[doc(hidden)]
 pub fn close_witnessed(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,witness:Option<&CancelToken>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,OriginalSourceCloseFailure>{
  use semio_framework_value_control_contract::RetainedWorkPoll;
  let done=|result,work|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
  if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  if self.borrowed_cancel&&witness.is_none(){return done(Err(OriginalSourceCloseFailure::Value(refused("original child cancellation return has no parent witness"))),0)}
  if self.retirement_is_pinned(){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return done(Err(OriginalSourceCloseFailure::Value(error)),0)};
  if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if self.lending_open||self.pending_roots.is_some()||self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty())||self.retiring.as_ref().is_some_and(SourceRetirement::terminal_is_empty){match pay_source_close_metadata(effect){Ok(true)=>{},Ok(false)=>return done(Ok(RetainedCloneStep::Progress(Default::default())),0),Err((error,work))=>return done(Err(OriginalSourceCloseFailure::Value(error)),work)}}
  if self.lending_open{self.lending_open=false;return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})),u128::from(SOURCE_CONTROL_WORK))}
  if self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty()){self.pending_owner.take();return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<semio_framework_value_shared_issuance::SharedOriginal<SourceRoots>>>(),..Default::default()})),u128::from(SOURCE_CONTROL_WORK))}
  if self.original.is_some(){let turn=SealedShared::try_owner_into(&mut self.original,&mut self.pending_owner,effect);return done(turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(OriginalSourceCloseFailure::Value),turn.work)}
  if let Some(owner)=self.pending_owner.as_mut(){let turn=owner.take_value_step(&mut self.pending_roots,effect);return done(turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(OriginalSourceCloseFailure::Value),turn.work)}
  if let Some(roots)=self.pending_roots.take(){*self.retiring=Some(SourceRetirement::new(roots));return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<SourceRoots>>()+size_of::<Option<SourceRetirement>>(),..Default::default()})),u128::from(SOURCE_CONTROL_WORK))}
  let retiring=match self.retiring.as_mut(){Some(value)=>value,None=>return done(Err(OriginalSourceCloseFailure::Value(ValueError::invariant("retiring absent"))),0)};
  if retiring.terminal_is_empty(){self.retiring.take();self.clock_authority=None;return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<SourceRetirement>>()+size_of::<Option<(u64,fn()->Option<u64>,u64)>>(),..Default::default()})),u128::from(SOURCE_CONTROL_WORK))}
  if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty()&&matches!(retiring.clock_demand(),Ok(None)){let Some(witness)=witness else{return done(Err(OriginalSourceCloseFailure::Value(ValueError::invariant("witness absent"))),0)};let turn=retiring.cancel.return_alias_step(witness,effect);return done(turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(OriginalSourceCloseFailure::Cancellation),turn.work)}
  retiring.close_source(effect).map(|step|RetainedCloneStep::Progress(step.progress()))
 }
}
impl Drop for OriginalCallerSourceParent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original source parent reached Drop before paid child return and physical roots closure")}}

/// 🪢️ See `README.md`.
pub struct OriginalCallerSourceLoan{source_identity:usize,original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,closing:bool,clock_authority:Option<(u64,fn()->Option<u64>,u64)>}
impl OriginalCallerSourceLoan{
 /// 📏️ Quotes the actual fixed loan header published into its caller-owned slot.
 pub fn admission_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 /// ⛽️ The source alias update and resident header publication are distinct actual actions.
 pub fn admission_work_demand()->u64{2}

 /// 🪪️ See `README.md`.
 pub fn source_identity(&self)->Result<usize,ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source loan identity has already returned"))?;if original.terminal_is_empty(){return Err(refused("original source loan identity has already returned"))}Ok(original.identity())}
 /// 🕰️ See `README.md`.
 pub fn original_clock_source(&self)->Result<(u64,fn()->Option<u64>,u64),ValueError>{if self.original.is_none(){return Err(refused("original source clock loan has already returned"))}self.clock_authority.ok_or_else(||refused("original source loan has no clock authority"))}
 fn validates_clock(&self,cx:&StepContext<'_>)->bool{self.clock_authority.is_some_and(|(binding,clock,deadline)|cx.original_clock_binding()==Some(binding)&&std::ptr::fn_addr_eq(clock,cx.clock_source())&&deadline==cx.original_absolute_deadline_us())}
 /// 🔗️ See `README.md`.
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original source loan cancellation has already returned"))?.get().cancel)}
 pub fn into_borrowed_parent(mut self)->OriginalCallerSourceParent{assert!(!self.closing&&self.original.is_some(),"only the actual live original source loan can issue a borrowed parent");let original=self.original.take();OriginalCallerSourceParent{original:ManuallyDrop::new(original),retiring:ManuallyDrop::new(None),pending_roots:ManuallyDrop::new(None),pending_owner:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:true,lending_open:true,clock_authority:self.clock_authority,resident_loans:std::sync::atomic::AtomicUsize::new(0)}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source loan has already returned"))?;if self.closing||!matches_roots(original.get(),cx)||!self.validates_clock(cx){return Err(refused("original source loan refused foreign or closing caller context"))}Ok(())}
 pub fn validate_retirement_context(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if self.original.is_none(){return Err(refused("original source loan has already returned"))}let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if parent.source_identity()?!=self.source_identity||!self.validates_clock(cx){return Err(refused("original loan retirement requires its same actual issuer roots"))}Ok(())}
 pub fn run_close<'context>(&mut self,cx:&mut StepContext<'context>,requested:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'context>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<OriginalCallerCloseReturn,OriginalTurnFailure>{self.validate_context(cx)?;let before=cx.retained_progress();let recipient=cx.original_recipient_identity();let original=cx.with_retained_loan(requested,run);let after=cx.retained_progress();let report=match &original{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()};let receipt_refusal=if recipient!=cx.original_recipient_identity(){Some(refused("original owned caller return changed recipient"))}else{delta(before,after).and_then(|actual|if actual==report{Ok(())}else{Err(refused("original owned caller return differs from actual posted effects"))}).err()};Ok(OriginalCallerCloseReturn{original,receipt_refusal})}
 /// 🪙️ See `README.md`.
 /// ⛽️ Quotes the original selected flag, alias return or terminal slot release.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}Ok(1)}
 pub fn retirement_demands(&self)->RetirementDemand{if self.terminal_is_empty(){return Default::default()}if !self.closing{return RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}let original=self.original.as_ref().expect("live original loan");if original.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<SealedShared<SourceRoots>>>()+size_of::<Option<(u64,fn()->Option<u64>,u64)>>(),depth:1,..Default::default()}}else{original.borrowed_alias_return_demand()}}
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()});}
  let witness=parent.original.as_ref().ok_or_else(||refused("original parent must remain live through strong loan return"))?;
  if parent.source_identity()?!=self.source_identity||!matches_roots(witness.get(),cx)||!self.validates_clock(cx){return Err(refused("owned source loan return requires actual parent context").into());}
  if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Blocked);}
  let demand=self.retirement_demands();crate::close_original_retained_effect_step(cx,demand,|effect|{
   let paid=|effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>|->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"owned source loan return exceeded its Work"))}};
   if !self.closing{paid(effect)?;self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
   let original=self.original.as_ref().ok_or_else(||ValueError::invariant("original absent"))?;if original.terminal_is_empty(){paid(effect)?;self.original.take();self.clock_authority=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
   let original=self.original.as_mut().ok_or_else(||ValueError::invariant("original absent"))?;original.return_borrowed_alias(witness,effect).result.map(RetainedCloneStep::Progress)
  })
 }
}
impl Drop for OriginalCallerSourceLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original static source loan reached Drop before paid borrowed-parent return")}}
fn matches_roots(roots:&SourceRoots,cx:&StepContext<'_>)->bool{if cx.original_source_turn().map_or(true,|turn|roots.turns.validate(turn).is_err()){return false}roots.authority.original().is_some_and(|ledger|Arc::ptr_eq(ledger,&cx.payload_ledger)&&ledger.operation==cx.operation()&&ledger.generation==cx.generation())&&roots.cancel.is_same_node(cx.original_cancel_token())}
#[doc(hidden)]
pub fn cancel_refusal(error:CancelTokenRetirementError)->ValueError{match error{CancelTokenRetirementError::Refused(error)=>error,CancelTokenRetirementError::Blocked(blocked)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,match blocked{semio_framework_async::CancelTokenRetirementBlocked::SharedAlias=>"original cancellation node retains a shared alias",semio_framework_async::CancelTokenRetirementBlocked::WeakAlias=>"original cancellation node retains a weak alias",semio_framework_async::CancelTokenRetirementBlocked::RegisteredWaiter=>"original cancellation node retains a registered wake owner",semio_framework_async::CancelTokenRetirementBlocked::WaiterContention=>"original cancellation waiter guard is occupied"})}}


/// 🪢️ See `README.md`.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalCallerCloseReturn{original:Result<InteractiveJobCloseStep,OriginalTurnFailure>,receipt_refusal:Option<ValueError>}
impl OriginalCallerCloseReturn{pub fn new(original:Result<InteractiveJobCloseStep,OriginalTurnFailure>,receipt_refusal:Option<ValueError>)->Self{Self{original,receipt_refusal}}pub fn admitted(&self)->Option<&Result<InteractiveJobCloseStep,OriginalTurnFailure>>{self.receipt_refusal.is_none().then_some(&self.original)}pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}pub fn into_original(self)->Result<Result<InteractiveJobCloseStep,OriginalTurnFailure>,Self>{if self.receipt_refusal.is_some(){Err(self)}else{Ok(self.original)}}}
#[doc(hidden)]
pub fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
#[doc(hidden)]
pub fn metadata_fits(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();grant.maximum_items>0&&grant.maximum_depth>0}
#[doc(hidden)]
pub fn refused(message:&'static str)->ValueError{ValueError::invariant(message)}
#[doc(hidden)]
pub fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{let subtract=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||refused("original caller cumulative receipt moved backwards"));Ok(RetainedCloneProgress{copied_items:subtract(after.copied_items,before.copied_items)?,copied_bytes:subtract(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:subtract(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:subtract(after.released_bytes,before.released_bytes)?})}

#[cfg(test)]
mod source_clock_loan_tests{
 use super::*;
 #[test]fn actual_clock_loan_keeps_its_address_and_releases_after_unwind(){let owner=SourceClockOwner::new(semio_framework_job_clock::OriginalSourceClock::new());let pointer={let loan=borrow_source_clock(&owner).unwrap();std::ptr::from_ref(&*loan)};let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let loan=borrow_source_clock(&owner).unwrap();assert!(std::ptr::eq(&*loan,pointer));assert!(loan.terminal_is_empty());assert!(borrow_source_clock(&owner).is_err());panic!("original clock callback unwound")}));assert!(interrupted.is_err());let loan=borrow_source_clock(&owner).unwrap();assert!(std::ptr::eq(&*loan,pointer));assert!(loan.terminal_is_empty());drop(loan);assert!(borrow_source_clock(&owner).is_ok());}
}
impl OriginalCallerSourceLoan{
 /// ⏱️ Settles the exact same-Source Core receipt before returning its borrowed reply.
 pub fn settle_original_work<'receipt>(&self,cx:&mut StepContext<'_>,check:semio_framework_value_control_contract::OriginalWorkPartialCheck<'receipt>)->semio_framework_value_control_contract::OriginalWorkPartialCheckComplete<'receipt>{let result=(||{self.validate_retirement_context(cx)?;let turn=cx.original_source_turn_identity().ok_or_else(||refused("original Source Work turn is absent"))?;if check.source_turn()!=Some(turn){return Err(refused("original Source Work receipt changed its turn"))}cx.settle_original_work_exact(check.delta())})();check.complete(result)}
}
impl StepContext<'_>{
 /// 👶️ Lends this context's live clock, work, stock and return budget to one child turn over the same paid child parent and settles what that turn spent.
 #[doc(hidden)]
 pub fn lend_child_turn<T>(&mut self,child_parent:&OriginalCallerSourceParent,requested:RetainedCloneGrant,run:impl for<'turn> FnOnce(&mut StepContext<'turn>)->Result<T,OriginalTurnFailure>)->Result<(Result<T,OriginalTurnFailure>,Option<ValueError>,RetainedCloneProgress,RetainedCloneProgress),OriginalTurnFailure>{
  let cx=self;let before=cx.retained_progress();let original=OriginalRetainedLoanIdentity::capture(cx);let grant=narrowed_ceiling(cx,requested)?;let fuel=cx.fuel_remaining;let work=cx.work_units_spent;let clock=cx.clock.get();let source_epoch=cx.source_epoch;let payload_page_granted=cx.payload_page_granted;let stage=cx.stage;let reserved_progress=cx.reserved_return_progress;let reserved_work=cx.reserved_return_work;let total_fuel=cx.original_total_fuel_remaining();let absolute_deadline=cx.original_absolute_deadline_us();let budget=StepBudget{fuel,deadline_us:cx.deadline_us,retained:grant};
  let(result,matched,remaining,returned_clock,returned_page,returned_stage,returned_cpu,returned_reserved_progress,returned_reserved_work)={let returns=OriginalReturnBudgetReference::Borrowed(cx.return_budget.as_mut().ok_or_else(||refused("original child requires its same live source budget loan"))?.get_mut());let mut child=child_parent.borrow_context_with_return(budget,cx.now_us,cx.preview_sequence,cx.retained_progress,returns,None)?;if !std::ptr::eq(child.original_source_parent()?,child_parent){return Err(refused("original child context accessor changed its actual paid parent").into())}child.work_units_spent=work;child.reserved_return_progress=reserved_progress;child.reserved_return_work=reserved_work;child.clock.set(clock);child.source_epoch=source_epoch;child.original_retirement=cx.original_retirement;child.original_clock_binding=cx.original_clock_binding;child.original_total_fuel=total_fuel;child.original_total_work_start=work.saturating_sub(u128::from(reserved_work));child.original_absolute_deadline=absolute_deadline;child.original_total_retained=cx.original_total_retained;child.original_total_retained_start=cx.original_total_retained_start;child.retained_policy=cx.retained_policy;if matches!(cx.cancel,StepCancelReference::Observed(_)){child.observe_original_cancellation()?;}child.payload_page_granted=payload_page_granted;child.stage=stage;let identity=OriginalRetainedLoanIdentity::capture(&child);let result=run(&mut child);let matched=identity.matches(&child)&&child.source_epoch==source_epoch&&child.fuel_remaining<=fuel&&matches_roots(child_parent.original.as_ref().ok_or_else(||ValueError::invariant("original absent"))?.get(),&child);(result,matched,child.fuel_remaining,child.clock.get(),child.payload_page_granted,child.stage,child.work_units_spent,child.reserved_return_progress,child.reserved_return_work)};
  let after=cx.retained_progress();let receipt_refusal=if !matched||!original.matches(cx){Some(refused("original child callback replaced its live source, recipient or incoming authority"))}else{delta(before,after).err().or_else(||delta(returned_reserved_progress,after).and_then(|current|if current.fits(grant){Ok(())}else{Err(refused("original child callback exceeded its actual parent loan ceiling"))}).err())};if matched&&original.matches(cx){cx.fuel_remaining=remaining;cx.work_units_spent=returned_cpu;cx.reserved_return_progress=returned_reserved_progress;cx.reserved_return_work=returned_reserved_work;cx.clock.set(returned_clock);cx.payload_page_granted=returned_page;cx.stage=returned_stage;};Ok((result,receipt_refusal,before,after))
 }
}
pub(crate) fn narrowed_ceiling(cx:&StepContext<'_>,requested:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{let posted=delta(cx.reserved_return_progress,cx.retained_progress())?;if !posted.fits(cx.retained){return Err(refused("original child parent recipient already exceeds its incoming ceiling"))}let remaining=cx.retained_grant();let add=|paid:usize,wanted:usize,available:usize|paid.checked_add(wanted.min(available)).ok_or_else(||refused("original child loan ceiling overflow"));Ok(RetainedCloneGrant{maximum_items:add(posted.copied_items,requested.maximum_items,remaining.maximum_items)?,maximum_copy_bytes:add(posted.copied_bytes,requested.maximum_copy_bytes,remaining.maximum_copy_bytes)?,maximum_capacity_bytes:add(posted.retained_capacity_bytes,requested.maximum_capacity_bytes,remaining.maximum_capacity_bytes)?,maximum_release_bytes:add(posted.released_bytes,requested.maximum_release_bytes,remaining.maximum_release_bytes)?,maximum_depth:remaining.maximum_depth.min(requested.maximum_depth)})}
impl OriginalCallerSourceParent{
 /// 🧳️ Whether the strong source is still held.
 #[doc(hidden)]
 pub fn original_is_held(&self)->bool{self.original.is_some()}
 /// 🪪️ Authenticates the still-retained initiating roots of a child source against this live context and names them.
 #[doc(hidden)]
 pub fn child_witness(&self,cx:&StepContext<'_>)->Result<usize,ValueError>{let witness=self.original.as_ref().ok_or_else(||refused("original child parent source has already entered retirement"))?;if !matches_roots(witness.get(),cx){return Err(refused("original child source requires its same still-retained initiating parent"))}Ok(witness.identity())}
 /// 🌿️ Whether the borrowed cancellation alias of this child parent is still lent.
 #[doc(hidden)]
 pub fn borrowed_cancel(&self)->bool{self.borrowed_cancel}
 /// 🌿️ Records that this child parent lends the cancellation of its initiating parent.
 #[doc(hidden)]
 pub fn lend_borrowed_cancel(&mut self){self.borrowed_cancel=true}
}
impl OriginalCallerSourceLoan{
 /// 🪪️ Names the strong source this loan still holds.
 #[doc(hidden)]
 pub fn retained_identity(&self)->Option<usize>{self.original.as_ref().map(|original|original.identity())}
}
impl SourceRoots{
 /// 🏗️ Assembles fresh Source roots from the caller-owned payload and cancellation owners, taking them only once the birth identity exists.
 #[doc(hidden)]
 pub fn assemble(authority:&mut Option<JobPayloadAuthority>,cancel:&mut Option<CancelToken>)->Result<Self,ValueError>{let birth=ORIGINAL_RETURN_BIRTH.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).map(u64::from).map_err(|_|refused("Source birth identity exhausted"))?;let turns=OriginalSourceTurnIssuer::new();let returns=OriginalReturnSource::empty(birth);let native=native::NativeSources::new();let clock=SourceClockOwner::new(semio_framework_job_clock::OriginalSourceClock::new());let authority=authority.take().expect("original payload slot checked");let cancel=cancel.take().expect("original cancellation slot checked");Ok(Self{turns,authority,cancel,returns,native,clock})}
 /// 📤️ Returns the original payload and cancellation owners.
 #[doc(hidden)]
 pub fn into_inputs(self)->(JobPayloadAuthority,CancelToken){let Self{authority,cancel,..}=self;(authority,cancel)}
 /// 🪪️ Names the payload source these roots own.
 #[doc(hidden)]
 pub fn payload_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.authority.identity()}
 /// 🛑️ Borrows the cancellation root these roots own.
 #[doc(hidden)]
 pub fn cancel(&self)->&CancelToken{&self.cancel}
 /// 🧳️ Names the issuer that seals these roots.
 #[doc(hidden)]
 pub fn sealing_issuer()->SharedIssuer<Self>{Self::issuer()}
}
impl OriginalCallerSourceParent{
 /// 🌱️ Wraps freshly sealed roots as the parent whose lending is open.
 #[doc(hidden)]
 pub fn born(original:Option<SealedShared<SourceRoots>>)->Self{Self{original:ManuallyDrop::new(original),retiring:ManuallyDrop::new(None),pending_roots:ManuallyDrop::new(None),pending_owner:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:false,lending_open:true,clock_authority:None,resident_loans:std::sync::atomic::AtomicUsize::new(0)}}
}
