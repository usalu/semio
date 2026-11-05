//! 🧳️ Genuine parent roots stay in one strong source while static children borrow actual incoming turns.
use super::*;
use semio_framework_value_retained_clone::{RetirementDemand,RetainedCloneStep,RetainedWorkOutcome,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use crate::original_job_continuation::{OriginalReturnSource,OriginalReturnSourceInput,OriginalReturnBudgetReference};

#[path="🏠️native/🦀️.rs"]
mod native;
pub use native::{ORIGINAL_NATIVE_SOURCE_CAPACITY,OriginalNativeLease,OriginalNativeRetirement,OriginalNativeBorrow,OriginalNativeRecipientIdentity,OriginalNativeRetiredWitness};
struct SourceRoots{authority:JobPayloadAuthority,cancel:CancelToken,returns:OriginalReturnSource,native:native::NativeSources}
impl SourceRoots{fn issuer()->SharedIssuer<Self>{let mut issuer=SharedIssuer::owned();issuer.transfer_copy_bytes=|_|0;issuer}}
struct SourceRetirement{authority:JobPayloadAuthority,cancel:CancelTokenRetirement,returns:OriginalReturnSource}
impl SourceRetirement{
 fn new(roots:SourceRoots)->Self{Self{authority:roots.authority,cancel:CancelTokenRetirement::from_token(roots.cancel),returns:roots.returns}}
 fn terminal_is_empty(&self)->bool{self.authority.terminal_is_empty()&&self.cancel.terminal_is_empty()&&self.returns.terminal_is_empty()}
 fn demands(&self)->Result<RetirementDemand,ValueError>{if !self.authority.terminal_is_empty(){Ok(self.authority.retirement_demands())}else if !self.cancel.terminal_is_empty(){self.cancel.retirement_demands()}else{Ok(RetirementDemand{depth:usize::from(!self.returns.terminal_is_empty()),..Default::default()})}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{if !self.authority.terminal_is_empty(){return self.authority.close_step(grant)}if self.cancel.terminal_is_empty(){return self.returns.close_step(grant)}let turn=self.cancel.close_step(grant);RetainedWorkOutcome{result:turn.result.map_err(cancel_refusal),work:turn.work}}
}
impl RetireOwned for SourceRoots{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SourceRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SourceRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SourceRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0)}let turn=Self::close_step(self,grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🏡️ Owns already-paid original cancellation and payload sources before any child or context borrows them.
pub struct OriginalCallerSourceParent{original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,retiring:ManuallyDrop<Option<SourceRetirement>>,pending_roots:ManuallyDrop<Option<SourceRoots>>,borrowed_cancel:bool,borrowed_source:bool,lending_open:bool,clock_authority:Option<(u64,fn()->Option<u64>,u64)>}
impl OriginalCallerSourceParent{
 /// 🚪️ Observes whether this original parent still accepts new genuine loans.
 pub fn lending_is_open(&self)->bool{self.lending_open}

 /// 🪪️ Stable paid source identity survives moves of its original parent or borrowed wrapper.
 pub fn source_identity(&self)->Result<usize,ValueError>{Ok(self.original.as_ref().ok_or_else(||refused("original source identity has entered physical retirement"))?.identity())}
 pub fn payload_authority_identity(&self)->Result<OriginalPayloadSourceIdentity,ValueError>{self.original.as_ref().and_then(|source|source.get().authority.identity()).ok_or_else(||refused("original payload source identity has entered physical retirement"))}
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original parent cancellation has entered physical retirement"))?.get().cancel)}
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<SourceRoots>::birth_bytes(),depth:1,..Default::default()}}
 pub fn admit(authority:JobPayloadAuthority,cancel:CancelToken,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,JobPayloadAuthority,CancelToken)>{if authority.terminal_is_empty(){return RetainedWorkOutcome{result:Err((refused("original parent requires its live paid payload source"),authority,cancel)),work:0}}let turn=SealedShared::admit(SourceRoots{authority,cancel,returns:OriginalReturnSource::empty(),native:native::NativeSources::new()},SourceRoots::issuer(),grant);RetainedWorkOutcome{work:turn.work,result:match turn.result{Ok((original,progress))=>Ok((Self{original:ManuallyDrop::new(Some(original)),retiring:ManuallyDrop::new(None),pending_roots:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:false,lending_open:true,clock_authority:None},progress)),Err((error,roots))=>Err((error,roots.authority,roots.cancel))}}}
 pub fn borrow_context<'turn>(&'turn self,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress)->Result<StepContext<'turn>,ValueError>{if self.borrowed_cancel{return Err(refused("inherited original child context requires its live parent budget loan"))}self.borrow_context_with_return(budget,now_us,sequence,recipient,self.return_source()?.try_borrow().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return source already lends its budget"))?)}
 pub(crate) fn borrow_context_with_return<'turn>(&'turn self,budget:StepBudget,now_us:fn()->Option<u64>,sequence:&'turn mut u64,recipient:&'turn mut RetainedCloneProgress,returns:OriginalReturnBudgetReference<'turn>)->Result<StepContext<'turn>,ValueError>{let roots=self.original.as_ref().ok_or_else(||refused("original parent source has entered physical retirement"))?.get();let ledger=roots.authority.original().ok_or_else(||refused("original parent payload source has closed"))?;let mut cx=StepContext::with_payload_authority(ledger.operation,ledger.generation,budget,&roots.cancel,now_us,sequence,recipient,&roots.authority)?;cx.source_parent=Some(self);cx.attach_original_return_budget(returns)?;if let Some((binding,clock,deadline))=self.clock_authority{if !std::ptr::fn_addr_eq(clock,now_us)||budget.deadline_us>deadline{return Err(refused("borrowed original source changed its clock or widened its absolute deadline"))}cx.original_clock_binding=Some(binding);cx.original_absolute_deadline=deadline;}Ok(cx)}
 pub(crate) fn return_source(&self)->Result<&OriginalReturnSource,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original return source has entered retirement"))?.get().returns)}
 pub(crate) fn admit_return_origin(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate_borrowed_context(cx)?;let input=OriginalReturnSourceInput::capture(cx)?;let bound=cx.return_budget.as_ref().ok_or_else(||refused("original return source has no live budget loan"))?.get().is_bound();if bound{cx.return_budget.as_mut().ok_or_else(||ValueError::invariant("return_budget absent"))?.get_mut().bind(input)?;return Ok(true)}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(false)}cx.return_budget.as_mut().ok_or_else(||ValueError::invariant("return_budget absent"))?.get_mut().bind(input)?;crate::receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(metadata())),work:1})?;Ok(true)}
 pub fn validate_borrowed_context(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source parent has entered physical retirement"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original reply requires its genuine source receiver"))}Ok(())}
 pub fn admit_loan(&self,output:&mut Option<OriginalCallerSourceLoan>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(loan)=output.as_ref(){loan.validate_context(cx)?;if loan.source_identity()?!=self.source_identity()?{return Err(refused("original source loan slot belongs to another parent").into())}return Ok(true)}
  if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("original parent loan requires its actual incoming source context").into())}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(false)}let binding=cx.original_clock_binding().ok_or_else(||refused("original source loan has no genuine bound clock authority"))?;let clock_authority=Some((binding,cx.clock_source(),cx.original_absolute_deadline_us()));let turn=original.try_duplicate(cx.retained_grant());let result=match turn.result{Ok((alias,progress))=>{*output=Some(OriginalCallerSourceLoan{source_identity:original.identity(),original:ManuallyDrop::new(Some(alias)),closing:false,clock_authority});Ok(RetainedCloneStep::Progress(progress))},Err(error)=>Err(error)};
  #[cfg(any(test,feature="testkit"))]crate::testkit::narrow_loan_receipt(cx);
  crate::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(output.is_some())
 }
 /// 🎟️ Places the real new alias and its owned caller header before the actual alias receipt is posted.
 pub(crate) fn admit_owned_turn(&self,output:&mut Option<crate::OriginalOwnedCallerTurn>,cx:&mut StepContext<'_>,epoch:u64)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(refused("owned original admission requires an empty caller receiver").into())}
  if !self.lending_open{return Err(refused("original source parent has entered loan closure").into())}let original=self.original.as_ref().ok_or_else(||refused("original parent has no source to lend"))?;if !matches_roots(original.get(),cx){return Err(refused("owned original admission requires its actual incoming source context").into())}if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(false)}
  let binding=cx.original_clock_binding().ok_or_else(||refused("owned original admission lacks its bound clock authority"))?;let clock_authority=Some((binding,cx.clock_source(),cx.original_absolute_deadline_us()));let parent_identity=self.source_identity()?;let turn=original.try_duplicate(cx.retained_grant());let result=match turn.result{Ok((alias,progress))=>{let loan=OriginalCallerSourceLoan{source_identity:original.identity(),original:ManuallyDrop::new(Some(alias)),closing:false,clock_authority};*output=Some(crate::OriginalOwnedCallerTurn::from_admitted_source(loan.into_borrowed_parent(),cx,parent_identity,epoch));Ok(RetainedCloneStep::Progress(progress))},Err(error)=>Err(error)};
  #[cfg(any(test,feature="testkit"))]crate::testkit::narrow_loan_receipt(cx);
  crate::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(output.is_some())
 }
 pub(crate) fn into_source_loan(mut self)->Result<OriginalCallerSourceLoan,Self>{if !self.borrowed_source||self.retiring.is_some()||self.pending_roots.is_some(){return Err(self)}let source_identity=self.original.as_ref().expect("live borrowed source").identity();let original=self.original.take();Ok(OriginalCallerSourceLoan{source_identity,original:ManuallyDrop::new(original),closing:false,clock_authority:self.clock_authority})}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending_roots.is_none()&&self.retiring.is_none()}
 /// 🪺️ Reads the paid source's actual strong-loan return without changing its lending authority.
 pub fn original_loans_are_returned(&self)->bool{!self.borrowed_source&&self.original.as_ref().is_none_or(SealedShared::original_aliases_are_returned)}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default());}
  if self.lending_open||self.pending_roots.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(original)=self.original.as_ref(){return original.original_transfer_demand();}
  self.retiring.as_ref().map_or(Ok(Default::default()),|retiring|if retiring.terminal_is_empty()||self.borrowed_cancel&&retiring.authority.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{retiring.demands()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.borrowed_source{return RetainedWorkOutcome{result:Err(refused("original borrowed parent must return its paid source loan to its issuer")),work:0};}
  if self.borrowed_cancel&&!self.terminal_is_empty(){return RetainedWorkOutcome{result:Err(refused("child source physical close requires its still-retained original parent witness")),work:0};}
  self.close_witnessed(grant,None)
 }
 fn close_witnessed(&mut self,grant:RetainedCloneGrant,witness:Option<&CancelToken>)->RetainedWorkOutcome{
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
  if self.borrowed_cancel&&witness.is_none(){return RetainedWorkOutcome{result:Err(refused("original child cancellation return has no parent witness")),work:0};}
  if self.native_sources_pending()||self.original.as_ref().is_some_and(|source|source.get().returns.retains_claims()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  if self.lending_open{self.lending_open=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1};}
  if let Some(original)=self.original.take(){
   let turn=original.try_unwrap(grant);let result=match turn.result{Ok((roots,progress))=>{*self.pending_roots=Some(roots);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.original=Some(original);Err(error)}};
   return RetainedWorkOutcome{result,work:turn.work};
  }
  if let Some(roots)=self.pending_roots.take(){*self.retiring=Some(SourceRetirement::new(roots));return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1};}
  let retiring=match self.retiring.as_mut(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("retiring absent").into()),work:0}};
  if retiring.terminal_is_empty(){self.retiring.take();self.clock_authority=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1};}
  let turn=if self.borrowed_cancel&&retiring.authority.terminal_is_empty()&&!retiring.cancel.terminal_is_empty(){let turn=retiring.cancel.return_alias_step(match witness{Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("witness absent").into()),work:0}},grant);RetainedWorkOutcome{result:turn.result.map_err(cancel_refusal),work:turn.work}}else{retiring.close_step(grant)};
  RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}
 }
}
impl Drop for OriginalCallerSourceParent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original source parent reached Drop before paid child return and physical roots closure")}}

