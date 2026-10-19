//! 🛂️ Member History retains one original decoding ledger and reborrows each actual caller turn.
use semio_framework_os_spr::history::{HistoryLog, RetainedHistorySourceDecode, RetainedHistorySourcePartial, RetainedHistorySourceStep, RetainedHistoryOpMetaFault};
use protocol::codec::{HistoryReadLimits,RetainedWireByteSource};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeRetirementRecipient};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeAllocation, native_decoding::NativeForwardedDecodeControl};
use semio_framework_value_native_owned::native_decoding::allocation::NativeForwardedDecodeReceipt;
use std::{mem::ManuallyDrop,cell::UnsafeCell,sync::atomic::{AtomicBool,Ordering}};

/// 🪢️ One stack loan shares the actual initiating caller only through an exclusive borrowed guard.
struct BorrowedOriginalTurn<'a,'source>{pub actual:UnsafeCell<&'a mut StepContext<'source>>,reserved:UnsafeCell<usize>,pub borrowed:AtomicBool}
/// 🛡️ The atomic gate excludes every mutable access, and the actual context must support thread transfer.
unsafe impl<'a,'source> Sync for BorrowedOriginalTurn<'a,'source> where StepContext<'source>:Send{}
struct OriginalTurnGuard<'a>(&'a AtomicBool);
impl Drop for OriginalTurnGuard<'_>{fn drop(&mut self){self.0.store(false,Ordering::Release)}}
impl<'a,'source> BorrowedOriginalTurn<'a,'source>{
 pub fn new(actual:&'a mut StepContext<'source>)->Self{Self{actual:UnsafeCell::new(actual),reserved:UnsafeCell::new(0),borrowed:AtomicBool::new(false)}}
 pub fn with<R>(&self,run:impl FnOnce(&mut StepContext<'source>,&mut usize)->R)->Result<R,ValueError>{self.borrowed.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).map_err(|_|invariant("original Member caller is already borrowed"))?;let guard=OriginalTurnGuard(&self.borrowed);let result=unsafe{run(&mut **self.actual.get(),&mut *self.reserved.get())};drop(guard);Ok(result)}
}

pub struct MemberHistoryReceiving {
 pub receipt:Option<NativeForwardedDecodeReceipt>,
 pub recipient:NativeDecodeRetirementRecipient,
 pub decoder:ManuallyDrop<Option<RetainedHistorySourceDecode>>,
 partial:ManuallyDrop<Option<RetainedHistorySourcePartial>>,
 pub fault:ManuallyDrop<Option<RetainedHistoryOpMetaFault>>,
 inner_fault:ManuallyDrop<Option<RetainedHistoryOpMetaFault>>,
 pub active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
 pub demand:RetirementDemand,
 pub maximum:usize,
 pub range:(usize,usize),
 pub limits:HistoryReadLimits,
 pub admitted:bool,
 pub closing:bool,
 pub closed:bool,
}

fn invariant(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}

fn with_original_control<T>(receipt:&mut Option<NativeForwardedDecodeReceipt>,recipient:&mut NativeDecodeRetirementRecipient,maximum:usize,cx:&mut StepContext<'_>,closing:bool,run:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->Result<T,OriginalTurnFailure>{
 let borrowed=BorrowedOriginalTurn::new(&mut *cx);
 let mut progress=|_|borrowed.with(|actual,_|!actual.should_yield()&&(closing||!actual.is_cancelled())).unwrap_or(false);
 let mut allocate=|quote:NativeDecodeAllocation|borrowed.with(|actual,reserved|{if actual.should_yield()||(!closing&&actual.is_cancelled()){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original Member decode caller yielded"))}let next=reserved.checked_add(quote.bytes).filter(|bytes|*bytes<=actual.retained_grant().maximum_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Member decode reservation exceeds actual turn"))?;*reserved=next;Ok(())}).and_then(|result|result);
 let mut control=if let Some(original)=receipt.take(){match NativeForwardedDecodeControl::rebind(original,&mut progress,&mut allocate,recipient){Ok(control)=>control,Err((error,original))=>{*receipt=Some(original);return Err(error.into())}}}else{let mut control=NativeDecodeControl::new_forwarded(maximum,&mut progress,&mut allocate);control.install_retirement_recipient(recipient)?;control};
 let result=control.scoped_stage(|original|Ok::<_,std::convert::Infallible>(run(original))).unwrap();
 *receipt=Some(control.detach().unwrap_or_else(|_|panic!("original Member decode continuation violated canonical control invariants")));
 Ok(result)
}

fn receive(cx:&mut StepContext<'_>,issued:Option<(RetainedCloneGrant,RetainedCloneProgress)>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let Some((original,progress))=issued else{return Ok(Default::default())};
 let raw=if original==grant{Ok(RetainedCloneStep::Progress(progress))}else{Err(invariant("original Member History cursor changed its supplied grant").with_retained_progress(progress))};
 crate::receive_member_close(cx,raw).map(|step|step.progress())
}

fn receive_failure(cx:&mut StepContext<'_>,issued:Option<(RetainedCloneGrant,RetainedCloneProgress)>,grant:RetainedCloneGrant,primary:ValueError)->OriginalTurnFailure{
 let progress=issued.map_or_else(||primary.retained_progress(),|(_,progress)|progress);
 let mut failure=crate::receive_member_close(cx,Err(primary.with_retained_progress(progress))).unwrap_err();
 if issued.is_some_and(|(original,_)|original!=grant)&&failure.context_refusal.is_none(){failure.context_refusal=Some(invariant("original Member History failure changed its supplied grant"))}
 failure
}

impl MemberHistoryReceiving {
 pub fn new(range:(usize,usize),limits:HistoryReadLimits,maximum:usize)->Self{Self{receipt:None,recipient:NativeDecodeRetirementRecipient::new(),decoder:ManuallyDrop::new(None),partial:ManuallyDrop::new(None),fault:ManuallyDrop::new(None),inner_fault:ManuallyDrop::new(None),active:ManuallyDrop::new(None),demand:RetirementDemand{depth:2,..Default::default()},maximum,range,limits,admitted:false,closing:false,closed:false}}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.decoder.is_none()&&self.partial.is_none()&&self.fault.is_none()&&self.inner_fault.is_none()&&self.active.is_none()&&self.recipient.terminal_is_empty()&&self.receipt.is_none()}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.active.as_ref(){return semio_framework_os_store_read::artifact_retirement_box_demands(active,body)}
  if self.fault.is_some(){return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.fault)}
  if self.inner_fault.is_some(){return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.inner_fault)}
  if !self.closing&&!self.recipient.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.recipient.next_copy_byte_demand()?,capacity_bytes:self.recipient.next_capacity_byte_demand(body)?,release_bytes:self.recipient.next_release_byte_demand()?,depth:self.recipient.next_depth_demand()?})}
  if self.decoder.is_some(){return Ok(RetirementDemand{depth:if self.closing{self.demand.depth.max(2)}else{self.demand.depth.max(1)},..self.demand})}
  if self.partial.is_some(){return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.partial)}
  if !self.recipient.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.recipient.next_copy_byte_demand()?,capacity_bytes:self.recipient.next_capacity_byte_demand(body)?,release_bytes:self.recipient.next_release_byte_demand()?,depth:self.recipient.next_depth_demand()?})}
  Ok(RetirementDemand{depth:if self.closed{0}else if !self.admitted&&!self.closing{2}else{1},..Default::default()})
 }
 pub fn step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,cx:&mut StepContext<'_>,grant:RetainedCloneGrant,history:&mut Option<HistoryLog>)->Result<RetainedHistorySourceStep,OriginalTurnFailure>{
  if self.closed{return Ok(RetainedHistorySourceStep::Closed)}
  if self.closing||self.fault.is_some(){return Ok(RetainedHistorySourceStep::Fault)}
  if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0{return Ok(RetainedHistorySourceStep::Blocked)}
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedHistorySourceStep::Blocked)}
  let decoder=&mut self.decoder;let admitted=&mut self.admitted;let demand=&mut self.demand;let range=self.range;let limits=self.limits;
  let(result,issued)=with_original_control(&mut self.receipt,&mut self.recipient,self.maximum,cx,false,|control|{
   if control.has_retirement_owner(){let raw=control.close_retirement_recipient(grant);let progress=match &raw{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};return(raw.map(|_|RetainedHistorySourceStep::Progress).map_err(Into::into),Some((grant,progress)))}
   let result=if !*admitted{match RetainedHistorySourceDecode::admit(source,range,limits,true,control,grant){Ok(Some(owner))=>{**decoder=Some(owner);*admitted=true;Ok(RetainedHistorySourceStep::Progress)},Ok(None)=>Ok(RetainedHistorySourceStep::Blocked),Err(error)=>Err(error)}}else if let Some(owner)=decoder.as_mut(){if owner.terminal_is_empty(){decoder.take();Ok(RetainedHistorySourceStep::Closed)}else if owner.is_ready(){owner.take_ready_into(history,grant).map(|ready|if ready{RetainedHistorySourceStep::Ready}else{RetainedHistorySourceStep::Blocked})}else{owner.step(source,control,grant)}}else{Ok(RetainedHistorySourceStep::Ready)};
   let issued=decoder.as_mut().and_then(|owner|owner.take_receipt());
   if let Some(owner)=decoder.as_ref(){if let Ok(next)=owner.demands(source,control,grant.maximum_copy_bytes){*demand=next}}
   (result,issued)
  })?;
  let step=match result{Ok(step)=>step,Err(RetainedHistoryOpMetaFault::Value(primary))=>{self.closing=true;return Err(receive_failure(cx,issued,grant,primary))},Err(error)=>{*self.fault=Some(error);self.closing=true;RetainedHistorySourceStep::Fault}};
  let progress=receive(cx,issued,grant)?;
  if self.fault.is_some(){return Ok(RetainedHistorySourceStep::Fault)}
  if self.admitted&&self.decoder.is_none()&&self.recipient.terminal_is_empty()&&issued.is_none(){self.receipt.take();self.closed=true;crate::receive_member_close(cx,Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))?;return Ok(RetainedHistorySourceStep::Ready)}
  Ok(if progress.copied_items>0{RetainedHistorySourceStep::Progress}else{step})
 }
 pub fn close_step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  self.closing=true;
  if cx.should_yield()||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.active.is_some()||self.fault.is_some()||self.inner_fault.is_some()||self.decoder.is_none()&&self.partial.is_some(){
   let active=&mut self.active;let fault=&mut self.fault;let inner_fault=&mut self.inner_fault;let partial=&mut self.partial;
   let raw=with_original_control(&mut self.receipt,&mut self.recipient,self.maximum,cx,true,|control|{control.charge(demand.capacity_bytes)?;if active.is_some(){semio_framework_os_store_read::artifact_retirement_box_close_step(active,grant)}else if fault.is_some(){semio_framework_os_store_read::artifact_retirement_admit_owned(fault,active,grant)}else if inner_fault.is_some(){semio_framework_os_store_read::artifact_retirement_admit_owned(inner_fault,active,grant)}else{semio_framework_os_store_read::artifact_retirement_admit_owned(partial,active,grant)}})?;
   return crate::receive_member_close(cx,raw)
  }
  if self.decoder.is_some(){
   let decoder=&mut self.decoder;let partial=&mut self.partial;let fault=&mut self.inner_fault;let demand=&mut self.demand;
   let(result,issued)=with_original_control(&mut self.receipt,&mut self.recipient,self.maximum,cx,true,|control|{
    let owner=decoder.as_mut().unwrap();owner.cancel();
    let result=if owner.terminal_is_empty(){Ok(RetainedHistorySourceStep::Closed)}else if owner.fault().is_some(){owner.take_fault_into(fault,grant).map(|_|RetainedHistorySourceStep::Progress)}else if owner.partial_is_receivable(){owner.take_partial_into(partial,grant).map(|_|RetainedHistorySourceStep::Progress)}else{owner.close_step(source,control,grant)};
    let issued=owner.take_receipt();
    if let Ok(next)=owner.demands(source,control,grant.maximum_copy_bytes){*demand=next}
    (result,issued)
   })?;
   match result{Err(RetainedHistoryOpMetaFault::Value(primary))=>return Err(receive_failure(cx,issued,grant,primary)),Err(error)=>*self.fault=Some(error),Ok(_)=>{}}
   let progress=receive(cx,issued,grant)?;
   if self.decoder.as_ref().is_some_and(|owner|owner.terminal_is_empty()){self.decoder.take();if issued.is_none(){return crate::receive_member_close(cx,Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))}}
   return Ok(RetainedCloneStep::Progress(progress))
  }
  if !self.recipient.terminal_is_empty(){let raw=with_original_control(&mut self.receipt,&mut self.recipient,self.maximum,cx,true,|control|control.close_retirement_recipient(grant))?;return crate::receive_member_close(cx,raw)}
  self.receipt.take();self.closed=true;
  crate::receive_member_close(cx,Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})))
 }
}
impl Drop for MemberHistoryReceiving{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"Member History abandoned original decoding custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.decoder);ManuallyDrop::drop(&mut self.partial);ManuallyDrop::drop(&mut self.fault);ManuallyDrop::drop(&mut self.inner_fault);ManuallyDrop::drop(&mut self.active)}}}}


