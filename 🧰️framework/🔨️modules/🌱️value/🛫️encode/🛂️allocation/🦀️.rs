//! 🛂️ Foreign physical factories obtain original receiving admission before each reservation.
//! 📏️ Cumulative admission records requested bytes; physical posts independently report actual capacity.
//! A performed allocator failure posts zero; cancellation records an effect that never began.
use super::{NativeEncodeControl,NativeEncodeContinuation,NativeEncodeProgress,NativeEncodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};
fn commit_admission(ledger:&mut crate::native_receiving::NativeAllocationLedger,request:NativeAllocationRequest)->Result<(),ValueError>{ledger.commit(request)}
#[path="📦️slot/🦀️.rs"]
mod slot;

pub use crate::{NativeAllocationEvent,NativeAllocationRequest};


pub use crate::NativeSendAllocationLoan;

pub use semio_framework_value_native_outcome::NativeEncodeAllocationOutcome;

/// 🚦️ Reports the original allocation quote without moving its retained outcome.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeAllocationQuoteStatus{Pending,Ready,Refused}
pub use semio_framework_value_native_outcome::{NativeEncodeAllocationProgress,NativeEncodeAllocationPhase,NativeEncodeAllocationBinding,advance_encode_quotes,settle_encode_allocation};
impl NativeEncodeControl<'_>{
 /// 📦️ Quotes and settles a producer while caller-owned progress retains both original parents.
 /// 🛂️ Quotes the actual Source allocation before lending its producer any effect.
 pub fn prepare_source_allocation(&mut self,source:crate::OriginalSourceTurnWitness<'_>,marker:&mut super::NativeEncodeEffectProgress<'_>,selected:crate::RetainedCloneGrant,bytes:usize)->Result<NativeEncodeAllocationQuoteStatus,ValueError>{
  let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Source allocation quote changed");let (grant,prior)=(*marker.local_binding).ok_or_else(invalid)?;if !prior.fits(grant)||!super::effect::admits(super::effect::remaining(grant,prior),selected){return Err(invalid())}let turn=source.identity();if self.source_turn!=Some(turn)||!matches!(self.allocation,NativeEncodeAllocationBinding::Local)||bytes==0||bytes>selected.maximum_capacity_bytes||marker.outcome.is_some(){return Err(invalid())}let ledger=self.ledger.as_ref().ok_or_else(invalid)?;if *marker.checkpoint_ready!=Some((turn,ledger.identity(),self.stage,ledger.owned_bytes(),self.completed,self.total,self.started))||marker.settlement.as_ref().is_none_or(|state|!state.admits_producer(selected))||marker.selection.is_some_and(|prior|prior!=selected){return Err(invalid())}*marker.selection=Some(selected);let allocation=self.scoped_allocation.as_deref_mut().ok_or_else(invalid)?;let work=self.action.recipient.as_deref_mut().ok_or_else(invalid)?;let physical=self.construction.as_deref_mut().ok_or_else(invalid)?;crate::native_receiving::span::progress::validate_source_authorities(source,allocation,work,physical)?;if marker.allocation.is_none(){if marker.actual.is_some(){return Err(invalid())}let owned=ledger.owned_bytes();let next=owned.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Source allocation exceeds its ceiling"))?;let request=NativeAllocationRequest{bytes,owned_bytes:owned,next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let scoped_identity=Some(allocation.identity());*marker.allocation=Some(NativeEncodeAllocationProgress::new(request,None,scoped_identity,true)?)}let state=marker.allocation.as_mut().expect("original Source allocation retained");if !state.prepared_for_producer()||state.request().bytes!=bytes||state.request().owned_bytes!=ledger.owned_bytes()||state.request().maximum_bytes!=self.maximum_bytes||!state.matches_scoped_authority(&*allocation){return Err(invalid())}if state.operation_started(){return Ok(NativeEncodeAllocationQuoteStatus::Ready)}if !advance_encode_quotes(&mut self.allocation,&mut self.scoped_allocation,state)?{return Ok(NativeEncodeAllocationQuoteStatus::Pending)}if state.quote_refused(){if !settle_encode_allocation(&mut self.allocation,&mut self.scoped_allocation,self.ledger.as_mut().ok_or_else(invalid)?,state,None)?{return Ok(NativeEncodeAllocationQuoteStatus::Pending)}let state=marker.allocation.take().expect("settled Source quote");*marker.outcome=Some(state.into_outcome().ok().expect("settled Source allocation"));return Ok(NativeEncodeAllocationQuoteStatus::Refused)}state.start_operation()?;Ok(NativeEncodeAllocationQuoteStatus::Ready)
 }
 pub fn allocate_effect(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>,&mut semio_framework_value_control_contract::OriginalWorkTurn<'_,'_>)->Result<bool,ValueError>)->Result<Option<NativeEncodeAllocationOutcome>,ValueError>{let empty=|primary|NativeEncodeAllocationOutcome{request:None,admitted:false,primary:Some(primary),forwarded_quote:None,scoped_quote:None,extent_refusal:None,forwarded_refusal:None,scoped_refusal:None,actual:None};if actual.is_some()||progress.is_some(){return Ok(Some(empty(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation requires empty retained markers"))))}let Some(next)=self.owned_bytes().checked_add(bytes).filter(|next|*next<=self.maximum_bytes)else{return Ok(Some(empty(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding allocation exceeds original ceiling"))))};if let Err(error)=self.checkpoint(){return Ok(Some(empty(error)))}let request=NativeAllocationRequest{bytes,owned_bytes:self.owned_bytes(),next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let forwarded_identity=match &mut self.allocation{NativeEncodeAllocationBinding::Forwarded(port)=>Some(port.identity()),NativeEncodeAllocationBinding::Local=>None};let scoped_identity=self.scoped_allocation.as_mut().map(|port|port.identity());*progress=Some(NativeEncodeAllocationProgress::new(request,forwarded_identity,scoped_identity,false)?);self.continue_allocation_effect(actual,progress,operation)}
 /// ▶️ Continues only an unstarted producer after recovering both original quotes.
 pub fn continue_allocation_effect(&mut self,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,operation:&mut impl FnMut(&mut Option<usize>,&mut semio_framework_value_control_contract::OriginalWorkTurn<'_,'_>)->Result<bool,ValueError>)->Result<Option<NativeEncodeAllocationOutcome>,ValueError>{let state=progress.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation progress is absent"))?;if state.operation_started(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation producer already started"))}if !advance_encode_quotes(&mut self.allocation,&mut self.scoped_allocation,state)?{return Ok(None)}if !state.quote_refused(){state.start_operation()?;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut work=semio_framework_value_control_contract::OriginalWorkTurn::new(&mut self.action);operation(actual,&mut work)}));state.record_actual(*actual)?;match result{Ok(Ok(true))=>{},Ok(Ok(false))=>return Ok(None),Ok(Err(error))=>state.record_primary(error)?,Err(payload)=>std::panic::resume_unwind(payload)}}if !settle_encode_allocation(&mut self.allocation,&mut self.scoped_allocation,self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger absent"))?,state,*actual)?{return Ok(None)}Ok(Some(progress.take().expect("completed original allocation").into_outcome().ok().expect("settled original allocation")))}
 /// 🪪️ Recovers original quote callbacks without a caller-supplied verdict.
 pub fn recover_allocation_quotes(&mut self,state:&mut NativeEncodeAllocationProgress)->Result<bool,ValueError>{advance_encode_quotes(&mut self.allocation,&mut self.scoped_allocation,state)}
 /// 🪪️ Visits the original retained frame alongside its exact allocation authorities.
 pub fn visit_allocation_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeEncodeAllocationRecovery<'_,'_>))->Result<bool,ValueError>{let Some(recipient)=self.retirement.as_deref_mut()else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native allocation recovery requires its original recipient"))};if recipient.identity!=recipient_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"allocation recovery recipient differs from original"))}let mut recovery=NativeEncodeAllocationRecovery{allocation:&mut self.allocation,ledger:self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger is lent"))?,scoped_allocation:&mut self.scoped_allocation};Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
 /// 🧾️ Posts the actual marker or cancels before effect, preserving both parent verdicts.
 pub fn settle_allocation_progress(&mut self,state:&mut NativeEncodeAllocationProgress,actual:Option<usize>)->Result<bool,ValueError>{settle_encode_allocation(&mut self.allocation,&mut self.scoped_allocation,self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger is lent"))?,state,actual)}
}
/// 🧭️ Reborrows the original allocation ports and ledger alongside the retained frame.
pub struct NativeEncodeAllocationRecovery<'a,'control>{allocation:&'a mut NativeEncodeAllocationBinding<'control>,ledger:&'a mut crate::native_receiving::NativeAllocationLedger,scoped_allocation:&'a mut Option<&'control mut (dyn NativeSendAllocationLoan+'control)>}
impl NativeEncodeAllocationRecovery<'_,'_>{
 /// 🪪️ Recovers each exact original quote from its resident authority.
 pub fn recover_allocation_quotes(&mut self,state:&mut NativeEncodeAllocationProgress)->Result<bool,ValueError>{advance_encode_quotes(&mut *self.allocation,&mut *self.scoped_allocation,state)}
 /// 🧾️ Settles both original parent transactions with the producer’s actual marker.
 pub fn settle_allocation_progress(&mut self,state:&mut NativeEncodeAllocationProgress,actual:Option<usize>)->Result<bool,ValueError>{settle_encode_allocation(&mut *self.allocation,&mut *self.scoped_allocation,&mut *self.ledger,state,actual)}
}