/// 🪢️ One paid strong source loan moves into a static child without exposing roots or renewing caller authority.
pub struct OriginalCallerSourceLoan{source_identity:usize,original:ManuallyDrop<Option<SealedShared<SourceRoots>>>,closing:bool,clock_authority:Option<(u64,fn()->Option<u64>,u64)>}
impl OriginalCallerSourceLoan{
 /// 🪪️ Reads the actual still-retained strong source identity without issuing any alias or authority.
 pub fn source_identity(&self)->Result<usize,ValueError>{let original=self.original.as_ref().ok_or_else(||refused("original source loan identity has already returned"))?;if original.terminal_is_empty(){return Err(refused("original source loan identity has already returned"))}Ok(original.identity())}
 /// 🕰️ Only the still-owned genuine source loan supplies its immutable opaque clock binding.
 pub fn original_clock_source(&self)->Result<(u64,fn()->Option<u64>,u64),ValueError>{if self.original.is_none(){return Err(refused("original source clock loan has already returned"))}self.clock_authority.ok_or_else(||refused("original source loan has no clock authority"))}
 fn validates_clock(&self,cx:&StepContext<'_>)->bool{self.clock_authority.is_some_and(|(binding,clock,deadline)|cx.original_clock_binding()==Some(binding)&&std::ptr::fn_addr_eq(clock,cx.clock_source())&&deadline==cx.original_absolute_deadline_us())}
 /// 🔗️ Borrows the exact retained original cancellation witness without creating or dropping an alias.
 pub fn cancellation(&self)->Result<&CancelToken,ValueError>{Ok(&self.original.as_ref().ok_or_else(||refused("original source loan cancellation has already returned"))?.get().cancel)}
 pub(crate) fn into_borrowed_parent(mut self)->OriginalCallerSourceParent{assert!(!self.closing&&self.original.is_some(),"only the actual live original source loan can issue a borrowed parent");let original=self.original.take();OriginalCallerSourceParent{original:ManuallyDrop::new(original),retiring:ManuallyDrop::new(None),pending_roots:ManuallyDrop::new(None),borrowed_cancel:false,borrowed_source:true,lending_open:true,clock_authority:self.clock_authority}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||refused("original source loan has already returned"))?;if self.closing||!matches_roots(original.get(),cx)||!self.validates_clock(cx){return Err(refused("original source loan refused foreign or closing caller context").into())}Ok(())}
 pub fn validate_retirement_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.original.is_none(){return Err(refused("original source loan has already returned").into())}let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if parent.source_identity()?!=self.source_identity||!self.validates_clock(cx){return Err(refused("original loan retirement requires its same actual issuer roots").into())}Ok(())}
 pub fn run_close<'context>(&mut self,cx:&mut StepContext<'context>,requested:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'context>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<OriginalCallerCloseReturn,OriginalTurnFailure>{self.validate_context(cx)?;let before=cx.retained_progress();let recipient=cx.original_recipient_identity();let original=cx.with_retained_loan(requested,run);let after=cx.retained_progress();let report=match &original{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()};let receipt_refusal=if recipient!=cx.original_recipient_identity(){Some(refused("original owned caller return changed recipient"))}else{delta(before,after).and_then(|actual|if actual==report{Ok(())}else{Err(refused("original owned caller return differs from actual posted effects"))}).err()};Ok(OriginalCallerCloseReturn{original,receipt_refusal})}
 /// 🪙️ Quotes the owning source's fixed metadata return without deriving authority from the quote.
 pub fn retirement_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()}}
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()});}
  let witness=parent.original.as_ref().ok_or_else(||refused("original parent must remain live through strong loan return"))?;
  if parent.source_identity()?!=self.source_identity||!matches_roots(witness.get(),cx)||!self.validates_clock(cx){return Err(refused("owned source loan return requires actual parent context").into());}
  if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Blocked);}
  crate::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|grant|{
   if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1};}
   let Some(original)=self.original.as_ref()else{return RetainedWorkOutcome{result:Err(semio_framework_value_retained_clone::ValueError::invariant("original absent")),work:0}};if original.terminal_is_empty(){self.original.take();self.clock_authority=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1};}
   let Some(original)=self.original.as_mut()else{return RetainedWorkOutcome{result:Err(semio_framework_value_retained_clone::ValueError::invariant("original absent")),work:0}};let turn=original.return_borrowed_alias(witness,grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
  })
 }
}
impl Drop for OriginalCallerSourceLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original static source loan reached Drop before paid borrowed-parent return")}}
fn matches_roots(roots:&SourceRoots,cx:&StepContext<'_>)->bool{roots.authority.original().is_some_and(|ledger|Arc::ptr_eq(ledger,&cx.payload_ledger)&&ledger.operation==cx.operation()&&ledger.generation==cx.generation())&&roots.cancel.is_same_node(cx.original_cancel_token())}
fn cancel_refusal(error:CancelTokenRetirementError)->ValueError{match error{CancelTokenRetirementError::Refused(error)=>error,CancelTokenRetirementError::Blocked(blocked)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,match blocked{semio_framework_async::CancelTokenRetirementBlocked::SharedAlias=>"original cancellation node retains a shared alias",semio_framework_async::CancelTokenRetirementBlocked::WeakAlias=>"original cancellation node retains a weak alias",semio_framework_async::CancelTokenRetirementBlocked::RegisteredWaiter=>"original cancellation node retains a registered wake owner",semio_framework_async::CancelTokenRetirementBlocked::WaiterContention=>"original cancellation waiter guard is occupied"})}}
#[path="👶️child/🦀️.rs"]
mod child_source;
pub use child_source::{OriginalCallerChildSource,OriginalChildTurnReturn};








