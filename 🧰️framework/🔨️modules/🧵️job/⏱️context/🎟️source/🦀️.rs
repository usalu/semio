//! 🧳️ Genuine parent roots stay in one strong source while static children borrow actual incoming turns.
use super::*;
use semio_framework_value_retained_clone::{RetirementDemand,RetainedCloneStep,RetainedWorkOutcome,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use crate::original_job_continuation::{OriginalReturnSource,OriginalReturnSourceInput,OriginalReturnBudgetReference};

#[path="🏭️return-origin/🦀️.rs"]
mod return_origin;
pub use return_origin::OriginalReturnBirthState;
#[path="🏭️admission/🦀️.rs"]
mod source_admission;
pub use source_admission::OriginalCallerSourceAdmission;
#[path="🏭️close/🦀️.rs"]
mod source_close_producer;
pub use source_close_producer::{OriginalSourceCloseContext,OriginalSourceCloseProducer,source_loan_transfer_demand};
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
struct SourceRoots{turns:OriginalSourceTurnIssuer,authority:JobPayloadAuthority,cancel:CancelToken,returns:OriginalReturnSource,native:native::NativeSources,clock:SourceClockOwner}
impl SourceRoots{fn issuer()->SharedIssuer<Self>{let mut issuer=SharedIssuer::owned();issuer.transfer_copy_bytes=|_|0;issuer}}
struct SourceRetirement{authority:JobPayloadAuthority,cancel:CancelTokenRetirement,returns:OriginalReturnSource,clock:SourceClockOwner}
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
pub struct OriginalCallerSourceParent{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,retiring:ManuallyDrop<Option<SourceRetirement>>,pending_roots:ManuallyDrop<Option<SourceRoots>>,pending_owner:ManuallyDrop<Option<semio_framework_value_shared_issuance::SharedOriginal<SourceRoots>>>,borrowed_cancel:bool,borrowed_source:bool,lending_open:bool,clock_authority:Option<(u64,fn()->Option<u64>,u64)>}
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
 pub fn return_source(&self)->Result<&OriginalReturnSource,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original return source has entered retirement"))?.get().returns)}
 /// 👁️ Reports whether the committed origin already serves this exact Source turn; births run only through the resident birth driver.
 pub(crate) fn admit_return_origin(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate_borrowed_context(cx)?;let turn=cx.original_source_turn_identity();Ok(cx.return_budget.as_ref().ok_or_else(||refused("original return source has no live budget loan"))?.get().origin().is_some_and(|origin|semio_framework_job_return_budget::OriginalReturnOrigin::source_turn(&origin)==turn))}
 pub fn validate_borrowed_context(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source parent has entered physical retirement"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original reply requires its genuine source receiver"))}Ok(())}
 /// 🏭️ Binds the actual host owner to this Source without sampling or allocation.
 pub fn original_host_clock<P:semio_framework_job_clock::OriginalHostClockProducer>(&self,cx:&StepContext<'_>,host:P)->Result<semio_framework_job_clock::OriginalClockAuthority<semio_framework_job_clock::OriginalHostClock<P>>,(ValueError,P)>{if let Err(error)=self.validate_borrowed_context(cx){return Err((error,host))}let Some(original)=self.original.as_ref()else{return Err((refused("original Source clock entered retirement"),host))};Ok(semio_framework_job_clock::OriginalClockAuthority::new(semio_framework_job_clock::OriginalHostClock::new(&original.get().turns,host)))}
 /// 🕰️ Installs the system host clock into this Source's resident clock under a Source-carrying Work effect.
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
 /// 📤️ Transfers the actual clock or context refusal without replacing it.
 pub fn take_recorded_clock_refusal(&self,cx:&StepContext<'_>,output:&mut Option<ValueError>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.take_refusal(&context,output)}
 /// ↩️ Clears only a same-dispatch clock entry whose original action never started.
 pub fn cancel_unstarted_recorded_clock(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.cancel_unstarted_reading(&context)}
 /// 🎯️ Derives a deadline from the same Source clock original reading.
 pub fn recorded_clock_deadline(&self,cx:&StepContext<'_>,observation:semio_framework_job_clock::OriginalClockObservation,duration:u64)->Result<semio_framework_job_clock::OriginalClockDeadline,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.deadline(&context,observation,duration)}
 /// 🚦️ Drives the same recorded deadline decision across Pending and unwind.
 pub fn poll_recorded_clock_decision(&self,cx:&StepContext<'_>,deadline:semio_framework_job_clock::OriginalClockDeadline,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<semio_framework_job_clock::OriginalClockDecisionStatus,ValueError>{let mut context=self.recorded_clock_context(cx)?;self.source_clock()?.poll_decision(&mut context,effect,deadline,cx.fuel_remaining())}
 /// 🛂️ Checks the actual resident clock decision against this Source wallet.
 pub fn recorded_clock_allows(&self,cx:&StepContext<'_>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.allows(&context,cx.fuel_remaining())}
 /// 📤️ Transfers the original decision directly to the caller-owned slot.
 pub fn take_recorded_clock_decision(&self,cx:&StepContext<'_>,output:&mut Option<bool>)->Result<bool,ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.take_decision(&context,cx.fuel_remaining(),output)}
 /// 🏁️ Finishes the exact original deadline decision after transfer.
 pub fn finish_recorded_clock_decision(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.finish_decision(&context)}
 /// ↩️ Clears only a decision whose original producer never started.
 pub fn cancel_unstarted_recorded_clock_decision(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let context=self.recorded_clock_context(cx)?;self.source_clock()?.cancel_unstarted_decision(&context)}
 /// 🎟️ Admits the actual host into this Source with its original physical and Work effect.
 pub fn admit_recorded_clock<P:semio_framework_job_clock::OriginalHostClockProducer+semio_framework_value_retained_clone::retirement::RetireOwned+Send+'static>(&self,cx:&StepContext<'_>,host:&mut Option<semio_framework_job_clock::OriginalClockAuthority<semio_framework_job_clock::OriginalHostClock<P>>>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let context=match self.recorded_clock_context(cx){Ok(context)=>context,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};match self.source_clock(){Ok(mut clock)=>clock.admit(&context,host,effect),Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}}

 /// 🕰️ Borrows the exact Source issuer and its still-owned dispatch for recorded clock context.
 pub fn recorded_clock_context<'turn>(&'turn self,cx:&'turn StepContext<'_>)->Result<semio_framework_job_clock::OriginalSystemClockContext<'turn>,ValueError>{self.validate_borrowed_context(cx)?;let roots=self.original.as_ref().ok_or_else(||refused("original Source clock entered retirement"))?.get();semio_framework_job_clock::OriginalSystemClockContext::new(&roots.turns,cx.original_source_turn()?)}
 /// 🕰️ Borrows the exact Source issuer and its still-owned dispatch for recorded clock entry.
 pub fn recorded_clock<'turn>(&'turn self,cx:&'turn StepContext<'_>)->Result<(&'turn semio_framework_job_clock::OriginalSourceClock,semio_framework_job_clock::OriginalSystemClockContext<'turn>),ValueError>{self.validate_borrowed_context(cx)?;let roots=self.original.as_ref().ok_or_else(||refused("original Source clock entered retirement"))?.get();let context=semio_framework_job_clock::OriginalSystemClockContext::new(&roots.turns,cx.original_source_turn()?)?;let guard=borrow_source_clock(&roots.clock)?;let clock=&*guard as *const semio_framework_job_clock::OriginalSourceClock;drop(guard);Ok((unsafe{&*clock},context))}
 /// 🎟️ Quotes the same counted alias action used by original loan admission.
 pub fn loan_admission_demand(&self)->Result<RetirementDemand,ValueError>{if !self.lending_open{return Err(refused("original source parent has entered loan closure"))}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if original.terminal_is_empty(){return Err(refused("original parent source has already returned"))}Ok(OriginalCallerSourceLoan::admission_demand())}
 /// ⛽️ One counted shared-alias update is the actual loan admission action.
 pub fn loan_admission_work_demand(&self)->Result<u64,ValueError>{self.loan_admission_demand()?;let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;Ok(original.duplicate_work_demand()?+1)}
 /// 🎟️ The physical alias is real and its receipt already posted once it exists, so a refused receipt leaves the loan in `output` for its owner to retire through `close_step`; callers must never discard a refused admission's slot.
 pub fn admit_loan(&self,output:&mut Option<OriginalCallerSourceLoan>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(loan)=output.as_ref(){loan.validate_context(cx)?;if loan.source_identity()?!=self.source_identity()?{return Err(refused("original source loan slot belongs to another parent").into())}return Ok(true)}
  if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("original parent loan requires its actual incoming source context").into())}let demand=self.loan_admission_demand()?;if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()<self.loan_admission_work_demand()?||!semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false)}let binding=cx.original_clock_binding().ok_or_else(||refused("original source loan has no genuine bound clock authority"))?;let clock_authority=Some((binding,cx.clock_source(),cx.original_absolute_deadline_us()));let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|original.try_duplicate(effect))?else{return Ok(false)};let overrun=performed>turn.work;let mut work=turn.work;let result=match turn.result{Ok((alias,mut progress))=>{work+=1;progress.copied_bytes+=demand.copy_bytes;*output=Some(OriginalCallerSourceLoan{source_identity:original.identity(),original:ManuallyDrop::new(Some(alias)),closing:false,clock_authority});Ok(RetainedCloneStep::Progress(progress))},Err(error)=>Err(error)};
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
 pub fn original_loans_are_returned(&self)->bool{!self.borrowed_source&&self.original.as_ref().is_none_or(SealedShared::original_aliases_are_returned)}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default());}
  if self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty()){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value_shared_issuance::SharedOriginal<SourceRoots>>>(),depth:1,..Default::default()})}if self.lending_open{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}if self.pending_roots.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SourceRoots>>()+size_of::<Option<SourceRetirement>>(),depth:1,..Default::default()})}
  if let Some(original)=self.original.as_ref(){return original.original_election_demand();}if let Some(owner)=self.pending_owner.as_ref(){return owner.take_value_demand();}
  self.retiring.as_ref().map_or(Ok(Default::default()),|retiring|{if retiring.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SourceRetirement>>()+size_of::<Option<(u64,fn()->Option<u64>,u64)>>(),depth:1,..Default::default()})}if let Some(demand)=retiring.clock_demand()?{return Ok(demand)}if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}retiring.demands()})
 }
 /// 🧮️ The selected parent close quote delegates every live child to its actual retirement owner.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if self.native_sources_pending()||self.original.as_ref().is_some_and(|source|source.get().returns.retains_claims()){return Ok(0)}if self.lending_open||self.pending_roots.is_some()||self.pending_owner.as_ref().is_some_and(|owner|owner.terminal_is_empty()){return Ok(SOURCE_CONTROL_WORK)}if let Some(original)=self.original.as_ref(){return original.original_election_work_demand()}if let Some(owner)=self.pending_owner.as_ref(){return owner.take_value_work_demand()}let Some(retiring)=self.retiring.as_ref()else{return Ok(0)};if retiring.terminal_is_empty(){return Ok(SOURCE_CONTROL_WORK)}if let Some(work)=retiring.clock_work()?{return Ok(work)}if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty(){return retiring.cancel.return_alias_work_demand()}retiring.retirement_work_demand()}
 /// ♻️ Closes this exact Source with the caller's original performed-work effect.
 pub fn close_step(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,OriginalSourceCloseFailure>{
  use semio_framework_value_control_contract::RetainedWorkPoll;
  if self.borrowed_source{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalSourceCloseFailure::Value(refused("original borrowed parent must return its paid source loan to its issuer"))),work:0})}
  if self.borrowed_cancel&&!self.terminal_is_empty(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalSourceCloseFailure::Value(refused("child source physical close requires its still-retained original parent witness"))),work:0})}
  self.close_witnessed(effect,None)
 }
 fn clock_retirement_selected(&self)->Result<bool,ValueError>{if self.lending_open||self.original.is_some()||self.pending_owner.is_some()||self.pending_roots.is_some(){return Ok(false)}match self.retiring.as_ref(){Some(retiring)if !retiring.terminal_is_empty()=>Ok(borrow_source_clock(&retiring.clock)?.retirement_demand()?.is_some()),_=>Ok(false)}}
 fn close_clock_retirement_into<'reply>(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retained_clone::OriginalRetirementFailure>,recover:bool)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retained_clone::OriginalRetirementFailure>,ValueError>{let retiring=self.retiring.as_mut().ok_or_else(||refused("original Source clock retirement is absent"))?;if recover{retiring.clock.get_mut().recover_retirement_into(effect,reply)}else{retiring.clock.get_mut().retirement_into(effect,reply)}}
 fn close_witnessed(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,witness:Option<&CancelToken>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,OriginalSourceCloseFailure>{
  use semio_framework_value_control_contract::RetainedWorkPoll;
  let done=|result,work|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
  if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  if self.borrowed_cancel&&witness.is_none(){return done(Err(OriginalSourceCloseFailure::Value(refused("original child cancellation return has no parent witness"))),0)}
  if self.native_sources_pending()||self.original.as_ref().is_some_and(|source|source.get().returns.retains_claims()){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
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
 pub fn into_borrowed_parent(mut self)->OriginalCallerSourceParent{assert!(!self.closing&&self.original.is_some(),"only the actual live original source loan can issue a borrowed parent");let original=self.original.take();OriginalCallerSourceParent{original:ManuallyDrop::new(original),retiring:ManuallyDrop::new(None),pending_roots:ManuallyDrop::new(None),pending_owner:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:true,lending_open:true,clock_authority:self.clock_authority}}
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
fn cancel_refusal(error:CancelTokenRetirementError)->ValueError{match error{CancelTokenRetirementError::Refused(error)=>error,CancelTokenRetirementError::Blocked(blocked)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,match blocked{semio_framework_async::CancelTokenRetirementBlocked::SharedAlias=>"original cancellation node retains a shared alias",semio_framework_async::CancelTokenRetirementBlocked::WeakAlias=>"original cancellation node retains a weak alias",semio_framework_async::CancelTokenRetirementBlocked::RegisteredWaiter=>"original cancellation node retains a registered wake owner",semio_framework_async::CancelTokenRetirementBlocked::WaiterContention=>"original cancellation waiter guard is occupied"})}}

#[path="👶️child/🦀️.rs"]
mod child_source;
pub use child_source::{OriginalCallerChildSource,OriginalChildTurnReturn};

/// 🪢️ See `README.md`.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalCallerCloseReturn{original:Result<InteractiveJobCloseStep,OriginalTurnFailure>,receipt_refusal:Option<ValueError>}
impl OriginalCallerCloseReturn{pub fn new(original:Result<InteractiveJobCloseStep,OriginalTurnFailure>,receipt_refusal:Option<ValueError>)->Self{Self{original,receipt_refusal}}pub fn admitted(&self)->Option<&Result<InteractiveJobCloseStep,OriginalTurnFailure>>{self.receipt_refusal.is_none().then_some(&self.original)}pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}pub fn into_original(self)->Result<Result<InteractiveJobCloseStep,OriginalTurnFailure>,Self>{if self.receipt_refusal.is_some(){Err(self)}else{Ok(self.original)}}}
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn metadata_fits(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();grant.maximum_items>0&&grant.maximum_depth>0}
fn refused(message:&'static str)->ValueError{ValueError::invariant(message)}
fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{let subtract=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||refused("original caller cumulative receipt moved backwards"));Ok(RetainedCloneProgress{copied_items:subtract(after.copied_items,before.copied_items)?,copied_bytes:subtract(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:subtract(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:subtract(after.released_bytes,before.released_bytes)?})}

#[cfg(test)]
mod source_clock_loan_tests{
 use super::*;
 #[test]fn actual_clock_loan_keeps_its_address_and_releases_after_unwind(){let owner=SourceClockOwner::new(semio_framework_job_clock::OriginalSourceClock::new());let pointer={let loan=borrow_source_clock(&owner).unwrap();std::ptr::from_ref(&*loan)};let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let loan=borrow_source_clock(&owner).unwrap();assert!(std::ptr::eq(&*loan,pointer));assert!(loan.terminal_is_empty());assert!(borrow_source_clock(&owner).is_err());panic!("original clock callback unwound")}));assert!(interrupted.is_err());let loan=borrow_source_clock(&owner).unwrap();assert!(std::ptr::eq(&*loan,pointer));assert!(loan.terminal_is_empty());drop(loan);assert!(borrow_source_clock(&owner).is_ok());}
}
