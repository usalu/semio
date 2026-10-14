//! 🧳️ Genuine parent roots stay in one strong source while static children borrow actual incoming turns.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedCloneStep,retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::sealed::{SealedShared,SharedIssuer}}};

struct SourceRoots{authority:JobPayloadAuthority,cancel:CancelToken}
struct SourceRetirement{authority:JobPayloadAuthority,cancel:CancelTokenRetirement}
impl SourceRetirement{
 fn new(roots:SourceRoots)->Self{Self{authority:roots.authority,cancel:CancelTokenRetirement::from_token(roots.cancel)}}
 fn terminal_is_empty(&self)->bool{self.authority.terminal_is_empty()&&self.cancel.terminal_is_empty()}
 fn demands(&self)->Result<RetirementDemand,ValueError>{if !self.authority.terminal_is_empty(){Ok(self.authority.retirement_demands())}else{self.cancel.retirement_demands()}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.authority.terminal_is_empty(){return self.authority.close_step(grant)}self.cancel.close_step(grant).map_err(|error|match error{CancelTokenRetirementError::Refused(error)=>error,CancelTokenRetirementError::Blocked(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original parent cancellation retains another owner")})}
}
impl RetireOwned for SourceRoots{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SourceRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SourceRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SourceRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete}match Self::close_step(self,grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🏡️ Owns already-paid original cancellation and payload sources before any child or context borrows them.
pub struct OriginalCallerSourceParent{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,retiring:ManuallyDrop<Option<SourceRetirement>>}
impl OriginalCallerSourceParent{
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<SourceRoots>::birth_bytes(),depth:1,..Default::default()}}
 pub fn admit(authority:JobPayloadAuthority,cancel:CancelToken,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,JobPayloadAuthority,CancelToken)>{if authority.terminal_is_empty(){return Err((refused("original parent requires its live paid payload source"),authority,cancel))}match SealedShared::admit(SourceRoots{authority,cancel},SharedIssuer::owned(),grant){Ok((original,progress))=>Ok((Self{original:ManuallyDrop::new(Some(original)),retiring:ManuallyDrop::new(None)},progress)),Err((error,roots))=>Err((error,roots.authority,roots.cancel))}}
 pub fn borrow_context<'turn>(&'turn self,budget:StepBudget,clock:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress)->Result<StepContext<'turn>,ValueError>{let roots=self.original.as_ref().ok_or_else(||refused("original parent source has entered physical retirement"))?.get();let ledger=roots.authority.original().ok_or_else(||refused("original parent payload source has closed"))?;StepContext::with_payload_authority(ledger.operation,ledger.generation,budget,&roots.cancel,clock,sequence,recipient,&roots.authority)}
 pub fn admit_loan(&self,cx:&mut StepContext<'_>)->Result<Option<OriginalCallerSourceLoan>,OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("original parent loan requires its actual incoming source context").into())}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(None)}let grant=cx.retained_grant();cx.consume_retained(metadata())?;let(alias,progress)=original.try_duplicate(grant)?;debug_assert_eq!(progress,metadata());cx.consume_fuel(1);Ok(Some(OriginalCallerSourceLoan{original:ManuallyDrop::new(Some(alias)),closing:false}))}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.retiring.is_none()}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<SourceRoots>(),release_bytes:SealedShared::<SourceRoots>::birth_bytes(),depth:1,..Default::default()})}self.retiring.as_ref().map_or(Ok(Default::default()),SourceRetirement::demands)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(original)=self.original.take(){return match original.try_unwrap(grant){Ok((roots,progress))=>{*self.retiring=Some(SourceRetirement::new(roots));Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.original=Some(original);Err(error)}}}let step=self.retiring.as_mut().unwrap().close_step(grant)?;if self.retiring.as_ref().unwrap().terminal_is_empty(){self.retiring.take();}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
}
impl Drop for OriginalCallerSourceParent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original source parent reached Drop before paid child return and physical roots closure")}}

/// 🪢️ One paid strong source loan moves into a static child without exposing roots or renewing caller authority.
pub struct OriginalCallerSourceLoan{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,closing:bool}
impl OriginalCallerSourceLoan{
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn run_close<'context>(&mut self,cx:&mut StepContext<'context>,requested:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'context>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<OriginalCallerCloseReturn,OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original source loan has already returned"))?;if self.closing||!matches_roots(original.get(),cx){return Err(refused("original source loan refused foreign or closing caller context").into())}let before=cx.retained_progress();let recipient=cx.original_recipient_identity();let original=cx.with_retained_loan(requested,run);let after=cx.retained_progress();let report=match &original{Ok(step)=>step.progress(),Err(failure)=>failure.primary.retained_progress()};let receipt_refusal=if recipient!=cx.original_recipient_identity(){Some(refused("original owned caller return changed recipient"))}else{delta(before,after).and_then(|actual|if actual==report{Ok(())}else{Err(refused("original owned caller return differs from actual posted effects"))}).err()};Ok(OriginalCallerCloseReturn{original,receipt_refusal})}
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}let witness=parent.original.as_ref().ok_or_else(||refused("original parent must remain live through strong loan return"))?;if !matches_roots(witness.get(),cx){return Err(refused("owned source loan return requires actual parent context").into())}self.closing=true;if cx.should_yield()||!metadata_fits(cx){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let progress=self.original.as_mut().unwrap().return_borrowed_alias(witness,cx.retained_grant())?;cx.consume_retained(progress)?;cx.consume_fuel(1);if self.original.as_ref().unwrap().terminal_is_empty(){self.original.take();}Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})}
}
impl Drop for OriginalCallerSourceLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original static source loan reached Drop before paid borrowed-parent return")}}
fn matches_roots(roots:&SourceRoots,cx:&StepContext<'_>)->bool{roots.authority.original().is_some_and(|ledger|Arc::ptr_eq(ledger,&cx.payload_ledger)&&ledger.operation==cx.operation()&&ledger.generation==cx.generation())&&std::ptr::eq(&roots.cancel,cx.original_cancel_token())}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
