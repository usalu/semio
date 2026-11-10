//! 🛂️ Decoder reservations retain original receiving allocation and retirement authority.
//! 📏️ Cumulative admission records requested bytes; physical posts independently report actual capacity.
//! A performed allocator failure posts zero; cancellation records an effect that never began.
use super::{NativeDecodeControl,NativeDecodeContinuation,NativeDecodeProgress,NativeDecodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};
pub(super) fn commit_admission(ledger:&mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,request:NativeAllocationRequest)->Result<(),ValueError>{ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original decode ledger absent"))?.commit(request)}
#[path="📦️slot/🦀️.rs"]
mod slot;
pub use crate::{NativeAllocationEvent,NativeAllocationRequest};

pub use crate::NativeSendAllocationLoan;
pub use semio_framework_value_native_outcome::NativeDecodeAllocationOutcome;
pub use semio_framework_value_native_outcome::{NativeDecodeAllocationProgress,NativeDecodeAllocationPhase};
pub(super) use semio_framework_value_native_outcome::{NativeDecodeAllocationBinding as Binding,NativeDecodeAllocationAdmission};
fn allocation_refused(primary:ValueError)->NativeDecodeAllocationOutcome{NativeDecodeAllocationOutcome{request:None,admitted:false,primary:Some(primary),quote_refusal:None,extent_refusal:None,settlement_refusal:None,actual:None}}
impl NativeDecodeControl<'_>{
 /// 📦️ Quotes and settles a caller-retained allocation effect.
 pub fn allocate_effect(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeDecodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<(),ValueError>)->NativeDecodeAllocationOutcome{self.allocate_effect_gated(bytes,actual,progress,&mut |actual|operation(actual).map(|()|true)).expect("ungated allocation always returns its outcome")}
 /// 🪢️ One-shot allocating effect: quotes the demand's capacity through this control's own (possibly forwarded) allocation port, runs the closure inside one original Work action and settles the physical marker once; `None` means the work quote was not admitted, the allocation quote is then cancelled.
pub fn with_allocated_effect<R>(&mut self,demand:crate::RetirementDemand,maximum_work:u64,run:impl FnOnce(&mut Self,&mut crate::RetainedAllocationEffect<'_>)->R)->Result<Option<R>,ValueError>{
  let refusal=|message:&'static str|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
  let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  if demand.capacity_bytes==0{return self.with_effect(grant,maximum_work,run)}
  let mut progress=None;
  match self.prepare_allocation_effect(demand.capacity_bytes,&None,&mut progress,true){NativeDecodeAllocationAdmission::Admitted=>{},NativeDecodeAllocationAdmission::Refused(outcome)=>return Err(outcome.primary.or(outcome.extent_refusal).or_else(||outcome.quote_refusal.map(|port|ValueError::new(ValueRefusalKind::AllocationFailed,format!("{port:?}")))).or_else(||outcome.settlement_refusal.map(|port|ValueError::new(ValueRefusalKind::AllocationFailed,format!("{port:?}")))).unwrap_or_else(||refusal("decoder allocation quote was refused"))),NativeDecodeAllocationAdmission::Pending=>return Err(refusal("decoder allocation quote stayed pending"))}
  let(widened,widened_work)=semio_framework_value_control_contract::invocation_extent(grant,maximum_work)?;
  let opened=semio_framework_value_control_contract::OriginalEffectRun::begin(&mut self.work,widened,widened_work);
  let mut open=match opened{Ok(Some(open))=>open,Ok(None)=>{if let Some(state)=progress.as_mut(){let _=self.settle_allocation_progress(state,None)?;}return Ok(None)},Err(error)=>{if let Some(state)=progress.as_mut(){let _=state.record_primary(error.clone());let _=self.settle_allocation_progress(state,None);}return Err(error)}};
  let started=progress.as_mut().ok_or_else(||refusal("decoder allocation quote is absent"))?.start_operation();
  let output={let mut effect=open.effect()?;semio_framework_value_control_contract::invoke_original(&mut effect,Some((grant,maximum_work)),|effect|run(self,effect))};
  let actual=open.actual();let mut spent=0;let finished=open.finish(&mut self.work,&mut spent);
  let state=progress.as_mut().ok_or_else(||refusal("decoder allocation quote is absent"))?;
  started?;state.record_actual(actual)?;
  if !self.settle_allocation_progress(state,actual)?{return Err(refusal("decoder allocation settlement stayed pending"))}
  finished?;Ok(Some(output?))
 }
 /// 🧭️ Keeps an original quote pending when a child receipt cannot yet be reconciled.
 /// 🪑️ Retains the original quote before lending the decoder and its actual effect to a child.
 pub(super) fn prepare_allocation_effect(&mut self,bytes:usize,actual:&Option<usize>,progress:&mut Option<NativeDecodeAllocationProgress>,cancellable:bool)->NativeDecodeAllocationAdmission{
  if actual.is_some()||progress.is_some(){return NativeDecodeAllocationAdmission::Refused(allocation_refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation requires empty retained markers")))}let Some(next)=self.owned_bytes().checked_add(bytes).filter(|next|*next<=self.maximum_bytes)else{return NativeDecodeAllocationAdmission::Refused(allocation_refused(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoding allocation exceeds original ceiling")))};if cancellable{if let Err(error)=self.checkpoint(){return NativeDecodeAllocationAdmission::Refused(allocation_refused(error))}}else{let progress=NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes()};let _=(self.callback)(progress);}let request=NativeAllocationRequest{bytes,owned_bytes:self.owned_bytes(),next_owned_bytes:next,maximum_bytes:self.maximum_bytes};NativeDecodeAllocationProgress::begin(request,&mut self.allocation,progress)
 }
 pub(super) fn allocate_effect_gated(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeDecodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<bool,ValueError>)->Option<NativeDecodeAllocationOutcome>{
  match self.prepare_allocation_effect(bytes,actual,progress,true){NativeDecodeAllocationAdmission::Admitted=>{},NativeDecodeAllocationAdmission::Refused(outcome)=>return Some(outcome),NativeDecodeAllocationAdmission::Pending=>return None}progress.as_mut().unwrap().start_operation().expect("original allocation quote accepted");let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(actual)));let state=progress.as_mut().unwrap();match result{Ok(Ok(true))=>state.record_actual(*actual).expect("original physical marker retained"),Ok(Ok(false))=>return None,Ok(Err(error))=>{state.record_actual(*actual).expect("original physical marker retained");if let Err(error)=state.record_primary(error){return Some(allocation_refused(error))}},Err(payload)=>{state.record_actual(*actual).expect("original physical marker retained");std::panic::resume_unwind(payload)}}match self.settle_allocation_progress(state,*actual){Ok(true)=>{},Ok(false)=>return None,Err(error)=>{if let Err(error)=state.record_primary(error){return Some(allocation_refused(error))}return None}}Some(progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome"))
 }
 /// 🪪️ Reconciles a pending callback using its original recipient's actual result.
 pub fn recover_allocation_callback(&mut self,state:&mut NativeDecodeAllocationProgress)->Result<bool,ValueError>{state.recover(&mut self.allocation)}
 /// 🪪️ Visits a typed retained frame alongside the original allocation authority.
 pub fn visit_allocation_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeDecodeAllocationRecovery<'_,'_>))->Result<bool,ValueError>{let Some(recipient)=self.retirement.as_deref_mut()else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native allocation recovery requires its original recipient"))};if recipient.funding.identity!=recipient_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation recovery recipient differs from original"))}let mut recovery=NativeDecodeAllocationRecovery{allocation:&mut self.allocation,ledger:&mut self.ledger};Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
 /// 🧾️ Settles a pending quote only through its original port at a known callback boundary.
 pub fn settle_allocation_progress(&mut self,state:&mut NativeDecodeAllocationProgress,actual:Option<usize>)->Result<bool,ValueError>{state.settle(&mut self.ledger,&mut self.allocation,actual)}
}


