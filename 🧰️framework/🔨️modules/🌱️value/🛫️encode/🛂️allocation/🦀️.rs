//! 🛂️ Foreign physical factories obtain original receiving admission before each reservation.
//! 📏️ Cumulative admission records requested bytes; physical posts independently report actual capacity.
//! A performed allocator failure posts zero; cancellation records an effect that never began.
use super::{NativeEncodeControl,NativeEncodeContinuation,NativeEncodeProgress,NativeEncodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};
fn commit_admission(ledger:&mut usize,request:NativeEncodeAllocation,actual:usize)->Result<(),ValueError>{let next=request.owned_bytes.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"actual native allocation extent overflow"))?;*ledger=next;Ok(())}
#[path="📦️slot/🦀️.rs"]
mod slot;

/// 📦️ Describes one exact requested allocation before either ledger or storage changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeAllocation{pub bytes:usize,pub owned_bytes:usize,pub next_owned_bytes:usize,pub maximum_bytes:usize}
/// 🧾️ Quotes original authority, then posts an actual effect or cancels before effect.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeAllocationEvent{Quote(NativeEncodeAllocation),Post{request:NativeEncodeAllocation,actual:usize},Cancel(NativeEncodeAllocation)}


/// 🚚️ Carries original receiving authority through native erased futures.
#[cfg(not(target_arch="wasm32"))]
pub type NativeEncodeAllocationPort<'a>=dyn FnMut(NativeEncodeAllocationEvent)->Result<(),ValueError>+Send+'a;
/// 🌐️ Keeps the same original receiving authority on the local guest executor.
#[cfg(target_arch="wasm32")]
pub type NativeEncodeAllocationPort<'a>=dyn FnMut(NativeEncodeAllocationEvent)->Result<(),ValueError>+'a;