/// 🧵️ Retains the original allocation port and cumulative ledger across progress callback lifetimes.
pub struct NativeForwardedEncodeContinuation<'a>{receipt:Option<NativeEncodeContinuation>,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),retirement:Option<&'a mut NativeEncodeRetirementRecipient>}

/// 🧮️ Exposes the canonical controlled serializer while preserving foreign allocation provenance.
pub struct NativeForwardedEncodeControl<'a>{inner:NativeEncodeControl<'a>}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedEncodeReceipt{receipt:NativeEncodeContinuation,recipient:u64,allocation_identity:crate::NativeAllocationIdentity}
impl crate::retirement::RetireOwned for NativeForwardedEncodeReceipt{
 type RetirementCursor=<NativeEncodeContinuation as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.receipt)}
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed(self.receipt)}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_birth_bytes(&self.receipt)}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeForwardedEncodeReceipt{
 /// 🔗️ Reborrows the original retained port without borrowing a progress observer.
 pub fn bind<'a>(self,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),recipient:&'a mut NativeEncodeRetirementRecipient)->Result<NativeForwardedEncodeContinuation<'a>,(ValueError,Self)>{
  if self.recipient!=recipient.identity||self.allocation_identity!=allocate.identity()||(self.receipt.total!=0&&self.receipt.completed>self.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan mismatches original recipient or ledger"),self))}
  Ok(NativeForwardedEncodeContinuation{receipt:Some(self.receipt),allocate,retirement:Some(recipient)})
 }

 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes()}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}