/// 🧭️ Reborrows the original allocation ports and ledger alongside the retained frame.
pub struct NativeDecodeAllocationRecovery<'a,'control>{allocation:&'a mut Binding<'control>,ledger:&'a mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>}
impl NativeDecodeAllocationRecovery<'_,'_>{
 pub fn recover_allocation_callback(&mut self,state:&mut NativeDecodeAllocationProgress)->Result<bool,ValueError>{state.recover(&mut *self.allocation)}
 pub fn settle_allocation_progress(&mut self,state:&mut NativeDecodeAllocationProgress,actual:Option<usize>)->Result<bool,ValueError>{state.settle(self.ledger,&mut *self.allocation,actual)}
}
/// 🧵️ Retains original ports, retirement slot and cumulative receipt across callback lifetimes.
pub struct NativeForwardedDecodeContinuation<'a>{receipt:Option<NativeDecodeContinuation>,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),retirement:Option<&'a mut NativeDecodeRetirementRecipient>}
/// 🧮️ Exposes canonical parsing under original receiving allocation admission.
pub struct NativeForwardedDecodeControl<'a>{inner:NativeDecodeControl<'a>}

/// 🧳️ The original caller retains the cumulative receipt across a forwarded callback or unwind.
pub struct NativeForwardedDecodeFrame<'owner,'control>{control:NativeForwardedDecodeControl<'control>,receipt:&'owner mut Option<NativeForwardedDecodeReceipt>,recipient:u64,allocation_identity:crate::NativeAllocationIdentity}
impl<'owner,'control> NativeForwardedDecodeFrame<'owner,'control>{
 /// ▶️ Starts or rebinds a caller-owned receipt under its original live allocation and retirement ports.
 pub fn begin(receipt:&'owner mut Option<NativeForwardedDecodeReceipt>,maximum:usize,callback:&'control mut dyn FnMut(NativeDecodeProgress)->bool,allocation:&'control mut (dyn NativeSendAllocationLoan+'control),recipient:&'control mut NativeDecodeRetirementRecipient)->Result<Self,ValueError>{let recipient_identity=recipient.funding.identity;let allocation_identity=allocation.identity();let control=if let Some(saved)=receipt.take(){match NativeForwardedDecodeControl::rebind(saved,callback,allocation,recipient){Ok(control)=>control,Err((error,saved))=>{*receipt=Some(saved);return Err(error)}}}else{let mut control=NativeDecodeControl::new_forwarded(maximum,callback,allocation);control.inner.retirement=Some(recipient);control};Ok(Self{control,receipt,recipient:recipient_identity,allocation_identity})}
 /// 🫴️ Lends only this original forwarded decoder for the current callback.
 pub fn control(&mut self)->&mut NativeForwardedDecodeControl<'control>{&mut self.control}
 /// 🧳️ Preserves the original owned result beside context refusal; frame drop restores the same receipt on return or unwind.
 pub fn scope<T,E>(&mut self,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,super::NativeCallFailure<T,E>>{
  let result=self.control.inner.scoped_stage(operation);
  let context=self.control.inner.validate_continuation().err();
  match(result,context){
   (Ok(output),None)=>Ok(output),
   (Ok(output),Some(error))=>Err(super::NativeCallFailure{output:Some(output),operation:None,context:Some(error),receipt_refusal:None}),
   (Err(error),context)=>Err(super::NativeCallFailure{output:None,operation:Some(error),context,receipt_refusal:None}),
  }
 }

}
impl Drop for NativeForwardedDecodeFrame<'_,'_>{fn drop(&mut self){let inner=&mut self.control.inner;let receipt=NativeDecodeContinuation{work:inner.work.state.take(),ledger:inner.ledger.take(),receiving:inner.receiving,maximum_bytes:inner.maximum_bytes,completed:inner.completed,total:inner.total,stage:inner.stage};*self.receipt=Some(NativeForwardedDecodeReceipt{receipt,recipient:self.recipient,allocation_identity:self.allocation_identity});}}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedDecodeReceipt{receipt:NativeDecodeContinuation,recipient:u64,allocation_identity:crate::NativeAllocationIdentity}
impl crate::retirement::RetireOwned for NativeForwardedDecodeReceipt{
 type RetirementCursor=<NativeDecodeContinuation as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.receipt)}
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed(self.receipt)}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_birth_bytes(&self.receipt)}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeForwardedDecodeReceipt{
 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.ledger.as_ref().expect("original decode ledger present").owned_bytes()}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
 /// ⏳️ Reports an unresolved original work quote or admitted reservation.
 pub fn has_pending_work(&self)->bool{self.receipt.work.has_live_actions()}
}
impl<'a> NativeDecodeControl<'a>{
 /// 🏭️ Binds every reservation to the original caller allocation port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut (dyn NativeSendAllocationLoan+'a))->NativeForwardedDecodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=Binding::Forwarded(allocate);NativeForwardedDecodeControl{inner}}
}
impl<'a> NativeForwardedDecodeControl<'a>{
 /// ⏹️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(mut self)->Result<NativeForwardedDecodeReceipt,(ValueError,Self)>{
  if self.inner.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction loan cannot detach from its original receiving hop"),self));}
  let Some(recipient)=self.inner.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};
  let recipient_identity=recipient.funding.identity;let allocation_identity=match &self.inner.allocation{Binding::Forwarded(port)=>port.identity(),Binding::Local=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded allocation loan is absent"),self))};let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};Ok(NativeForwardedDecodeReceipt{receipt,recipient:recipient_identity,allocation_identity})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedDecodeReceipt,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),recipient:&'a mut NativeDecodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedDecodeReceipt)>{
  if receipt.recipient!=recipient.funding.identity||receipt.allocation_identity!=allocate.identity()||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let NativeForwardedDecodeReceipt{receipt:original,recipient:recipient_identity,allocation_identity}=receipt;let mut inner=match NativeDecodeControl::resume(original,callback){Ok(inner)=>inner,Err((error,original))=>return Err((error,NativeForwardedDecodeReceipt{receipt:original,recipient:recipient_identity,allocation_identity}))};inner.allocation=Binding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves original allocation and retirement authority together with the cumulative receipt.
 pub fn pause(mut self)->Result<NativeForwardedDecodeContinuation<'a>,(ValueError,Self)>{if self.inner.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction loan cannot pause outside its original receiving hop"),self))}if !matches!(self.inner.allocation,Binding::Forwarded(_)){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded decoder lacks original allocation port"),self))}let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};let Binding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,Binding::Local)else{unreachable!()};Ok(NativeForwardedDecodeContinuation{receipt:Some(receipt),allocate,retirement:self.inner.retirement.take()})}
 /// ⏳️ Rebinds only progress while retaining original allocation and retirement authority.
 pub fn resume(mut receipt:NativeForwardedDecodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,(ValueError,NativeForwardedDecodeContinuation<'a>)>{let Some(original)=receipt.receipt.take()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving"),receipt))};let mut inner=match NativeDecodeControl::resume(original,callback){Ok(inner)=>inner,Err((error,original))=>{receipt.receipt=Some(original);return Err((error,receipt))}};inner.allocation=Binding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
struct ForwardedReceiptScope<'slot,'control>{control:NativeDecodeControl<'control>,receipt:&'slot mut Option<NativeDecodeContinuation>}
impl Drop for ForwardedReceiptScope<'_,'_>{fn drop(&mut self){let inner=&mut self.control;*self.receipt=Some(NativeDecodeContinuation{work:inner.work.state.take(),ledger:inner.ledger.take(),receiving:inner.receiving,maximum_bytes:inner.maximum_bytes,completed:inner.completed,total:inner.total,stage:inner.stage});}}
impl<'a> NativeForwardedDecodeContinuation<'a>{
 /// 🪢️ Binds the original caller port directly without creating replacement progress authority.
 pub fn new(maximum_bytes:usize,allocate:&'a mut (dyn NativeSendAllocationLoan+'a))->Self{Self{receipt:Some(NativeDecodeContinuation{work:super::work::WorkState::fresh(),ledger:Some(semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger::new(0)),receiving:false,maximum_bytes,completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🫴️ Installs the original caller return slot before retained decoder ownership exists.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded decoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 🔁️ Reborrows original ports and retirement slot for one synchronous receiving hop.
 pub fn decode<T,E>(&mut self,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,super::NativeCallFailure<T,E>>{
  let refused=|error|super::NativeCallFailure{output:None,operation:None,context:Some(error),receipt_refusal:None};
  let Some(receipt)=self.receipt.take()else{return Err(refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving")))};
  let allocation_loan=&mut *self.allocate;
  let mut control=match NativeDecodeControl::resume(receipt,callback){Ok(control)=>control,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(refused(error))}};
  control.allocation=Binding::Forwarded(allocation_loan);control.retirement=self.retirement.as_deref_mut();
  let mut scope=ForwardedReceiptScope{control,receipt:&mut self.receipt};
  let result=scope.control.scoped_stage(operation);let context=scope.control.validate_continuation().err();
  match(result,context){(Ok(output),None)=>Ok(output),(Ok(output),Some(error))=>Err(super::NativeCallFailure{output:Some(output),operation:None,context:Some(error),receipt_refusal:None}),(Err(error),context)=>Err(super::NativeCallFailure{output:None,operation:Some(error),context,receipt_refusal:None})}
 }
 /// 📬️ Returns the original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").maximum_bytes}
 /// 📩️ Returns cumulative original receiving admission.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").ledger.as_ref().expect("original decode ledger present").owned_bytes()}
}
impl<'a> std::ops::Deref for NativeForwardedDecodeControl<'a>{type Target=NativeDecodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedDecodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}

#[cfg(test)]
#[path="🧪️tests/🪢️unwind.rs"]
mod unwind_tests;

/// 🚦️ Records the original budget admission callback boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeDecodeAdmissionPhase{Started,Accepted,Refused}
/// 🧭️ Keeps the original budget request and callback verdict outside the operation stack.
pub struct NativeDecodeAdmissionProgress{request:NativeAllocationRequest,identity:Option<crate::NativeAllocationIdentity>,phase:NativeDecodeAdmissionPhase,refusal:Option<crate::NativeAllocationRefusal>,transaction:Option<crate::NativeAllocationTransaction>}
impl NativeDecodeAdmissionProgress{
 /// 📏️ Borrows the exact original requested admission.
 pub fn request(&self)->NativeAllocationRequest{self.request}
 /// 🚦️ Observes the original callback boundary.
 pub fn phase(&self)->NativeDecodeAdmissionPhase{self.phase}
}
impl Drop for NativeDecodeAdmissionProgress{fn drop(&mut self){assert!(std::thread::panicking()||self.phase!=NativeDecodeAdmissionPhase::Started&&self.transaction.is_none(),"original admission callback remains unresolved")}}
impl crate::retirement::RetireOwned for NativeDecodeAdmissionProgress{
 type RetirementCursor=semio_framework_value_native_encoding::native_receiving::NativeRetirementGate<Self,<Option<crate::NativeAllocationRefusal> as crate::retirement::RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{self.phase!=NativeDecodeAdmissionPhase::Started&&self.transaction.is_none()&&crate::retirement::RetireOwned::retirement_ready(&self.refusal)}
 fn retirement_seed(mut self)->Self::RetirementCursor{if !crate::retirement::RetireOwned::retirement_ready(&self){return semio_framework_value_native_encoding::native_receiving::NativeRetirementGate::Pending(self)}let causes=self.refusal.take();semio_framework_value_native_encoding::native_receiving::NativeRetirementGate::Ready(crate::retirement::RetireOwned::retirement_seed(causes))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeDecodeControl<'_>{
 /// 🎟️ Starts a budget-only admission using a caller-owned interruption slot.
 pub fn begin_admission(&mut self,bytes:usize,slot:&mut Option<NativeDecodeAdmissionProgress>)->Result<(),ValueError>{
  if slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native admission requires an empty progress slot"))}
  let next=self.owned_bytes().checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native admission exceeds original ceiling"))?;
  self.checkpoint()?;let request=NativeAllocationRequest{bytes,owned_bytes:self.owned_bytes(),next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity()),Binding::Local=>None};
  *slot=Some(NativeDecodeAdmissionProgress{request,identity,phase:if identity.is_some(){NativeDecodeAdmissionPhase::Started}else{NativeDecodeAdmissionPhase::Accepted},refusal:None,transaction:None});
  if let Binding::Forwarded(port)=&mut self.allocation{let state=slot.as_mut().expect("admission progress present");if let Err(error)=port.begin(request,false,&mut state.transaction){state.phase=NativeDecodeAdmissionPhase::Refused;drop(slot.take());return Err(error)}let result=state.transaction.as_mut().expect("original admission transaction").take_verdict().expect("original admission verdict");match result{Ok(())=>{commit_admission(&mut self.ledger,request)?;state.phase=NativeDecodeAdmissionPhase::Accepted},Err(error)=>{state.phase=NativeDecodeAdmissionPhase::Refused;state.refusal=Some(error)}}port.finish(&mut state.transaction)?}else{commit_admission(&mut self.ledger,request)?}Ok(())
 }
 /// 🪪️ Records the original callback verdict after an interrupted admission.
 pub fn recover_admission(&mut self,state:&mut NativeDecodeAdmissionProgress)->Result<bool,ValueError>{let owned=self.owned_bytes();let port=match &mut self.allocation{Binding::Forwarded(port)if Some(port.identity())==state.identity=>port,_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission authority changed"))};if state.phase!=NativeDecodeAdmissionPhase::Started||owned!=state.request.owned_bytes||self.maximum_bytes!=state.request.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission is not pending"))}let transaction=state.transaction.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission transaction is absent"))?;if !port.recover(transaction)?{return Ok(false)}let verdict=transaction.take_verdict().expect("recovered admission verdict");match verdict{Ok(())=>{commit_admission(&mut self.ledger,state.request)?;state.phase=NativeDecodeAdmissionPhase::Accepted},Err(error)=>{state.phase=NativeDecodeAdmissionPhase::Refused;state.refusal=Some(error)}}port.finish(&mut state.transaction)?;Ok(true)}
 /// 🧾️ Commits a settled budget admission without a physical allocation receipt.
 pub fn finish_admission(&mut self,slot:&mut Option<NativeDecodeAdmissionProgress>)->Result<(),crate::NativeAllocationFailure>{if let Some(state)=slot.as_mut(){if state.transaction.is_some()&&state.phase!=NativeDecodeAdmissionPhase::Started{match &mut self.allocation{Binding::Forwarded(port)if Some(port.identity())==state.identity=>port.finish(&mut state.transaction).map_err(crate::NativeAllocationFailure::from)?,_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission authority changed").into())}}}let Some(state)=slot.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native admission progress is absent").into())};let identity=match &self.allocation{Binding::Forwarded(port)=>Some(port.identity()),Binding::Local=>None};if state.transaction.is_some()||state.phase==NativeDecodeAdmissionPhase::Started||state.identity!=identity||self.owned_bytes()!=if state.phase==NativeDecodeAdmissionPhase::Accepted{state.request.next_owned_bytes}else{state.request.owned_bytes}||self.maximum_bytes!=state.request.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"admission requires its original ledger and settled callback").into())}let mut state=slot.take().expect("admission progress present");if state.phase==NativeDecodeAdmissionPhase::Accepted{return Ok(())}let mut failure=crate::NativeAllocationFailure::default();failure.forwarded_admission=state.refusal.take();failure.progress=crate::NativeAllocationProgress{prior_admitted_bytes:Some(state.request.owned_bytes),requested_bytes:Some(state.request.bytes),maximum_bytes:Some(state.request.maximum_bytes),admitted_bytes:state.request.owned_bytes,actual_capacity_bytes:None};Err(failure)}
}

