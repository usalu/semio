//! 🧳️ Genuine parent roots stay in one strong source while static children borrow actual incoming turns.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedCloneStep,retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::sealed::{SealedShared,SharedIssuer}}};

struct SourceRoots{authority:JobPayloadAuthority,cancel:CancelToken}
impl SourceRoots{fn issuer()->SharedIssuer<Self>{let mut issuer=SharedIssuer::owned();issuer.transfer_copy_bytes=|_|0;issuer}}
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
pub struct OriginalCallerSourceParent{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,retiring:ManuallyDrop<Option<SourceRetirement>>,borrowed_cancel:bool,borrowed_source:bool,lending_open:bool}
impl OriginalCallerSourceParent{
 /// 🪪️ Stable paid source identity survives moves of its original parent or borrowed wrapper.
 pub fn source_identity(&self)->Result<usize,ValueError>{Ok(self.original.as_ref().ok_or_else(||refused("original source identity has entered physical retirement"))?.identity())}
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original parent cancellation has entered physical retirement"))?.get().cancel)}
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<SourceRoots>::birth_bytes(),depth:1,..Default::default()}}
 pub fn admit(authority:JobPayloadAuthority,cancel:CancelToken,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,JobPayloadAuthority,CancelToken)>{if authority.terminal_is_empty(){return Err((refused("original parent requires its live paid payload source"),authority,cancel))}match SealedShared::admit(SourceRoots{authority,cancel},SourceRoots::issuer(),grant){Ok((original,progress))=>Ok((Self{original:ManuallyDrop::new(Some(original)),retiring:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:false,lending_open:true},progress)),Err((error,roots))=>Err((error,roots.authority,roots.cancel))}}
 pub fn borrow_context<'turn>(&'turn self,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress)->Result<StepContext<'turn>,ValueError>{let roots=self.original.as_ref().ok_or_else(||refused("original parent source has entered physical retirement"))?.get();let ledger=roots.authority.original().ok_or_else(||refused("original parent payload source has closed"))?;let mut cx=StepContext::with_payload_authority(ledger.operation,ledger.generation,budget,&roots.cancel,now_us,sequence,recipient,&roots.authority)?;cx.source_parent=Some(self);Ok(cx)}
 pub(crate) fn validate_borrowed_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original source parent has entered physical retirement"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original reply requires its genuine source receiver").into())}Ok(())}
 pub fn admit_loan(&self,cx:&mut StepContext<'_>)->Result<Option<OriginalCallerSourceLoan>,OriginalTurnFailure>{if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("original parent loan requires its actual incoming source context").into())}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(None)}let grant=cx.retained_grant();cx.consume_retained(metadata())?;let(alias,progress)=original.try_duplicate(grant)?;debug_assert_eq!(progress,metadata());cx.consume_fuel(1);Ok(Some(OriginalCallerSourceLoan{original:ManuallyDrop::new(Some(alias)),closing:false}))}
 pub(crate) fn into_source_loan(mut self)->Result<OriginalCallerSourceLoan,Self>{if !self.borrowed_source||self.retiring.is_some(){return Err(self)}let original=self.original.take();Ok(OriginalCallerSourceLoan{original:ManuallyDrop::new(original),closing:false})}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.retiring.is_none()}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if let Some(original)=self.original.as_ref(){return original.original_transfer_demand()}self.retiring.as_ref().map_or(Ok(Default::default()),|retiring|if self.borrowed_cancel&&retiring.authority.terminal_is_empty(){Ok(RetirementDemand{depth:usize::from(!retiring.cancel.terminal_is_empty()),..Default::default()})}else{retiring.demands()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.borrowed_source{return Err(refused("original borrowed parent must return its paid source loan to its issuer"))}if self.borrowed_cancel&&!self.terminal_is_empty(){return Err(refused("child source physical close requires its still-retained original parent witness"))}self.close_witnessed(grant,None)}
 fn close_witnessed(&mut self,grant:RetainedCloneGrant,witness:Option<&CancelToken>)->Result<RetainedCloneStep,ValueError>{self.lending_open=false;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if self.borrowed_cancel&&witness.is_none(){return Err(refused("original child cancellation return has no parent witness"))}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(original)=self.original.take(){return match original.try_unwrap(grant){Ok((roots,progress))=>{*self.retiring=Some(SourceRetirement::new(roots));Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.original=Some(original);Err(error)}}}let retiring=self.retiring.as_mut().unwrap();let step=if self.borrowed_cancel&&retiring.authority.terminal_is_empty(){retiring.cancel.return_alias_step(witness.unwrap(),grant).map_err(cancel_refusal)?}else{retiring.close_step(grant)?};if retiring.terminal_is_empty(){self.retiring.take();}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
}
impl Drop for OriginalCallerSourceParent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original source parent reached Drop before paid child return and physical roots closure")}}

/// 🪢️ One paid strong source loan moves into a static child without exposing roots or renewing caller authority.
pub struct OriginalCallerSourceLoan{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,closing:bool}
impl OriginalCallerSourceLoan{
 /// 🔗️ Borrows the exact retained original cancellation witness without creating or dropping an alias.
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original source loan cancellation has already returned"))?.get().cancel)}
 pub(crate) fn into_borrowed_parent(mut self)->OriginalCallerSourceParent{assert!(!self.closing&&self.original.is_some(),"only the actual live original source loan can issue a borrowed parent");let original=self.original.take();OriginalCallerSourceParent{original:ManuallyDrop::new(original),retiring:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:true,lending_open:true}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original source loan has already returned"))?;if self.closing||!matches_roots(original.get(),cx){return Err(refused("original source loan refused foreign or closing caller context").into())}Ok(())}
 pub fn validate_retirement_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original source loan has already returned"))?;if !matches_roots(original.get(),cx){return Err(refused("original loan retirement requires its same actual issuer roots").into())}Ok(())}
 pub fn run_close<'context>(&mut self,cx:&mut StepContext<'context>,requested:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'context>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<OriginalCallerCloseReturn,OriginalTurnFailure>{self.validate_context(cx)?;let before=cx.retained_progress();let recipient=cx.original_recipient_identity();let original=cx.with_retained_loan(requested,run);let after=cx.retained_progress();let report=match &original{Ok(step)=>step.progress(),Err(failure)=>failure.primary.retained_progress()};let receipt_refusal=if recipient!=cx.original_recipient_identity(){Some(refused("original owned caller return changed recipient"))}else{delta(before,after).and_then(|actual|if actual==report{Ok(())}else{Err(refused("original owned caller return differs from actual posted effects"))}).err()};Ok(OriginalCallerCloseReturn{original,receipt_refusal})}
 /// 🪙️ Quotes the owning source's fixed metadata return without deriving authority from the quote.
 pub fn retirement_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()}}
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}let witness=parent.original.as_ref().ok_or_else(||refused("original parent must remain live through strong loan return"))?;if !matches_roots(witness.get(),cx){return Err(refused("owned source loan return requires actual parent context").into())}self.closing=true;if cx.should_yield()||!metadata_fits(cx){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let progress=self.original.as_mut().unwrap().return_borrowed_alias(witness,cx.retained_grant())?;cx.consume_retained(progress)?;cx.consume_fuel(1);if self.original.as_ref().unwrap().terminal_is_empty(){self.original.take();}Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})}
}
impl Drop for OriginalCallerSourceLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original static source loan reached Drop before paid borrowed-parent return")}}
fn matches_roots(roots:&SourceRoots,cx:&StepContext<'_>)->bool{roots.authority.original().is_some_and(|ledger|Arc::ptr_eq(ledger,&cx.payload_ledger)&&ledger.operation==cx.operation()&&ledger.generation==cx.generation())&&roots.cancel.is_same_node(cx.original_cancel_token())}
fn cancel_refusal(error:CancelTokenRetirementError)->ValueError{match error{CancelTokenRetirementError::Refused(error)=>error,CancelTokenRetirementError::Blocked(_)=>refused("original child cancellation alias retains a blocked owner")}}
#[path="👶️child/🦀️.rs"]
mod child_source;
pub use child_source::{OriginalCallerChildSource,OriginalChildTurnReturn};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
