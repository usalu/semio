//! 📸️ Captured original callers retain paid aliases and exclusively receive genuine supplier turns.
use super::*;
use semio_framework_async::{CancelTokenRetirement,CancelTokenRetirementError};
#[path="../🎟️source/🦀️.rs"]
mod owned_source;
pub use owned_source::{OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalCallerChildSource,OriginalChildTurnReturn};

pub(super) struct OriginalCallerSource<'source>{pub(super) ledger:&'source Arc<JobPayloadOperationLedger>,pub(super) cancel:&'source CancelToken,pub(super) operation:OperationId,pub(super) generation:Generation,pub(super) clock:fn()->Option<u64>,pub(super) deadline:u64}
static NEXT_CALLER_ISSUER:AtomicU64=AtomicU64::new(1);

/// 📷️ Keeps original borrowed roots live until each independently funded alias is captured or returned.
pub struct OriginalCallerCapture<'source>{source:OriginalCallerSource<'source>,ledger:ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,cancel:ManuallyDrop<Option<CancelTokenRetirement>>,issuer:u64}
impl<'source> OriginalCallerCapture<'source>{
 pub fn begin(authority:&'source JobPayloadAuthority,cancel:&'source CancelToken,cx:&StepContext<'_>)->Result<Self,ValueError>{let actual=cx.borrowed_original_caller_source()?;let ledger=authority.original().ok_or_else(||refused("original factory ledger has already closed"))?;if !Arc::ptr_eq(ledger,actual.ledger)||!std::ptr::eq(cancel,actual.cancel){return Err(refused("original factory sources do not witness the actual caller"))}let source=OriginalCallerSource{ledger,cancel,operation:actual.operation,generation:actual.generation,clock:actual.clock,deadline:actual.deadline};let issuer=NEXT_CALLER_ISSUER.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|refused("original caller issuer identity exhausted"))?;Ok(Self{source,ledger:ManuallyDrop::new(None),cancel:ManuallyDrop::new(None),issuer})}
 fn matches(&self,cx:&StepContext<'_>)->bool{cx.operation()==self.source.operation&&cx.generation()==self.source.generation&&Arc::ptr_eq(&cx.payload_ledger,self.source.ledger)&&std::ptr::eq(cx.original_cancel_token(),self.source.cancel)&&std::ptr::fn_addr_eq(cx.clock_source(),self.source.clock)&&cx.deadline_us()<=self.source.deadline}
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.matches(cx){return Err(refused("original capture requires its actual source context").into())}if self.ledger.is_some()&&self.cancel.is_some(){return Ok(true)}if cx.is_cancelled()||cx.should_yield()||!metadata_fits(cx){return Ok(false)}if self.ledger.is_none(){cx.consume_retained(metadata())?;*self.ledger=Some(Arc::clone(self.source.ledger));cx.consume_fuel(1);return Ok(false)}if let Some(alias)=cx.admit_original_cancel_alias()?{*self.cancel=Some(CancelTokenRetirement::from_token(alias));cx.consume_fuel(1);}Ok(self.cancel.is_some())}
 pub fn finish(mut self)->Result<(CapturedOriginalCaller<'source>,OriginalCallerIssuer),Self>{if self.ledger.is_none()||self.cancel.is_none(){return Err(self)}let source=OriginalCallerSource{ledger:self.source.ledger,cancel:self.source.cancel,operation:self.source.operation,generation:self.source.generation,clock:self.source.clock,deadline:self.source.deadline};let original=CapturedOriginalCaller{ledger:ManuallyDrop::new(self.ledger.take()),cancel:ManuallyDrop::new(self.cancel.take()),source,issuer:self.issuer};let issuer=OriginalCallerIssuer{identity:self.issuer,epoch:0,closing:false};Ok((original,issuer))}
 pub fn terminal_is_empty(&self)->bool{self.ledger.is_none()&&self.cancel.is_none()}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if !self.matches(cx){return Err(refused("partial capture return requires its original source").into())}close_aliases(&mut self.ledger,&mut self.cancel,cx)}
}
impl Drop for OriginalCallerCapture<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original partial caller capture reached Drop before paid alias return")}}

/// 🧳️ Owns only the actual paid source aliases; it cannot create a context, budget, root or deadline.
pub struct CapturedOriginalCaller<'source>{ledger:ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,cancel:ManuallyDrop<Option<CancelTokenRetirement>>,source:OriginalCallerSource<'source>,issuer:u64}
impl CapturedOriginalCaller<'_>{
 fn matches(&self,cx:&StepContext<'_>)->bool{self.ledger.is_some()&&self.cancel.as_ref().is_some_and(|cancel|cancel.is_original_alias_witness(cx.original_cancel_token()))&&self.matches_return(cx)}
 pub fn terminal_is_empty(&self)->bool{self.ledger.is_none()&&self.cancel.is_none()}
 pub fn close_step(&mut self,issuer:&mut OriginalCallerIssuer,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if issuer.identity!=self.issuer{return Err(refused("captured caller return requires its actual issuer").into())}if !self.terminal_is_empty()&&!self.matches_return(cx){return Err(refused("captured caller return requires its original ledger and cancellation witness").into())}issuer.closing=true;close_aliases(&mut self.ledger,&mut self.cancel,cx)}
 fn matches_return(&self,cx:&StepContext<'_>)->bool{cx.operation()==self.source.operation&&cx.generation()==self.source.generation&&std::ptr::fn_addr_eq(cx.clock_source(),self.source.clock)&&cx.deadline_us()<=self.source.deadline&&Arc::ptr_eq(self.source.ledger,&cx.payload_ledger)&&std::ptr::eq(self.source.cancel,cx.original_cancel_token())}
}
impl Drop for CapturedOriginalCaller<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"captured original caller reached Drop before paid alias return")}}