static ALLOCATION_LOAN_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
/// 🪪️ Retains the original allocation authority across short callback loans.
pub struct NativeEncodeAllocationIssuer{identity:u64}
impl NativeEncodeAllocationIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{std::mem::size_of::<u64>()}
 /// 🏷️ Creates one non-reusable original issuer.
 pub fn new()->Self{let identity=ALLOCATION_LOAN_IDS.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("allocation issuer identity exhausted");Self{identity}}
 /// 🔗️ Borrows the original callback without changing its issuer.
 pub fn loan<'a>(&'a mut self,port:&'a mut NativeEncodeAllocationPort<'a>)->NativeEncodeAllocationLoan<'a>{NativeEncodeAllocationLoan{identity:self.identity,port,issuer:std::marker::PhantomData}}
 /// 🧷️ Lends the original scoped callback under the same durable issuer.
 pub fn scoped_loan<'a>(&'a mut self,port:&'a mut dyn FnMut(NativeEncodeAllocationEvent)->Result<(),ValueError>)->NativeEncodeScopedAllocationLoan<'a>{NativeEncodeScopedAllocationLoan{identity:self.identity,port,issuer:std::marker::PhantomData}}
}
impl Default for NativeEncodeAllocationIssuer{fn default()->Self{Self::new()}}
/// 🎟️ Carries one short original callback borrow and its issuer identity.
pub struct NativeEncodeAllocationLoan<'a>{identity:u64,port:&'a mut NativeEncodeAllocationPort<'a>,issuer:std::marker::PhantomData<&'a mut NativeEncodeAllocationIssuer>}
impl<'a> NativeEncodeAllocationLoan<'a>{pub(super) fn reborrow(&mut self)->NativeEncodeAllocationLoan<'_>{NativeEncodeAllocationLoan{identity:self.identity,port:&mut *self.port,issuer:std::marker::PhantomData}}}
impl<'a> std::ops::Deref for NativeEncodeAllocationLoan<'a>{type Target=NativeEncodeAllocationPort<'a>;fn deref(&self)->&Self::Target{self.port}}
impl<'a> std::ops::DerefMut for NativeEncodeAllocationLoan<'a>{fn deref_mut(&mut self)->&mut Self::Target{self.port}}
/// 🧷️ Keeps a short scoped callback borrow tied to its durable issuer.
pub struct NativeEncodeScopedAllocationLoan<'a>{pub(super) identity:u64,port:&'a mut dyn FnMut(NativeEncodeAllocationEvent)->Result<(),ValueError>,issuer:std::marker::PhantomData<&'a mut NativeEncodeAllocationIssuer>}
impl NativeEncodeScopedAllocationLoan<'_>{pub(super) fn reborrow(&mut self)->NativeEncodeScopedAllocationLoan<'_>{NativeEncodeScopedAllocationLoan{identity:self.identity,port:&mut *self.port,issuer:std::marker::PhantomData}}}
impl<'a> std::ops::Deref for NativeEncodeScopedAllocationLoan<'a>{type Target=dyn FnMut(NativeEncodeAllocationEvent)->Result<(),ValueError>+'a;fn deref(&self)->&Self::Target{self.port}}
impl<'a> std::ops::DerefMut for NativeEncodeScopedAllocationLoan<'a>{fn deref_mut(&mut self)->&mut Self::Target{self.port}}
/// 🧳️ Owns the operation, extent and both original allocation settlement refusals.
#[derive(Debug,PartialEq)]
pub struct NativeEncodeAllocationOutcome{pub request:Option<NativeEncodeAllocation>,pub admitted:bool,pub primary:Option<ValueError>,pub forwarded_quote:Option<ValueError>,pub scoped_quote:Option<ValueError>,pub extent_refusal:Option<ValueError>,pub forwarded_refusal:Option<ValueError>,pub scoped_refusal:Option<ValueError>,pub actual:Option<usize>}
impl crate::retirement::RetireOwned for NativeEncodeAllocationOutcome{fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.primary),crate::retirement::deferred(self.forwarded_quote),crate::retirement::deferred(self.scoped_quote),crate::retirement::deferred(self.extent_refusal),crate::retirement::deferred(self.forwarded_refusal),crate::retirement::deferred(self.scoped_refusal)])}fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.primary),crate::retirement::deferred_birth_bytes_for(&self.forwarded_quote),crate::retirement::deferred_birth_bytes_for(&self.scoped_quote),crate::retirement::deferred_birth_bytes_for(&self.extent_refusal),crate::retirement::deferred_birth_bytes_for(&self.forwarded_refusal),crate::retirement::deferred_birth_bytes_for(&self.scoped_refusal)])}fn controlled_retirement_supported()->bool{true}}
/// 🚦️ Records whether an original port callback may already have taken effect.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeAllocationPhase{Absent,QuoteStarted,QuoteAccepted,QuoteRefused,SettlementStarted,Settled}
/// 🧭️ Retains the original allocation request and callback boundaries during unwinding.
pub struct NativeEncodeAllocationProgress{pub(super) request:NativeEncodeAllocation,pub(super) outcome:NativeEncodeAllocationOutcome,pub(super) forwarded:NativeEncodeAllocationPhase,pub(super) scoped:NativeEncodeAllocationPhase,pub(super) operation_started:bool,settled:bool,forwarded_identity:Option<u64>,scoped_identity:Option<u64>}
impl NativeEncodeAllocationProgress{
 /// 🪪️ Borrows the exact original request without exposing mutable authority.
 pub fn request(&self)->NativeEncodeAllocation{self.request}
 /// 📊️ Borrows the retained outcome without modifying callback history.
 pub fn outcome(&self)->&NativeEncodeAllocationOutcome{&self.outcome}
 /// 🔍️ Reports whether the producer began its effect.
 pub fn operation_started(&self)->bool{self.operation_started}
 /// 🚦️ Observes the original callback boundary.
 pub fn forwarded_phase(&self)->NativeEncodeAllocationPhase{self.forwarded}pub fn scoped_phase(&self)->NativeEncodeAllocationPhase{self.scoped}
 /// 🫴️ Transfers the outcome only after every original callback is settled.
 pub fn into_outcome(self)->Result<NativeEncodeAllocationOutcome,Self>{if self.pending(){Err(self)}else{let owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::ptr::read(&owner.outcome)})}}
 /// ⏳️ Reports whether original callback custody remains unresolved.
 pub fn is_pending(&self)->bool{self.pending()}
}
impl NativeEncodeAllocationProgress{
 /// 🧭️ Reports whether an original callback still requires resolution.
 pub fn pending(&self)->bool{(self.operation_started&&!self.settled)||matches!(self.forwarded,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::QuoteAccepted|NativeEncodeAllocationPhase::SettlementStarted)||matches!(self.scoped,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::QuoteAccepted|NativeEncodeAllocationPhase::SettlementStarted)}}
