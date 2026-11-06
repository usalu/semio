//! 🛂️ Decoder reservations retain original receiving allocation and retirement authority.
//! 📏️ Cumulative admission records requested bytes; physical posts independently report actual capacity.
//! A performed allocator failure posts zero; cancellation records an effect that never began.
use super::{NativeDecodeControl,NativeDecodeContinuation,NativeDecodeProgress,NativeDecodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};
fn commit_admission(ledger:&mut usize,request:NativeDecodeAllocation,actual:usize)->Result<(),ValueError>{let next=request.owned_bytes.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"actual native allocation extent overflow"))?;*ledger=next;Ok(())}
#[path="📦️slot/🦀️.rs"]
mod slot;
/// 📦️ Describes one reservation before storage or cumulative accounting changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeAllocation{pub bytes:usize,pub owned_bytes:usize,pub next_owned_bytes:usize,pub maximum_bytes:usize}
/// 🧾️ Quotes original authority, then posts an actual effect or cancels before effect.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeDecodeAllocationEvent{Quote(NativeDecodeAllocation),Post{request:NativeDecodeAllocation,actual:usize},Cancel(NativeDecodeAllocation)}

/// 🚚️ Carries original receiving authority through native erased futures.
#[cfg(not(target_arch="wasm32"))]
pub type NativeDecodeAllocationPort<'a>=dyn FnMut(NativeDecodeAllocationEvent)->Result<(),ValueError>+Send+'a;
/// 🌐️ Keeps original receiving authority on the local guest executor.
#[cfg(target_arch="wasm32")]
pub type NativeDecodeAllocationPort<'a>=dyn FnMut(NativeDecodeAllocationEvent)->Result<(),ValueError>+'a;
static ALLOCATION_LOAN_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
/// 🪪️ Retains the original allocation authority across short callback loans.
pub struct NativeDecodeAllocationIssuer{identity:u64}
impl NativeDecodeAllocationIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{std::mem::size_of::<u64>()}
 /// 🏷️ Creates one non-reusable original issuer.
 pub fn new()->Self{let identity=ALLOCATION_LOAN_IDS.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("allocation issuer identity exhausted");Self{identity}}
 /// 🔗️ Borrows the original callback without changing its issuer.
 pub fn loan<'a>(&'a mut self,port:&'a mut NativeDecodeAllocationPort<'a>)->NativeDecodeAllocationLoan<'a>{NativeDecodeAllocationLoan{identity:self.identity,port,issuer:std::marker::PhantomData}}
}
impl Default for NativeDecodeAllocationIssuer{fn default()->Self{Self::new()}}
/// 🎟️ Carries one short original callback borrow and its issuer identity.
pub struct NativeDecodeAllocationLoan<'a>{identity:u64,port:&'a mut NativeDecodeAllocationPort<'a>,issuer:std::marker::PhantomData<&'a mut NativeDecodeAllocationIssuer>}
impl<'a> NativeDecodeAllocationLoan<'a>{pub(super) fn reborrow(&mut self)->NativeDecodeAllocationLoan<'_>{NativeDecodeAllocationLoan{identity:self.identity,port:&mut *self.port,issuer:std::marker::PhantomData}}}
impl<'a> std::ops::Deref for NativeDecodeAllocationLoan<'a>{type Target=NativeDecodeAllocationPort<'a>;fn deref(&self)->&Self::Target{self.port}}
impl<'a> std::ops::DerefMut for NativeDecodeAllocationLoan<'a>{fn deref_mut(&mut self)->&mut Self::Target{self.port}}
/// 🧳️ Retains operation, extent and independent original allocation settlement refusals.
#[derive(Debug,PartialEq)]
pub struct NativeDecodeAllocationOutcome{pub request:Option<NativeDecodeAllocation>,pub admitted:bool,pub primary:Option<ValueError>,pub quote_refusal:Option<ValueError>,pub extent_refusal:Option<ValueError>,pub settlement_refusal:Option<ValueError>,pub actual:Option<usize>}
impl crate::retirement::RetireOwned for NativeDecodeAllocationOutcome{fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.primary),crate::retirement::deferred(self.quote_refusal),crate::retirement::deferred(self.extent_refusal),crate::retirement::deferred(self.settlement_refusal)])}fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.primary),crate::retirement::deferred_birth_bytes_for(&self.quote_refusal),crate::retirement::deferred_birth_bytes_for(&self.extent_refusal),crate::retirement::deferred_birth_bytes_for(&self.settlement_refusal)])}fn controlled_retirement_supported()->bool{true}}
/// 🚦️ Records whether an original callback may already have taken effect.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeDecodeAllocationPhase{Absent,QuoteStarted,QuoteAccepted,QuoteRefused,SettlementStarted,Settled}
/// 🧭️ Keeps the original request and settlement boundary in the caller frame.
pub struct NativeDecodeAllocationProgress{pub(super) request:NativeDecodeAllocation,pub(super) outcome:NativeDecodeAllocationOutcome,pub(super) phase:NativeDecodeAllocationPhase,pub(super) operation_started:bool,settled:bool,identity:Option<u64>}
impl NativeDecodeAllocationProgress{
 /// 🪪️ Borrows the exact original request without exposing mutable authority.
 pub fn request(&self)->NativeDecodeAllocation{self.request}
 /// 📊️ Borrows the retained outcome without modifying callback history.
 pub fn outcome(&self)->&NativeDecodeAllocationOutcome{&self.outcome}
 /// 🔍️ Reports whether the producer began its effect.
 pub fn operation_started(&self)->bool{self.operation_started}
 /// 🚦️ Observes the original callback boundary.
 pub fn phase(&self)->NativeDecodeAllocationPhase{self.phase}
 /// 🫴️ Transfers the outcome only after every original callback is settled.
 pub fn into_outcome(self)->Result<NativeDecodeAllocationOutcome,Self>{if self.pending(){Err(self)}else{let owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::ptr::read(&owner.outcome)})}}
 /// ⏳️ Reports whether original callback custody remains unresolved.
 pub fn is_pending(&self)->bool{self.pending()}
}
impl NativeDecodeAllocationProgress{
 /// 🧭️ Reports whether an original callback still requires resolution.
 pub fn pending(&self)->bool{(self.operation_started&&!self.settled)||matches!(self.phase,NativeDecodeAllocationPhase::QuoteStarted|NativeDecodeAllocationPhase::QuoteAccepted|NativeDecodeAllocationPhase::SettlementStarted)}}