/// 🎟️ Exclusively issues an actual supplied context; epochs never construct or renew authority.
pub struct OriginalCallerIssuer{identity:u64,epoch:u64,closing:bool}
impl OriginalCallerIssuer{
 pub fn issue<'turn,'context>(&'turn mut self,original:&CapturedOriginalCaller,cx:&'turn mut StepContext<'context>)->Result<OriginalCallerTurn<'turn,'context>,ValueError>{if self.closing||self.identity!=original.issuer||!original.matches(cx){return Err(refused("original caller supplier refused foreign or closing context"))}let epoch=self.epoch.checked_add(1).ok_or_else(||refused("original caller turn epoch exhausted"))?;self.epoch=epoch;let before=cx.retained_progress();let recipient_identity=cx.retained_progress as*mut RetainedCloneProgress as usize;Ok(OriginalCallerTurn{issuer:self,context:cx,before,recipient_identity,epoch,returned:false})}
}

/// 🤝️ Borrows one live supplier context and the same exclusive receipt recipient through the callback.
pub struct OriginalCallerTurn<'turn,'context>{issuer:&'turn mut OriginalCallerIssuer,context:&'turn mut StepContext<'context>,before:RetainedCloneProgress,recipient_identity:usize,epoch:u64,returned:bool}
impl<'turn,'context> OriginalCallerTurn<'turn,'context>{
 pub fn epoch(&self)->u64{self.epoch}
 pub fn context(&mut self)->&mut StepContext<'context>{self.context}
 pub fn close(mut self,requested:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'context>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->OriginalCallerCloseReturn{let original=self.context.with_retained_loan(requested,run);let after=self.context.retained_progress();let report=match &original{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()};let receipt_refusal=if self.recipient_identity!=self.context.retained_progress as*mut RetainedCloneProgress as usize{Some(refused("original caller return changed recipient"))}else{delta(self.before,after).and_then(|actual|if actual==report{Ok(())}else{Err(refused("original caller return differs from its already received effects"))}).err()};self.issuer.epoch=self.epoch;self.returned=true;OriginalCallerCloseReturn{original,receipt_refusal}}
}
impl Drop for OriginalCallerTurn<'_,'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.returned,"original caller turn reached Drop before exact receipt receiving")}}

/// 🪢️ Preserves the complete child return and any independent receiving refusal in one owned carrier.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct OriginalCallerCloseReturn{original:Result<InteractiveJobCloseStep,OriginalTurnFailure>,receipt_refusal:Option<ValueError>}
impl OriginalCallerCloseReturn{pub fn admitted(&self)->Option<&Result<InteractiveJobCloseStep,OriginalTurnFailure>>{self.receipt_refusal.is_none().then_some(&self.original)}pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}pub fn into_original(self)->Result<Result<InteractiveJobCloseStep,OriginalTurnFailure>,Self>{if self.receipt_refusal.is_some(){Err(self)}else{Ok(self.original)}}}
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn metadata_fits(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();grant.maximum_items>0&&grant.maximum_depth>0}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{let subtract=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||refused("original caller cumulative receipt moved backwards"));Ok(RetainedCloneProgress{copied_items:subtract(after.copied_items,before.copied_items)?,copied_bytes:subtract(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:subtract(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:subtract(after.released_bytes,before.released_bytes)?})}
fn close_aliases(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,cancel:&mut Option<CancelTokenRetirement>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if ledger.is_none()&&cancel.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if cx.should_yield()||!metadata_fits(cx){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}if let Some(original)=cancel.as_mut(){let step=original.return_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(|error|match error{CancelTokenRetirementError::Refused(error)=>OriginalTurnFailure::debit_original(error,cx),CancelTokenRetirementError::Blocked(_)=>refused("original caller cancellation alias is blocked").into()})?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);if original.terminal_is_empty(){cancel.take();}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}if !ledger.as_ref().is_some_and(|original|Arc::ptr_eq(original,&cx.payload_ledger)){return Err(refused("original caller ledger return requires the same live source").into())}cx.consume_retained(metadata())?;drop(ledger.take());cx.consume_fuel(1);Ok(InteractiveJobCloseStep::Pending{progress:metadata()})}