impl Drop for NativeEncodeAllocationProgress{fn drop(&mut self){assert!(std::thread::panicking()||!self.pending(),"original allocation callback remains unsettled")}}
struct PendingAllocationProgress{_progress:NativeEncodeAllocationProgress}
impl crate::retirement::RetirementCursor for PendingAllocationProgress{fn close_step(&mut self,_:crate::RetainedCloneGrant)->crate::retirement::RetirementStep{crate::retirement::RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original allocation callback must be resolved before retirement").into(),0)}fn terminal_is_empty(&self)->bool{false}}
impl crate::retirement::RetireOwned for NativeEncodeAllocationProgress{fn retirement_ready(&self)->bool{!self.pending()}fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{if self.pending(){Box::new(PendingAllocationProgress{_progress:self})}else{crate::retirement::RetireOwned::retirement(self.into_outcome().ok().expect("settled allocation outcome"))}}fn retirement_birth_bytes(&self)->Option<usize>{if self.pending(){None}else{crate::retirement::RetireOwned::retirement_birth_bytes(&self.outcome)}}fn controlled_retirement_supported()->bool{true}}

impl NativeEncodeControl<'_>{
 /// 📦️ Quotes and settles an effect while caller-owned progress survives every callback.
 pub fn allocate_effect(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<(),ValueError>)->NativeEncodeAllocationOutcome{self.allocation_effect(bytes,actual,progress,operation)}
 fn allocation_effect(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>)->Result<(),ValueError>)->NativeEncodeAllocationOutcome{
  let empty=|primary|NativeEncodeAllocationOutcome{request:None,admitted:false,primary:Some(primary),forwarded_quote:None,scoped_quote:None,extent_refusal:None,forwarded_refusal:None,scoped_refusal:None,actual:None};if actual.is_some()||progress.is_some(){return empty(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation requires empty retained markers"))}let Some(next)=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes)else{return empty(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding allocation exceeds original ceiling"))};if let Err(error)=self.checkpoint(){return empty(error)}let request=NativeEncodeAllocation{bytes,owned_bytes:self.owned_bytes,next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let forwarded_identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};let scoped_identity=self.scoped_allocation.as_mut().map(|port|port.identity);*progress=Some(NativeEncodeAllocationProgress{request,outcome:NativeEncodeAllocationOutcome{request:Some(request),admitted:false,primary:None,forwarded_quote:None,scoped_quote:None,extent_refusal:None,forwarded_refusal:None,scoped_refusal:None,actual:None},forwarded:if forwarded_identity.is_some(){NativeEncodeAllocationPhase::QuoteStarted}else{NativeEncodeAllocationPhase::Absent},scoped:NativeEncodeAllocationPhase::Absent,operation_started:false,settled:false,forwarded_identity,scoped_identity});
  if let Binding::Forwarded(port)=&mut self.allocation{let result=port(NativeEncodeAllocationEvent::Quote(request));let state=progress.as_mut().unwrap();match result{Ok(())=>state.forwarded=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.forwarded=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.forwarded_quote=Some(error);return progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome")}}}
  if let Some(port)=self.scoped_allocation.as_mut(){progress.as_mut().unwrap().scoped=NativeEncodeAllocationPhase::QuoteStarted;let result=port(NativeEncodeAllocationEvent::Quote(request));let state=progress.as_mut().unwrap();match result{Ok(())=>state.scoped=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.scoped=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.scoped_quote=Some(error);if let Err(error)=self.settle_allocation_progress(state,None){return empty(error)}return progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome")}}}
  progress.as_mut().unwrap().operation_started=true;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(actual)));let state=progress.as_mut().unwrap();state.outcome.actual=*actual;match result{Ok(result)=>state.outcome.primary=result.err(),Err(payload)=>std::panic::resume_unwind(payload)}if let Err(error)=self.settle_allocation_progress(state,*actual){return empty(error)}progress.take().unwrap().into_outcome().ok().expect("settled allocation outcome")
 }
 /// 🪪️ Reconciles the original forwarded callback using its recipient's actual result.
 pub fn resolve_forwarded_allocation_callback(&mut self,state:&mut NativeEncodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.forwarded_identity||!matches!(state.forwarded,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.forwarded{NativeEncodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.forwarded=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.forwarded=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.forwarded_quote=Some(error)}},NativeEncodeAllocationPhase::SettlementStarted=>{state.outcome.forwarded_refusal=result.err();state.forwarded=NativeEncodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 /// 🪪️ Reconciles the scoped callback using its recipient's actual result.
 pub fn resolve_scoped_allocation_callback(&mut self,state:&mut NativeEncodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=self.scoped_allocation.as_mut().map(|port|port.identity);if identity!=state.scoped_identity||!matches!(state.scoped,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.scoped{NativeEncodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.scoped=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.scoped=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.scoped_quote=Some(error)}},NativeEncodeAllocationPhase::SettlementStarted=>{state.outcome.scoped_refusal=result.err();state.scoped=NativeEncodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 /// 🪪️ Visits a typed retained frame alongside the original allocation authority.
 pub fn visit_allocation_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeEncodeAllocationRecovery<'_,'_>))->Result<bool,ValueError>{let Some(recipient)=self.retirement.as_deref_mut()else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native allocation recovery requires its original recipient"))};if recipient.identity!=recipient_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation recovery recipient differs from original"))}let mut recovery=NativeEncodeAllocationRecovery{allocation:&mut self.allocation,owned_bytes:&mut self.owned_bytes,scoped_allocation:&mut self.scoped_allocation};Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
 /// 🧾️ Settles a pending original quote only when its port identities and callback phases are known.
 pub fn settle_allocation_progress(&mut self,state:&mut NativeEncodeAllocationProgress,actual:Option<usize>)->Result<(),ValueError>{
  let forwarded=match &mut self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};let scoped=self.scoped_allocation.as_mut().map(|port|port.identity);if forwarded!=state.forwarded_identity||scoped!=state.scoped_identity||matches!(state.forwarded,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted)||matches!(state.scoped,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress requires its original ports and settled callback boundaries"))}if state.settled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress is already settled"))}if actual.is_some()&&!state.operation_started{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation cannot report an unperformed effect"))}if state.outcome.actual.is_some()&&state.outcome.actual!=actual{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation actual differs from retained progress"))}state.outcome.actual=actual;let event=match actual{None=>NativeEncodeAllocationEvent::Cancel(state.request),Some(count)=>{if let Err(error)=commit_admission(&mut self.owned_bytes,state.request,count){state.outcome.extent_refusal=Some(error)}if count>state.request.bytes&&state.outcome.extent_refusal.is_none(){state.outcome.extent_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual native allocation exceeds its quote"))}state.outcome.admitted=true;NativeEncodeAllocationEvent::Post{request:state.request,actual:count}}};if state.forwarded==NativeEncodeAllocationPhase::QuoteAccepted{state.forwarded=NativeEncodeAllocationPhase::SettlementStarted;if let Binding::Forwarded(port)=&mut self.allocation{state.outcome.forwarded_refusal=port(event).err()}state.forwarded=NativeEncodeAllocationPhase::Settled}if state.scoped==NativeEncodeAllocationPhase::QuoteAccepted{state.scoped=NativeEncodeAllocationPhase::SettlementStarted;if let Some(port)=self.scoped_allocation.as_mut(){state.outcome.scoped_refusal=port(event).err()}state.scoped=NativeEncodeAllocationPhase::Settled}state.settled=true;Ok(())
 }
}
impl NativeEncodeAllocationOutcome{
 /// 🧳️ Transfers every directional cause into canonical caller-owned failure custody.
 pub fn into_allocation_failure(self,prior:usize)->Option<crate::NativeAllocationFailure>{let Self{request,admitted,primary,forwarded_quote,scoped_quote,extent_refusal,forwarded_refusal,scoped_refusal,actual}=self;let mut failure=crate::NativeAllocationFailure::default();failure.primary=primary;failure.extent=extent_refusal;failure.forwarded_quote=forwarded_quote;failure.scoped_quote=scoped_quote;if actual.is_some(){failure.forwarded_post=forwarded_refusal;failure.scoped_post=scoped_refusal}else{failure.forwarded_cancel=forwarded_refusal;failure.scoped_cancel=scoped_refusal}let admitted_bytes=request.map_or(prior,|request|if admitted{actual.and_then(|count|request.owned_bytes.checked_add(count)).unwrap_or(request.owned_bytes)}else{request.owned_bytes});failure.progress=crate::NativeAllocationProgress{prior_admitted_bytes:request.map(|request|request.owned_bytes),requested_bytes:request.map(|request|request.bytes),maximum_bytes:request.map(|request|request.maximum_bytes),admitted_bytes,actual_capacity_bytes:actual};if failure.is_empty(){None}else{Some(failure)}}
}

/// 🧭️ Reborrows the original allocation ports and ledger alongside the retained frame.
pub struct NativeEncodeAllocationRecovery<'a,'control>{allocation:&'a mut Binding<'control>,owned_bytes:&'a mut usize,scoped_allocation:&'a mut Option<&'control mut NativeEncodeScopedAllocationLoan<'control>>}
impl NativeEncodeAllocationRecovery<'_,'_>{
 pub fn resolve_forwarded_allocation_callback(&mut self,state:&mut NativeEncodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=match &mut *self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};if identity!=state.forwarded_identity||!matches!(state.forwarded,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.forwarded{NativeEncodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.forwarded=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.forwarded=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.forwarded_quote=Some(error)}},NativeEncodeAllocationPhase::SettlementStarted=>{state.outcome.forwarded_refusal=result.err();state.forwarded=NativeEncodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 pub fn resolve_scoped_allocation_callback(&mut self,state:&mut NativeEncodeAllocationProgress,result:Result<(),ValueError>)->Result<(),(ValueError,Result<(),ValueError>)>{let identity=self.scoped_allocation.as_mut().map(|port|port.identity);if identity!=state.scoped_identity||!matches!(state.scoped,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation callback is not pending on this original port"),result))}match state.scoped{NativeEncodeAllocationPhase::QuoteStarted=>match result{Ok(())=>state.scoped=NativeEncodeAllocationPhase::QuoteAccepted,Err(error)=>{state.scoped=NativeEncodeAllocationPhase::QuoteRefused;state.outcome.scoped_quote=Some(error)}},NativeEncodeAllocationPhase::SettlementStarted=>{state.outcome.scoped_refusal=result.err();state.scoped=NativeEncodeAllocationPhase::Settled},_=>unreachable!()}Ok(())}
 pub fn settle_allocation_progress(&mut self,state:&mut NativeEncodeAllocationProgress,actual:Option<usize>)->Result<(),ValueError>{
  let forwarded=match &mut *self.allocation{Binding::Forwarded(port)=>Some(port.identity),Binding::Local=>None};let scoped=self.scoped_allocation.as_mut().map(|port|port.identity);if forwarded!=state.forwarded_identity||scoped!=state.scoped_identity||matches!(state.forwarded,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted)||matches!(state.scoped,NativeEncodeAllocationPhase::QuoteStarted|NativeEncodeAllocationPhase::SettlementStarted){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress requires its original ports and settled callback boundaries"))}if state.settled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation progress is already settled"))}if actual.is_some()&&!state.operation_started{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation cannot report an unperformed effect"))}if state.outcome.actual.is_some()&&state.outcome.actual!=actual{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation actual differs from retained progress"))}state.outcome.actual=actual;let event=match actual{None=>NativeEncodeAllocationEvent::Cancel(state.request),Some(count)=>{if let Err(error)=commit_admission(&mut *self.owned_bytes,state.request,count){state.outcome.extent_refusal=Some(error)}if count>state.request.bytes&&state.outcome.extent_refusal.is_none(){state.outcome.extent_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual native allocation exceeds its quote"))}state.outcome.admitted=true;NativeEncodeAllocationEvent::Post{request:state.request,actual:count}}};if state.forwarded==NativeEncodeAllocationPhase::QuoteAccepted{state.forwarded=NativeEncodeAllocationPhase::SettlementStarted;if let Binding::Forwarded(port)=&mut *self.allocation{state.outcome.forwarded_refusal=port(event).err()}state.forwarded=NativeEncodeAllocationPhase::Settled}if state.scoped==NativeEncodeAllocationPhase::QuoteAccepted{state.scoped=NativeEncodeAllocationPhase::SettlementStarted;if let Some(port)=self.scoped_allocation.as_mut(){state.outcome.scoped_refusal=port(event).err()}state.scoped=NativeEncodeAllocationPhase::Settled}state.settled=true;Ok(())
 }
}
pub(super) enum Binding<'a>{Local,Forwarded(&'a mut NativeEncodeAllocationLoan<'a>)}

/// 🧵️ Retains the original allocation port and cumulative ledger across progress callback lifetimes.
pub struct NativeForwardedEncodeContinuation<'a>{receipt:Option<NativeEncodeContinuation>,allocate:&'a mut NativeEncodeAllocationLoan<'a>,retirement:Option<&'a mut NativeEncodeRetirementRecipient>}

/// 🧮️ Exposes the canonical controlled serializer while preserving foreign allocation provenance.
pub struct NativeForwardedEncodeControl<'a>{inner:NativeEncodeControl<'a>}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedEncodeReceipt{receipt:NativeEncodeContinuation,recipient:u64,allocation_identity:u64}
impl crate::retirement::RetireOwned for NativeForwardedEncodeReceipt{
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.receipt)}
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::deferred(self.receipt)}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(&self.receipt).then(||crate::retirement::deferred_birth_bytes_for(&self.receipt))}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeForwardedEncodeReceipt{
 /// 🔗️ Reborrows the original retained port without borrowing a progress observer.
 pub fn bind<'a>(self,allocate:&'a mut NativeEncodeAllocationLoan<'a>,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<NativeForwardedEncodeContinuation<'a>,(ValueError,Self)>{
  if self.recipient!=recipient.identity||self.allocation_identity!=allocate.identity||(self.receipt.total!=0&&self.receipt.completed>self.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan mismatches original recipient or ledger"),self))}
  Ok(NativeForwardedEncodeContinuation{receipt:Some(self.receipt),allocate,retirement:Some(recipient)})
 }

 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}

impl<'a> NativeEncodeControl<'a>{
 /// 🏭️ Binds every requested reservation to the caller's original receiving port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeAllocationLoan<'a>)->NativeForwardedEncodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=Binding::Forwarded(allocate);NativeForwardedEncodeControl{inner}}
}
impl<'a> NativeForwardedEncodeControl<'a>{
 /// ⏹️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(self)->Result<NativeForwardedEncodeReceipt,(ValueError,Self)>{
  let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};
  let Some(recipient)=self.inner.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};
  Ok(NativeForwardedEncodeReceipt{receipt,recipient:recipient.identity,allocation_identity:match &self.inner.allocation{Binding::Forwarded(port)=>port.identity,Binding::Local=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded allocation loan is absent"),self))}})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedEncodeReceipt,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeAllocationLoan<'a>,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedEncodeReceipt)>{
  if receipt.recipient!=recipient.identity||receipt.allocation_identity!=allocate.identity||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let mut inner=NativeEncodeControl::resume(receipt.receipt,callback).expect("validated original native receiving receipt");inner.allocation=Binding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves the original allocation port together with its cumulative accounting.
 pub fn pause(mut self)->Result<NativeForwardedEncodeContinuation<'a>,ValueError>{let Binding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,Binding::Local)else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native continuation lacks original allocation port"))};let retirement=self.inner.retirement.take();let receipt=self.inner.pause()?;Ok(NativeForwardedEncodeContinuation{receipt:Some(receipt),allocate,retirement})}
 /// ⏳️ Rebinds only progress while the original allocation port remains unchanged.
 pub fn resume(mut receipt:NativeForwardedEncodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,(ValueError,NativeForwardedEncodeContinuation<'a>)>{if let Err(error)=receipt.validate(){return Err((error,receipt))}let mut inner=NativeEncodeControl::resume(receipt.receipt.take().expect("validated original forwarded receipt"),callback).expect("validated original forwarded accounting");inner.allocation=Binding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
impl<'a> NativeForwardedEncodeContinuation<'a>{
 fn validate(&self)->Result<(),ValueError>{let receipt=self.receipt.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation is not at a receiving boundary"))?;if (receipt.total!=0&&receipt.completed>receipt.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation has invalid original accounting"))}Ok(())}
 /// 🫴️ Returns the same cumulative receiving receipt while the owner keeps its original ports.
 pub fn detach(mut self)->Result<NativeForwardedEncodeReceipt,ValueError>{
  let recipient=self.retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan lacks its original retirement recipient"))?;
  let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan lacks its consuming receipt"))?;
  Ok(NativeForwardedEncodeReceipt{receipt,recipient:recipient.identity,allocation_identity:self.allocate.identity})
 }

 /// 🎟️ Retains the original allocation port before the first physical hop binds its progress observer.
 pub fn new(maximum_bytes:usize,allocate:&'a mut NativeEncodeAllocationLoan<'a>)->Self{Self{receipt:Some(NativeEncodeContinuation{actual_work:0,receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🔁️ Runs one synchronous hop under a shorter progress borrow and preserves the original port on refusal.
 pub fn encode<T>(&mut self,callback:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{self.validate()?;let receipt=self.receipt.take().expect("validated original forwarded receipt");let mut allocation_loan=self.allocate.reborrow();let mut control=NativeEncodeControl::resume(receipt,callback).expect("validated original forwarded accounting");control.allocation=Binding::Forwarded(&mut allocation_loan);control.retirement=self.retirement.as_deref_mut();let result=control.scoped_stage(operation);self.receipt=Some(control.continuation()?);result}
 /// 🧷️ Installs the same caller return slot for every subsequent physical hop.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded encoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 📬️ Returns the same complete receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").maximum_bytes}
 /// 📩️ Returns ownership admitted by the original receiving port through every hop.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").owned_bytes}
}
impl<'a> std::ops::Deref for NativeForwardedEncodeControl<'a>{type Target=NativeEncodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedEncodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🔒️receipt/🦀️.rs"]
mod receipt_integrity_tests;

#[cfg(test)]
#[path="🧪️tests/🪢️unwind.rs"]
mod unwind_tests;

impl NativeEncodeControl<'_>{
 /// 🧳️ Places a completed original allocation outcome in the caller-owned slot before later callbacks.
 pub fn allocate_effect_into(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,outcome:&mut Option<NativeEncodeAllocationOutcome>,operation:&mut impl FnMut(&mut Option<usize>)->Result<(),ValueError>)->Result<(),ValueError>{if outcome.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation outcome slot is occupied"))}*outcome=Some(self.allocate_effect(bytes,actual,progress,operation));Ok(())}
}
