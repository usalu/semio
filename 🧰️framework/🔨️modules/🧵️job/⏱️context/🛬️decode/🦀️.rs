//! 🛬️ A paid original source lends actual native decode imports and receives the concrete producer's effects.
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext};
use semio_framework_value::{ErasedSnapshotRetirement,NativeDecodeControl,NativeDecodeRetirementRecipient,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::native_decoding::{NativeDecodeAllocation,NativeForwardedDecodeControl};
use semio_framework_value::native_decoding::allocation::NativeForwardedDecodeReceipt;
use std::{cell::UnsafeCell,mem::ManuallyDrop,sync::atomic::{AtomicBool,Ordering}};

/// 📋️ The original caller authors a fixed cumulative ownership ceiling before decoding begins.
#[derive(Clone,Copy,Debug)]
pub struct OriginalNativeDecodePolicy{pub maximum_owned_bytes:usize}
/// 🧾️ The actual bounded producer returns its unchanged supplied grant, physical receipt and raw work.
pub struct OriginalNativeDecodeIssued<T>{pub value:Result<T,ValueError>,pub grant:RetainedCloneGrant,pub progress:RetainedCloneProgress,pub work:u64}
/// 🧳️ A receiving refusal stays beside the original outcome without destroying either source.
pub struct OriginalNativeDecodeOutcome<T>{pub value:Result<T,ValueError>,pub progress:RetainedCloneProgress,pub work:u64,pub receiving_refusal:Option<OriginalTurnFailure>}

struct ActualTurn<'a,'source>{actual:UnsafeCell<&'a mut StepContext<'source>>,reserved:UnsafeCell<usize>,borrowed:AtomicBool}
/// 🔒️ Every mutable caller access is excluded by the atomic gate; its declared context supports transfer.
unsafe impl<'a,'source> Sync for ActualTurn<'a,'source> where StepContext<'source>:Send{}
struct ActualGuard<'a>(&'a AtomicBool);
impl Drop for ActualGuard<'_>{fn drop(&mut self){self.0.store(false,Ordering::Release)}}
impl<'a,'source> ActualTurn<'a,'source>{
 fn new(actual:&'a mut StepContext<'source>)->Self{Self{actual:UnsafeCell::new(actual),reserved:UnsafeCell::new(0),borrowed:AtomicBool::new(false)}}
 fn with<R>(&self,run:impl FnOnce(&mut StepContext<'source>,&mut usize)->R)->Result<R,ValueError>{self.borrowed.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).map_err(|_|refusal("original native decoder caller is already borrowed"))?;let guard=ActualGuard(&self.borrowed);let result=unsafe{run(&mut **self.actual.get(),&mut *self.reserved.get())};drop(guard);Ok(result)}
}
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn metadata(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

/// 🎟️ Owns the genuine source loan, original retirement recipient and consuming forwarded decoder receipt.
pub struct OriginalNativeDecodeSource{policy:OriginalNativeDecodePolicy,source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,original_recipient:ManuallyDrop<Option<NativeDecodeRetirementRecipient>>,receipt:Option<NativeForwardedDecodeReceipt>,closing:bool,returning:bool,closed:bool}
impl OriginalNativeDecodeSource{
 /// 🪹️ Keeps only an authored declaration; no source, recipient, controller or allocation is minted here.
 pub const fn empty(policy:OriginalNativeDecodePolicy)->Self{Self{policy,source:ManuallyDrop::new(None),original_recipient:ManuallyDrop::new(None),receipt:None,closing:false,returning:false,closed:false}}
 /// 🪪️ Admits the actual source before recipient birth and before any native decode import can be lent.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing||self.closed{return Err(refusal("closing original native decode source cannot admit work").into())}
  if let Some(source)=self.source.as_ref(){source.validate_context(cx)?}else{*self.source=cx.original_source_parent()?.admit_loan(cx)?;return Ok(false)}
  if self.original_recipient.is_none(){if !metadata(cx)?{return Ok(false)}*self.original_recipient=Some(NativeDecodeRetirementRecipient::new());return Ok(false)}
  if self.receipt.is_none(){if !metadata(cx)?{return Ok(false)}let _=self.with_control(cx,false,|_|())?;return Ok(false)}
  Ok(true)
 }
 fn with_control<T>(&mut self,cx:&mut StepContext<'_>,closing:bool,run:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->Result<T,OriginalTurnFailure>{
  self.source.as_ref().ok_or_else(||refusal("original native decoder has no admitted source"))?.validate_context(cx)?;
  let borrowed=ActualTurn::new(&mut *cx);
  let mut progress=|_|borrowed.with(|actual,_|!actual.should_yield()&&(closing||!actual.is_cancelled())).unwrap_or(false);
  let mut allocate=|quote:NativeDecodeAllocation|borrowed.with(|actual,reserved|{if actual.should_yield()||(!closing&&actual.is_cancelled()){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode caller yielded"))}let next=reserved.checked_add(quote.bytes).filter(|bytes|*bytes<=actual.retained_grant().maximum_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native reservation exceeds the actual turn capacity"))?;*reserved=next;Ok(())}).and_then(|answer|answer);
  let original_recipient=self.original_recipient.as_mut().ok_or_else(||refusal("original native decoder has no admitted recipient"))?;
  let mut control=if let Some(original)=self.receipt.take(){match NativeForwardedDecodeControl::rebind(original,&mut progress,&mut allocate,original_recipient){Ok(control)=>control,Err((error,original))=>{self.receipt=Some(original);return Err(error.into())}}}else{let mut control=NativeDecodeControl::new_forwarded(self.policy.maximum_owned_bytes,&mut progress,&mut allocate);control.install_retirement_recipient(original_recipient)?;control};
  let answer=control.scoped_stage(|control|Ok::<_,std::convert::Infallible>(run(control))).unwrap();
  self.receipt=Some(control.detach().unwrap_or_else(|_|panic!("original native decode supplier violated the canonical consuming receipt")));
  Ok(answer)
 }
 /// 🧮️ Reservations never debit birth; the concrete bounded producer's actual receipt returns to this exact caller.
 pub fn receive_step<T>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->OriginalNativeDecodeIssued<T>)->Result<Option<OriginalNativeDecodeOutcome<T>>,OriginalTurnFailure>{
  if !self.admit_step(cx)?||cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let fuel=cx.fuel_remaining();let issued=self.with_control(cx,false,|control|run(control,grant))?;
  Ok(Some(Self::receive_issued(cx,grant,fuel,issued)))
 }
 /// ♻️ A retained producer borrows its authentic decoder during paid close even after original cancellation.
 pub fn close_producer_step<T>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->OriginalNativeDecodeIssued<T>)->Result<Option<OriginalNativeDecodeOutcome<T>>,OriginalTurnFailure>{
  if self.closing{return Err(refusal("original producer close cannot resume after native source return began").into())}if self.source.is_none()||cx.should_yield(){return Ok(None)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let fuel=cx.fuel_remaining();let issued=self.with_control(cx,true,|control|run(control,grant))?;Ok(Some(Self::receive_issued(cx,grant,fuel,issued)))
 }
 fn receive_issued<T>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,issued:OriginalNativeDecodeIssued<T>)->OriginalNativeDecodeOutcome<T>{
  let mut receiving_refusal=None;
  if issued.grant!=grant||!issued.progress.fits(grant){receiving_refusal=Some(OriginalTurnFailure::from(refusal("original native producer changed its supplied grant or exceeded its physical receipt").with_retained_progress(issued.progress)))}
  if issued.work>fuel&&receiving_refusal.is_none(){receiving_refusal=Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original native producer exceeded the supplied caller work").with_retained_progress(issued.progress).into())}
  cx.consume_fuel(issued.work);
  if let Err(error)=cx.consume_retained(issued.progress){if let Some(failure)=receiving_refusal.as_mut(){failure.receipt_refusal=Some(error)}else{receiving_refusal=Some(error.into())}}
  OriginalNativeDecodeOutcome{value:issued.value,progress:issued.progress,work:issued.work,receiving_refusal}
 }
 /// 📏️ Quotes the retained recipient or original source metadata return without constructing an authority.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closed{return Ok(Default::default())}if let Some(recipient)=self.original_recipient.as_ref().filter(|recipient|!recipient.terminal_is_empty()){return Ok(RetirementDemand{copy_bytes:recipient.next_copy_byte_demand()?,capacity_bytes:recipient.next_capacity_byte_demand(body)?,release_bytes:recipient.next_release_byte_demand()?,depth:recipient.next_depth_demand()?})}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// ♻️ Returns recipient owners and the consuming receipt before the genuine source loan's last close phase.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  self.closing=true;
  if cx.should_yield(){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.original_recipient.as_ref().is_some_and(|recipient|!recipient.terminal_is_empty()){
   let grant=RetainedCloneGrant{maximum_items:cx.retained_grant().maximum_items.min(1),..cx.retained_grant()};let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
   if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let raw=self.with_control(cx,true,|control|control.close_retirement_recipient(grant))?;
   return semio_framework_job::close_original_retained_step(cx,demand,|_|raw).map(|step|match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>RetainedCloneStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>RetainedCloneStep::Complete(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>RetainedCloneStep::Progress(Default::default())})
  }
  if self.receipt.is_some(){if !metadata(cx)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.receipt.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if self.original_recipient.is_some(){if !metadata(cx)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.original_recipient.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(source)=self.source.as_mut(){self.returning=true;let parent=cx.original_source_parent()?;let before=cx.retained_progress();source.close_step(parent,cx)?;if source.terminal_is_empty(){self.source.take();}let after=cx.retained_progress();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}))}
  if !metadata(cx)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 /// 📊️ Borrows the retained consuming decoder ledger without creating another admission.
 pub fn owned_bytes(&self)->Option<usize>{self.receipt.as_ref().map(NativeForwardedDecodeReceipt::owned_bytes)}
 /// 🔒️ No original recipient, decoder receipt or source alias remains after the final paid turn.
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.source.is_none()&&self.original_recipient.is_none()&&self.receipt.is_none()}
}
impl Drop for OriginalNativeDecodeSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native decode source requires explicit terminal source return")}}