impl<'a> NativeEncodeControl<'a>{
 /// 🏭️ Binds every requested reservation to the caller's original receiving port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut (dyn NativeSendAllocationLoan+'a))->NativeForwardedEncodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=NativeEncodeAllocationBinding::Forwarded(allocate);NativeForwardedEncodeControl{inner}}
}
impl<'a> NativeForwardedEncodeControl<'a>{
 /// ⏹️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(mut self)->Result<NativeForwardedEncodeReceipt,(ValueError,Self)>{
  let Some(recipient)=self.inner.retirement.as_ref().map(|recipient|recipient.identity)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};let allocation_identity=match &self.inner.allocation{NativeEncodeAllocationBinding::Forwarded(port)=>port.identity(),NativeEncodeAllocationBinding::Local=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded allocation loan is absent"),self))};let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};Ok(NativeForwardedEncodeReceipt{receipt,recipient,allocation_identity})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedEncodeReceipt,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),recipient:&'a mut NativeEncodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedEncodeReceipt)>{
  if receipt.recipient!=recipient.identity||receipt.allocation_identity!=allocate.identity()||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let mut inner=NativeEncodeControl::resume(receipt.receipt,callback).expect("validated original native receiving receipt");inner.allocation=NativeEncodeAllocationBinding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves the original allocation port together with its cumulative accounting.
 pub fn pause(mut self)->Result<NativeForwardedEncodeContinuation<'a>,(ValueError,Self)>{let NativeEncodeAllocationBinding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,NativeEncodeAllocationBinding::Local)else{return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native continuation lacks original allocation port"),self))};let retirement=self.inner.retirement.take();match self.inner.pause(){Ok(receipt)=>Ok(NativeForwardedEncodeContinuation{receipt:Some(receipt),allocate,retirement}),Err((error,mut inner))=>{inner.allocation=NativeEncodeAllocationBinding::Forwarded(allocate);inner.retirement=retirement;Err((error,Self{inner}))}}}
 /// ⏳️ Rebinds only progress while the original allocation port remains unchanged.
 pub fn resume(mut receipt:NativeForwardedEncodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,(ValueError,NativeForwardedEncodeContinuation<'a>)>{if let Err(error)=receipt.validate(){return Err((error,receipt))}let mut inner=NativeEncodeControl::resume(receipt.receipt.take().expect("validated original forwarded receipt"),callback).expect("validated original forwarded accounting");inner.allocation=NativeEncodeAllocationBinding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
struct ForwardedEncodeReceiptScope<'owner,'control>{control:NativeEncodeControl<'control>,receipt:&'owner mut Option<NativeEncodeContinuation>}
impl Drop for ForwardedEncodeReceiptScope<'_,'_>{fn drop(&mut self){let control=&mut self.control;*self.receipt=Some(NativeEncodeContinuation{work:control.action.state.take(),receiving:control.receiving,maximum_bytes:control.maximum_bytes,ledger:control.ledger.take(),completed:control.completed,total:control.total,stage:control.stage});}}
impl<'a> NativeForwardedEncodeContinuation<'a>{
 fn validate(&self)->Result<(),ValueError>{let receipt=self.receipt.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation is not at a receiving boundary"))?;if receipt.work.has_live_actions(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native continuation requires its original live work loan"))}if (receipt.total!=0&&receipt.completed>receipt.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation has invalid original accounting"))}Ok(())}
 /// 🫴️ Returns the same cumulative receiving receipt while the owner keeps its original ports.
 pub fn detach(mut self)->Result<NativeForwardedEncodeReceipt,(ValueError,Self)>{
  if let Err(error)=self.validate(){return Err((error,self))}if self.receipt.as_ref().and_then(|receipt|receipt.ledger.as_ref()).is_none_or(|ledger|ledger.is_bound()){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation retains its paired ledger"),self))}let Some(recipient)=self.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan lacks its original retirement recipient"),self))};let recipient=recipient.identity;let allocation_identity=self.allocate.identity();let receipt=self.receipt.take().expect("validated original forwarded receipt");Ok(NativeForwardedEncodeReceipt{receipt,recipient,allocation_identity})
 }

 /// 🎟️ Retains the original allocation port before the first physical hop binds its progress observer.
 pub fn new(maximum_bytes:usize,allocate:&'a mut (dyn NativeSendAllocationLoan+'a))->Self{Self{receipt:Some(NativeEncodeContinuation{work:super::action::WorkState::fresh(),receiving:false,maximum_bytes,ledger:Some(crate::native_receiving::NativeAllocationLedger::new(0)),completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🔁️ Runs one hop while caller custody retains its receipt on return and unwind.
 pub fn encode<T,E>(&mut self,callback:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,semio_framework_value_call::NativeCallFailure<T,E>>{
  if let Err(error)=self.validate(){return Err(semio_framework_value_call::NativeCallFailure{output:None,operation:None,context:Some(error),receipt_refusal:None})}let receipt=self.receipt.take().expect("validated original forwarded receipt");let mut control=match NativeEncodeControl::resume(receipt,callback){Ok(control)=>control,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(semio_framework_value_call::NativeCallFailure{output:None,operation:None,context:Some(error),receipt_refusal:None})}};control.allocation=NativeEncodeAllocationBinding::Forwarded(&mut *self.allocate);control.retirement=self.retirement.as_deref_mut();let mut scope=ForwardedEncodeReceiptScope{control,receipt:&mut self.receipt};let result=scope.control.scoped_stage(operation);let context=scope.control.validate_continuation().err();match(result,context){(Ok(output),None)=>Ok(output),(Ok(output),Some(error))=>Err(semio_framework_value_call::NativeCallFailure{output:Some(output),operation:None,context:Some(error),receipt_refusal:None}),(Err(error),context)=>Err(semio_framework_value_call::NativeCallFailure{output:None,operation:Some(error),context,receipt_refusal:None})}
 }
 /// 🧷️ Installs the same caller return slot for every subsequent physical hop.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded encoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 📬️ Returns the same complete receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").maximum_bytes}
 /// 📩️ Returns ownership admitted by the original receiving port through every hop.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").owned_bytes()}
}
impl<'a> std::ops::Deref for NativeForwardedEncodeControl<'a>{type Target=NativeEncodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedEncodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🔒️receipt/🦀️.rs"]
mod receipt_integrity_tests;

/// 🚦️ Observes an independent original admission callback boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeAdmissionPhase{Absent,NotCalled,Started,Accepted,Refused}
/// 🧭️ Retains both original budget admission verdicts and their issuers.
pub struct NativeEncodeAdmissionProgress{request:NativeAllocationRequest,forwarded_identity:Option<crate::NativeAllocationIdentity>,scoped_identity:Option<crate::NativeAllocationIdentity>,forwarded:NativeEncodeAdmissionPhase,scoped:NativeEncodeAdmissionPhase,forwarded_refusal:Option<crate::NativeAllocationRefusal>,scoped_refusal:Option<crate::NativeAllocationRefusal>,forwarded_transaction:Option<crate::NativeAllocationTransaction>,scoped_transaction:Option<crate::NativeAllocationTransaction>,forwarded_verdict:Option<Result<(),crate::NativeAllocationRefusal>>,scoped_verdict:Option<Result<(),crate::NativeAllocationRefusal>>}
impl NativeEncodeAdmissionProgress{
 /// 📏️ Borrows the unchanged admission request.
 pub fn request(&self)->NativeAllocationRequest{self.request}
 /// 🚦️ Reports the forwarded callback boundary.
 pub fn forwarded_phase(&self)->NativeEncodeAdmissionPhase{self.forwarded}
 /// 🚦️ Reports the scoped callback boundary.
 pub fn scoped_phase(&self)->NativeEncodeAdmissionPhase{self.scoped}
}
impl Drop for NativeEncodeAdmissionProgress{fn drop(&mut self){assert!(std::thread::panicking()||!matches!(self.forwarded,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled)&&!matches!(self.scoped,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled)&&self.forwarded_transaction.is_none()&&self.scoped_transaction.is_none()&&self.forwarded_verdict.is_none()&&self.scoped_verdict.is_none(),"original admission callbacks remain unsettled")}}
impl crate::retirement::RetireOwned for NativeEncodeAdmissionProgress{
 type RetirementCursor=crate::native_receiving::NativeRetirementGate<Self,<(Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>) as crate::retirement::RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{!(matches!(self.forwarded,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled)||matches!(self.scoped,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled))&&self.forwarded_transaction.is_none()&&self.scoped_transaction.is_none()&&self.forwarded_verdict.is_none()&&self.scoped_verdict.is_none()&&crate::retirement::RetireOwned::retirement_ready(&self.forwarded_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.scoped_refusal)}
 fn retirement_seed(mut self)->Self::RetirementCursor{if !crate::retirement::RetireOwned::retirement_ready(&self){return crate::native_receiving::NativeRetirementGate::Pending(self)}let causes=(self.forwarded_refusal.take(),self.scoped_refusal.take());crate::native_receiving::NativeRetirementGate::Ready(crate::retirement::RetireOwned::retirement_seed(causes))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
fn drive_encode_admission(port:&mut dyn NativeSendAllocationLoan,request:NativeAllocationRequest,phase:&mut NativeEncodeAdmissionPhase,transaction:&mut Option<crate::NativeAllocationTransaction>,verdict:&mut Option<Result<(),crate::NativeAllocationRefusal>>,refusal:&mut Option<crate::NativeAllocationRefusal>,ledger:&mut crate::native_receiving::NativeAllocationLedger)->Result<bool,ValueError>{use crate::NativeAllocationTransactionPhase as Phase;if *phase==NativeEncodeAdmissionPhase::NotCalled{*phase=NativeEncodeAdmissionPhase::Started}if transaction.is_none(){port.begin(request,false,transaction)?}else{let state=transaction.as_mut().expect("original admission transaction");if state.request()!=request{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission request changed"))}if state.phase()==Phase::AdmitStarted&&!port.recover(state)?{return Ok(false)}}let state=transaction.as_mut().expect("original admission transaction");if state.phase()==Phase::AdmitStarted{return Ok(false)}if verdict.is_none(){*verdict=state.take_verdict()}if verdict.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission verdict is absent"))}if matches!(verdict.as_ref(),Some(Ok(()))){commit_admission(ledger,request)?}port.finish(transaction)?;match verdict.take().expect("original admission verdict retained"){Ok(())=>*phase=NativeEncodeAdmissionPhase::Accepted,Err(error)=>{*phase=NativeEncodeAdmissionPhase::Refused;*refusal=Some(error)}}Ok(true)}
impl NativeEncodeControl<'_>{
 fn admission_identity(&self)->(Option<crate::NativeAllocationIdentity>,Option<crate::NativeAllocationIdentity>){(match &self.allocation{NativeEncodeAllocationBinding::Forwarded(port)=>Some(port.identity()),NativeEncodeAllocationBinding::Local=>None},self.scoped_allocation.as_ref().map(|port|port.identity()))}
 fn admission_valid(&self,state:&NativeEncodeAdmissionProgress)->bool{let accepted=state.forwarded==NativeEncodeAdmissionPhase::Accepted||state.scoped==NativeEncodeAdmissionPhase::Accepted||matches!(state.forwarded_verdict.as_ref(),Some(Ok(())))||matches!(state.scoped_verdict.as_ref(),Some(Ok(())));self.admission_identity()==(state.forwarded_identity,state.scoped_identity)&&self.maximum_bytes==state.request.maximum_bytes&&self.owned_bytes()==if accepted{state.request.next_owned_bytes}else{state.request.owned_bytes}}
 /// 🎟️ Creates the original budget admission frame before any authority callback.
 pub fn begin_admission(&mut self,bytes:usize,slot:&mut Option<NativeEncodeAdmissionProgress>)->Result<(),ValueError>{if slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native admission requires an empty progress slot"))}let next=self.owned_bytes().checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native admission exceeds original ceiling"))?;self.checkpoint()?;let request=NativeAllocationRequest{bytes,owned_bytes:self.owned_bytes(),next_owned_bytes:next,maximum_bytes:self.maximum_bytes};let(forwarded_identity,scoped_identity)=self.admission_identity();*slot=Some(NativeEncodeAdmissionProgress{request,forwarded_identity,scoped_identity,forwarded:if forwarded_identity.is_some(){NativeEncodeAdmissionPhase::NotCalled}else{NativeEncodeAdmissionPhase::Absent},scoped:if scoped_identity.is_some(){NativeEncodeAdmissionPhase::NotCalled}else{NativeEncodeAdmissionPhase::Absent},forwarded_refusal:None,scoped_refusal:None,forwarded_transaction:None,scoped_transaction:None,forwarded_verdict:None,scoped_verdict:None});self.continue_admission(slot.as_mut().expect("admission progress present"))}
 /// ▶️ Drives each original authority, recovering only its own recorded callback verdict.
 pub fn continue_admission(&mut self,state:&mut NativeEncodeAdmissionProgress)->Result<(),ValueError>{if !self.admission_valid(state){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"admission requires its original authority and ledger"))}if matches!(state.forwarded,NativeEncodeAdmissionPhase::NotCalled|NativeEncodeAdmissionPhase::Started){let NativeEncodeAllocationBinding::Forwarded(port)=&mut self.allocation else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded admission owner is absent"))};if !drive_encode_admission(&mut **port,state.request,&mut state.forwarded,&mut state.forwarded_transaction,&mut state.forwarded_verdict,&mut state.forwarded_refusal,self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger is lent"))?)?{return Ok(())}}if matches!(state.scoped,NativeEncodeAdmissionPhase::NotCalled|NativeEncodeAdmissionPhase::Started){let port=self.scoped_allocation.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped admission owner is absent"))?;if !drive_encode_admission(&mut **port,state.request,&mut state.scoped,&mut state.scoped_transaction,&mut state.scoped_verdict,&mut state.scoped_refusal,self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger is lent"))?)?{return Ok(())}}Ok(())}
 /// 🧾️ Transfers every refusal after both original admission callbacks settle.
 pub fn finish_admission(&mut self,slot:&mut Option<NativeEncodeAdmissionProgress>)->Result<(),crate::NativeAllocationFailure>{let Some(state)=slot.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native admission progress is absent").into())};if !self.admission_valid(state)||matches!(state.forwarded,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled)||matches!(state.scoped,NativeEncodeAdmissionPhase::Started|NativeEncodeAdmissionPhase::NotCalled)||state.forwarded_transaction.is_some()||state.scoped_transaction.is_some()||state.forwarded_verdict.is_some()||state.scoped_verdict.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"admission requires its original settled authorities").into())}let mut state=slot.take().expect("admission progress present");let local=state.forwarded==NativeEncodeAdmissionPhase::Absent&&state.scoped==NativeEncodeAdmissionPhase::Absent;let accepted=local||state.forwarded==NativeEncodeAdmissionPhase::Accepted||state.scoped==NativeEncodeAdmissionPhase::Accepted;if local{commit_admission(self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original encode ledger is lent"))?,state.request)?}let mut failure=crate::NativeAllocationFailure::default();failure.forwarded_admission=state.forwarded_refusal.take();failure.scoped_admission=state.scoped_refusal.take();failure.progress=crate::NativeAllocationProgress{prior_admitted_bytes:Some(state.request.owned_bytes),requested_bytes:Some(state.request.bytes),maximum_bytes:Some(state.request.maximum_bytes),admitted_bytes:if accepted{state.request.next_owned_bytes}else{state.request.owned_bytes},actual_capacity_bytes:None};if failure.is_empty(){Ok(())}else{Err(failure)}}
}

#[cfg(test)]
#[path="🧪️tests/🪢️unwind.rs"]
mod unwind_tests;

impl NativeEncodeControl<'_>{
 /// 🧳️ Places a completed original allocation outcome in the caller-owned slot before later callbacks.
 pub fn allocate_effect_into(&mut self,bytes:usize,actual:&mut Option<usize>,progress:&mut Option<NativeEncodeAllocationProgress>,outcome:&mut Option<NativeEncodeAllocationOutcome>,operation:&mut impl FnMut(&mut Option<usize>,&mut semio_framework_value_control_contract::OriginalWorkTurn<'_,'_>)->Result<bool,ValueError>)->Result<bool,ValueError>{if outcome.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native allocation outcome slot is occupied"))}*outcome=self.allocate_effect(bytes,actual,progress,operation)?;Ok(outcome.is_some())}
}