impl Drop for NativeDecodeAllocationProgress{fn drop(&mut self){assert!(std::thread::panicking()||!self.pending(),"original allocation callback remains unsettled")}}
struct PendingAllocationProgress{_progress:NativeDecodeAllocationProgress}
impl crate::retirement::RetirementCursor for PendingAllocationProgress{fn close_step(&mut self,_:crate::RetainedCloneGrant)->crate::retirement::RetirementStep{crate::retirement::RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original allocation callback must be resolved before retirement").into(),0)}fn terminal_is_empty(&self)->bool{false}}
impl crate::retirement::RetireOwned for NativeDecodeAllocationProgress{fn retirement_ready(&self)->bool{!self.pending()}fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{if self.pending(){Box::new(PendingAllocationProgress{_progress:self})}else{crate::retirement::RetireOwned::retirement(self.into_outcome().ok().expect("settled allocation outcome"))}}fn retirement_birth_bytes(&self)->Option<usize>{if self.pending(){None}else{crate::retirement::RetireOwned::retirement_birth_bytes(&self.outcome)}}fn controlled_retirement_supported()->bool{true}}

impl NativeDecodeControl<'_>{
 /// 📦️ Quotes and settles a caller-retained allocation effect.
 pub fn allocate_effect(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeDecodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<(),ValueError>)->NativeDecodeAllocationOutcome{self.allocate_effect_gated(bytes,actual,progress,&mut |actual|operation(actual).map(|()|true)).expect("ungated allocation always returns its outcome")}
 /// 🧭️ Keeps an original quote pending when a child receipt cannot yet be reconciled.
 pub(super) fn allocate_effect_gated(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeDecodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<bool,ValueError>)->Option<NativeDecodeAllocationOutcome>{
  let empty=|primary|NativeDecodeAllocationOutcome{request:None,admitted:false,primary:Some(primary),quote_refusal:None,extent_refusal:None,settlement_refusal:None,actual:None};if actual.is_some()||progress.is_some(){return Some(empty(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation requires empty retained markers")))}let Some(next)=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes)else{return Some(empty(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoding allocation exceeds original ceiling")))};if let Err(error)=self.checkpoint(){return Some(empty(error))}let request=NativeDecodeAllocation{bytes,owned_bytes:self.owned_bytes,next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};*progress=Some(NativeDecodeAllocationProgress{request,outcome:NativeDecodeAllocationOutcome{request:Some(request),admitted:false,primary:None,quote_refusal:None,extent_refusal:None,settlement_refusal:None,actual:None},phase:if identity.is_some(){NativeDecodeAllocationPhase::QuoteStarted}else{NativeDecodeAllocationPhase::Absent},operation_started:false,settled:false,identity});if let Binding::Forwarded(port)=&mut self.allocation{let result=port(NativeDecodeAllocationEvent::Quote(request));let state=progress.as_mut().unwrap();match result{Ok(())=>state.phase=NativeDecodeAllocationPhase::QuoteAccepted,Err(error)=>{state.phase=NativeDecodeAllocationPhase::QuoteRefused;state.outcome.quote_refusal=Some(error);return Some(progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome"))}}}progress.as_mut().unwrap().operation_started=true;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(actual)));let state=progress.as_mut().unwrap();match result{Ok(Ok(true))=>state.outcome.actual=*actual,Ok(Ok(false))=>return None,Ok(Err(error))=>{state.outcome.actual=*actual;state.outcome.primary=Some(error)},Err(payload)=>{state.outcome.actual=*actual;std::panic::resume_unwind(payload)}}if let Err(error)=self.settle_allocation_progress(state,*actual){return Some(empty(error))}Some(progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome"))
 }
 /// 🪪️ Reconciles a pending callback using its original recipient's actual result.
 pub fn resolve_allocation_callback(&mut self,state:&mut NativeDecodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.identity||!matches!(state.phase,NativeDecodeAllocationPhase::QuoteStarted|NativeDecodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.phase{NativeDecodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.phase=NativeDecodeAllocationPhase::QuoteAccepted,Err(error)=>{state.phase=NativeDecodeAllocationPhase::QuoteRefused;state.outcome.quote_refusal=Some(error)}},NativeDecodeAllocationPhase::SettlementStarted=>{state.outcome.settlement_refusal=result.err();state.phase=NativeDecodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 /// 🪪️ Visits a typed retained frame alongside the original allocation authority.
 pub fn visit_allocation_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeDecodeAllocationRecovery<'_,'_>))->Result<bool,ValueError>{let Some(recipient)=self.retirement.as_deref_mut()else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native allocation recovery requires its original recipient"))};if recipient.identity!=recipient_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation recovery recipient differs from original"))}let mut recovery=NativeDecodeAllocationRecovery{allocation:&mut self.allocation,owned_bytes:&mut self.owned_bytes};Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
 /// 🧾️ Settles a pending quote only through its original port at a known callback boundary.
 pub fn settle_allocation_progress(&mut self,state:&mut NativeDecodeAllocationProgress,actual:Option<usize>)->Result<(),ValueError>{let identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.identity||matches!(state.phase,NativeDecodeAllocationPhase::QuoteStarted|NativeDecodeAllocationPhase::SettlementStarted){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress requires its original port and settled callback boundary"))}if state.settled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress is already settled"))}if actual.is_some()&&!state.operation_started{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation cannot report an unperformed effect"))}if state.outcome.actual.is_some()&&state.outcome.actual!=actual{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation actual differs from retained progress"))}state.outcome.actual=actual;let event=match actual{None=>NativeDecodeAllocationEvent::Cancel(state.request),Some(count)=>{if let Err(error)=commit_admission(&mut self.owned_bytes,state.request,count){state.outcome.extent_refusal=Some(error)}if count>state.request.bytes&&state.outcome.extent_refusal.is_none(){state.outcome.extent_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual native allocation exceeds its quote"))}state.outcome.admitted=true;NativeDecodeAllocationEvent::Post{request:state.request,actual:count}}};if state.phase==NativeDecodeAllocationPhase::QuoteAccepted{state.phase=NativeDecodeAllocationPhase::SettlementStarted;if let Binding::Forwarded(port)=&mut self.allocation{state.outcome.settlement_refusal=port(event).err()}state.phase=NativeDecodeAllocationPhase::Settled}state.settled=true;Ok(())}
}
impl NativeDecodeAllocationOutcome{
 /// 🧳️ Transfers every directional cause into canonical caller-owned failure custody.
 pub fn into_allocation_failure(self,prior:usize)->Option<crate::NativeAllocationFailure>{let Self{request,admitted,primary,quote_refusal,extent_refusal,settlement_refusal,actual}=self;let mut failure=crate::NativeAllocationFailure::default();failure.primary=primary;failure.extent=extent_refusal;failure.forwarded_quote=quote_refusal;if actual.is_some(){failure.forwarded_post=settlement_refusal}else{failure.forwarded_cancel=settlement_refusal}let admitted_bytes=request.map_or(prior,|request|if admitted{actual.and_then(|count|request.owned_bytes.checked_add(count)).unwrap_or(request.owned_bytes)}else{request.owned_bytes});failure.progress=crate::NativeAllocationProgress{prior_admitted_bytes:request.map(|request|request.owned_bytes),requested_bytes:request.map(|request|request.bytes),maximum_bytes:request.map(|request|request.maximum_bytes),admitted_bytes,actual_capacity_bytes:actual};if failure.is_empty(){None}else{Some(failure)}}
}

/// 🧭️ Reborrows the original allocation ports and ledger alongside the retained frame.
pub struct NativeDecodeAllocationRecovery<'a,'control>{allocation:&'a mut Binding<'control>,owned_bytes:&'a mut usize}
impl NativeDecodeAllocationRecovery<'_,'_>{
 pub fn resolve_allocation_callback(&mut self,state:&mut NativeDecodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=match &mut *self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.identity||!matches!(state.phase,NativeDecodeAllocationPhase::QuoteStarted|NativeDecodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.phase{NativeDecodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.phase=NativeDecodeAllocationPhase::QuoteAccepted,Err(error)=>{state.phase=NativeDecodeAllocationPhase::QuoteRefused;state.outcome.quote_refusal=Some(error)}},NativeDecodeAllocationPhase::SettlementStarted=>{state.outcome.settlement_refusal=result.err();state.phase=NativeDecodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 pub fn settle_allocation_progress(&mut self,state:&mut NativeDecodeAllocationProgress,actual:Option<usize>)->Result<(),ValueError>{let identity=match &mut *self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.identity||matches!(state.phase,NativeDecodeAllocationPhase::QuoteStarted|NativeDecodeAllocationPhase::SettlementStarted){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress requires its original port and settled callback boundary"))}if state.settled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress is already settled"))}if actual.is_some()&&!state.operation_started{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation cannot report an unperformed effect"))}if state.outcome.actual.is_some()&&state.outcome.actual!=actual{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation actual differs from retained progress"))}state.outcome.actual=actual;let event=match actual{None=>NativeDecodeAllocationEvent::Cancel(state.request),Some(count)=>{if let Err(error)=commit_admission(&mut *self.owned_bytes,state.request,count){state.outcome.extent_refusal=Some(error)}if count>state.request.bytes&&state.outcome.extent_refusal.is_none(){state.outcome.extent_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual native allocation exceeds its quote"))}state.outcome.admitted=true;NativeDecodeAllocationEvent::Post{request:state.request,actual:count}}};if state.phase==NativeDecodeAllocationPhase::QuoteAccepted{state.phase=NativeDecodeAllocationPhase::SettlementStarted;if let Binding::Forwarded(port)=&mut *self.allocation{state.outcome.settlement_refusal=port(event).err()}state.phase=NativeDecodeAllocationPhase::Settled}state.settled=true;Ok(())}
}
pub(super) enum Binding<'a>{Local,Forwarded(&'a mut NativeDecodeAllocationLoan<'a>)}
/// 🧵️ Retains original ports, retirement slot and cumulative receipt across callback lifetimes.
pub struct NativeForwardedDecodeContinuation<'a>{receipt:Option<NativeDecodeContinuation>,allocate:&'a mut NativeDecodeAllocationLoan<'a>,retirement:Option<&'a mut NativeDecodeRetirementRecipient>}
/// 🧮️ Exposes canonical parsing under original receiving allocation admission.
pub struct NativeForwardedDecodeControl<'a>{inner:NativeDecodeControl<'a>}

/// 🧳️ The original caller retains the cumulative receipt across a forwarded callback or unwind.
pub struct NativeForwardedDecodeFrame<'owner,'control>{control:NativeForwardedDecodeControl<'control>,receipt:&'owner mut Option<NativeForwardedDecodeReceipt>,recipient:u64,allocation_identity:u64}
impl<'owner,'control> NativeForwardedDecodeFrame<'owner,'control>{
 /// ▶️ Starts or rebinds a caller-owned receipt under its original live allocation and retirement ports.
 pub fn begin(receipt:&'owner mut Option<NativeForwardedDecodeReceipt>,maximum:usize,callback:&'control mut dyn FnMut(NativeDecodeProgress)->bool,allocation:&'control mut NativeDecodeAllocationLoan<'control>,recipient:&'control mut NativeDecodeRetirementRecipient)->Result<Self,ValueError>{let recipient_identity=recipient.identity;let allocation_identity=allocation.identity;let control=if let Some(saved)=receipt.take(){match NativeForwardedDecodeControl::rebind(saved,callback,allocation,recipient){Ok(control)=>control,Err((error,saved))=>{*receipt=Some(saved);return Err(error)}}}else{let mut control=NativeDecodeControl::new_forwarded(maximum,callback,allocation);control.inner.retirement=Some(recipient);control};Ok(Self{control,receipt,recipient:recipient_identity,allocation_identity})}
 /// 🫴️ Lends only this original forwarded decoder for the current callback.
 pub fn control(&mut self)->&mut NativeForwardedDecodeControl<'control>{&mut self.control}
}
impl Drop for NativeForwardedDecodeFrame<'_,'_>{fn drop(&mut self){let inner=&mut self.control.inner;let receipt=NativeDecodeContinuation{actual_work:inner.work.actual,reserved_work:inner.work.reserved,reserved_issuer:inner.work.reserved_issuer,reserved_action:inner.work.reserved_action,work_quote_pending:inner.work.quote_pending,ledger:inner.ledger,receiving:inner.receiving,maximum_bytes:inner.maximum_bytes,owned_bytes:inner.owned_bytes,completed:inner.completed,total:inner.total,stage:inner.stage};*self.receipt=Some(NativeForwardedDecodeReceipt{receipt,recipient:self.recipient,allocation_identity:self.allocation_identity});inner.work.reserved=0;inner.work.reserved_issuer=None;inner.work.reserved_action=None;inner.work.quote_pending=false}}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedDecodeReceipt{receipt:NativeDecodeContinuation,recipient:u64,allocation_identity:u64}
impl crate::retirement::RetireOwned for NativeForwardedDecodeReceipt{
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.receipt)&&crate::retirement::RetireOwned::retirement_birth_bytes(&self.receipt).is_some()}
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::deferred(self.receipt)}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_birth_bytes(&self.receipt).map(|_|crate::retirement::deferred_birth_bytes_for(&self.receipt))}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeForwardedDecodeReceipt{
 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
 /// ⏳️ Reports an unresolved original work quote or admitted reservation.
 pub fn has_pending_work(&self)->bool{self.receipt.reserved_issuer.is_some()||self.receipt.work_quote_pending}
}
impl<'a> NativeDecodeControl<'a>{
 /// 🏭️ Binds every reservation to the original caller allocation port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut NativeDecodeAllocationLoan<'a>)->NativeForwardedDecodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=Binding::Forwarded(allocate);NativeForwardedDecodeControl{inner}}
}
impl<'a> NativeForwardedDecodeControl<'a>{
 /// ⏹️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(mut self)->Result<NativeForwardedDecodeReceipt,(ValueError,Self)>{
  if self.inner.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction loan cannot detach from its original receiving hop"),self));}
  let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};
  let Some(recipient)=self.inner.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};
  let recipient_identity=recipient.identity;let allocation_identity=match &self.inner.allocation{Binding::Forwarded(port)=>port.identity,Binding::Local=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded allocation loan is absent"),self))};self.inner.work.reserved=0;self.inner.work.reserved_issuer=None;self.inner.work.reserved_action=None;self.inner.work.quote_pending=false;Ok(NativeForwardedDecodeReceipt{receipt,recipient:recipient_identity,allocation_identity})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedDecodeReceipt,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut NativeDecodeAllocationLoan<'a>,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedDecodeReceipt)>{
  if receipt.recipient!=recipient.identity||receipt.allocation_identity!=allocate.identity||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let NativeForwardedDecodeReceipt{receipt:original,recipient:recipient_identity,allocation_identity}=receipt;let mut inner=match NativeDecodeControl::resume(original,callback){Ok(inner)=>inner,Err((error,original))=>return Err((error,NativeForwardedDecodeReceipt{receipt:original,recipient:recipient_identity,allocation_identity}))};inner.allocation=Binding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves original allocation and retirement authority together with the cumulative receipt.
 pub fn pause(mut self)->Result<NativeForwardedDecodeContinuation<'a>,(ValueError,Self)>{if self.inner.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction loan cannot pause outside its original receiving hop"),self))}let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};if !matches!(self.inner.allocation,Binding::Forwarded(_)){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded decoder lacks original allocation port"),self))}let Binding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,Binding::Local)else{unreachable!()};self.inner.work.reserved=0;self.inner.work.reserved_issuer=None;self.inner.work.reserved_action=None;self.inner.work.quote_pending=false;Ok(NativeForwardedDecodeContinuation{receipt:Some(receipt),allocate,retirement:self.inner.retirement.take()})}
 /// ⏳️ Rebinds only progress while retaining original allocation and retirement authority.
 pub fn resume(mut receipt:NativeForwardedDecodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,(ValueError,NativeForwardedDecodeContinuation<'a>)>{let Some(original)=receipt.receipt.take()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving"),receipt))};let mut inner=match NativeDecodeControl::resume(original,callback){Ok(inner)=>inner,Err((error,original))=>{receipt.receipt=Some(original);return Err((error,receipt))}};inner.allocation=Binding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
impl<'a> NativeForwardedDecodeContinuation<'a>{
 /// 🪢️ Binds the original caller port directly without creating replacement progress authority.
 pub fn new(maximum_bytes:usize,allocate:&'a mut NativeDecodeAllocationLoan<'a>)->Self{Self{receipt:Some(NativeDecodeContinuation{actual_work:0,reserved_work:0,reserved_issuer:None,reserved_action:None,work_quote_pending:false,ledger:super::next_ledger(),receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🫴️ Installs the original caller return slot before retained decoder ownership exists.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded decoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 🔁️ Reborrows original ports and retirement slot for one synchronous receiving hop.
 pub fn decode<T>(&mut self,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving"))?;let mut allocation_loan=self.allocate.reborrow();let mut control=match NativeDecodeControl::resume(receipt,callback){Ok(control)=>control,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(error)}};control.allocation=Binding::Forwarded(&mut allocation_loan);control.retirement=self.retirement.as_deref_mut();let result=control.scoped_stage(operation);let receipt=control.continuation()?;control.work.reserved=0;control.work.reserved_issuer=None;control.work.reserved_action=None;control.work.quote_pending=false;self.receipt=Some(receipt);result}
 /// 📬️ Returns the original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").maximum_bytes}
 /// 📩️ Returns cumulative original receiving admission.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").owned_bytes}
}
impl<'a> std::ops::Deref for NativeForwardedDecodeControl<'a>{type Target=NativeDecodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedDecodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}

#[cfg(test)]
#[path="🧪️tests/🪢️unwind.rs"]
mod unwind_tests;

/// 🎟️ Tracks per-hop physical quotes and actual posts without changing cumulative admission.
#[derive(Default)]
pub struct NativeDecodeAllocationBudget{pending:Option<(NativeDecodeAllocation,crate::RetainedCloneGrant)>,spent:usize}
impl NativeDecodeAllocationBudget{
 /// 🔁️ Starts a new caller hop only after its earlier quote settled.
 pub fn begin_hop(&mut self){if self.pending.is_none(){self.spent=0}}
 /// 📊️ Borrows actual physical extent observed in this hop.
 pub fn spent(&self)->usize{self.spent}
 /// 🧾️ Borrows the unchanged pending request and original grant.
 pub fn pending(&self)->Option<(NativeDecodeAllocation,crate::RetainedCloneGrant)>{self.pending}
 /// 🏁️ Reports that no original allocation callback remains pending.
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()}
 pub fn receive(&mut self,event:NativeDecodeAllocationEvent,grant:crate::RetainedCloneGrant,may_quote:bool)->Result<(),ValueError>{
  match event{
   NativeDecodeAllocationEvent::Quote(request)=>{
    if self.pending.is_some(){return Err(budget_invariant("original native allocation already retains a quote"))}
    if !may_quote{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode caller yielded"))}
    if self.spent.checked_add(request.bytes).is_none_or(|bytes|bytes>grant.maximum_capacity_bytes){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native decode reservation exceeds actual turn"))}
    self.pending=Some((request,grant));Ok(())
   },
   NativeDecodeAllocationEvent::Post{request,actual}=>{
    let Some((original,issued))=self.pending else{return Err(budget_invariant("original native allocation post has no quote"))};
    if original!=request||issued!=grant{return Err(budget_invariant("original native allocation post changed its quoted authority"))}
    self.pending=None;
    self.spent=self.spent.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native physical extent overflow"))?;
    if actual>request.bytes||self.spent>issued.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native physical allocation exceeds its quote"))}Ok(())
   },
   NativeDecodeAllocationEvent::Cancel(request)=>{
    let Some((original,issued))=self.pending else{return Err(budget_invariant("original native allocation cancellation has no quote"))};
    if original!=request||issued!=grant{return Err(budget_invariant("original native allocation cancellation changed its quoted authority"))}
    self.pending=None;Ok(())
   }
  }
 }
}
impl Drop for NativeDecodeAllocationBudget{fn drop(&mut self){assert!(std::thread::panicking()||self.pending.is_none(),"original native allocation quote remains unsettled")}}
fn budget_invariant(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
