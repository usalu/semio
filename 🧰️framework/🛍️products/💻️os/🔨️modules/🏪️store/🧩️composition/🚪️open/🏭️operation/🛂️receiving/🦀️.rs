//! 🛂️ Member History retains one original decoding ledger and reborrows each actual caller turn.
use semio_os_spr::history::{HistoryLog, RetainedHistorySourceDecode, RetainedHistorySourcePartial, RetainedHistorySourceStep, RetainedHistoryOpMetaFault};
use protocol::codec::{HistoryReadLimits,RetainedWireByteSource};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeRetirementRecipient};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeAllocation,NativeDecodeAllocationEvent,NativeDecodeAllocationIssuer,NativeForwardedDecodeControl};
use semio_framework_value_native_owned::native_decoding::allocation::{NativeForwardedDecodeReceipt,NativeDecodeAllocationBudget};
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


#[derive(Default)]
struct MemberHistoryAllocation{issuer:NativeDecodeAllocationIssuer,binding:Option<semio_framework_job::OriginalSourceBinding>,state:NativeDecodeAllocationBudget}
impl MemberHistoryAllocation{
 fn bind(&mut self,cx:&StepContext<'_>)->Result<(),ValueError>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invariant("Member decoding requires its original source and clock"))}
  if self.binding.is_none(){self.binding=Some(semio_framework_job::OriginalSourceBinding::borrow(cx))}
  self.state.begin_hop();
  Ok(())
 }
}

pub struct MemberHistoryReceiving {
 allocation:MemberHistoryAllocation,
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


struct OriginalMemberWork<'a,'turn,'source>{turn:&'a BorrowedOriginalTurn<'turn,'source>,closing:bool}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OriginalMemberWork<'_,'_,'_>{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))}
}
fn paused(cx:&StepContext<'_>,closing:bool)->bool{if closing{cx.retirement_should_yield()}else{cx.should_yield()}}
fn with_original_control<T>(receipt:&mut Option<NativeForwardedDecodeReceipt>,recipient:&mut NativeDecodeRetirementRecipient,allocation:&mut MemberHistoryAllocation,maximum:usize,cx:&mut StepContext<'_>,closing:bool,run:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->Result<(T,u64),OriginalTurnFailure>{
 allocation.bind(cx)?;
 let state=&mut allocation.state;
 let borrowed=BorrowedOriginalTurn::new(&mut *cx);
 let mut progress=|_|borrowed.with(|actual,_|!paused(actual,closing)&&(closing||!actual.is_cancelled())).unwrap_or(false);
 let mut allocate=|event:NativeDecodeAllocationEvent|borrowed.with(|actual,_|{let may_quote=!matches!(event,NativeDecodeAllocationEvent::Quote(_))||!paused(actual,closing)&&(closing||!actual.is_cancelled());state.receive(event,actual.retained_grant(),may_quote)}).and_then(|result|result);
 let mut allocation_loan=allocation.issuer.loan(&mut allocate);
 let mut control=if let Some(original)=receipt.take(){match NativeForwardedDecodeControl::rebind(original,&mut progress,&mut allocation_loan,recipient){Ok(control)=>control,Err((error,original))=>{*receipt=Some(original);return Err(error.into())}}}else{let mut control=NativeDecodeControl::new_forwarded(maximum,&mut progress,&mut allocation_loan);control.install_retirement_recipient(recipient)?;control};
 let before=control.actual_work();let mut work=OriginalMemberWork{turn:&borrowed,closing};
 let result=control.with_work_recipient(&mut work,|native|Ok::<_,ValueError>(run(native)));
 let observed=control.actual_work().saturating_sub(before);
 *receipt=Some(control.detach().map_err(|_|invariant("original Member decode continuation violated canonical control invariants"))?);
 result.map(|value|(value,observed)).map_err(Into::into)
}
fn controlled_retirement(control:&mut NativeDecodeControl<'_>,quote:Result<u64,ValueError>,run:impl FnOnce(&mut NativeDecodeControl<'_>)->RetainedWorkOutcome)->(RetainedWorkOutcome,Option<ValueError>){
 let quote=match quote{Ok(quote)=>quote,Err(error)=>return(RetainedWorkOutcome{result:Err(error),work:0},None)};
 match control.prepare_work(quote){Ok(true)=>{},Ok(false)=>return(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},None),Err(error)=>return(RetainedWorkOutcome{result:Err(error),work:0},None)}
 let turn=run(control);let refusal=control.commit_work(turn.work).err();(turn,refusal)
}
fn history_progress(error:&RetainedHistoryOpMetaFault)->RetainedCloneProgress{match error{RetainedHistoryOpMetaFault::Value(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Native(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Retirement(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Json(_)=>Default::default()}}
fn receive<T,E>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,turn:RetainedWorkOutcome<T,E>,issued:Option<(RetainedCloneGrant,RetainedCloneProgress)>,observed:u64,error_progress:fn(&E)->RetainedCloneProgress)->semio_framework_job_native_decode::OriginalNativeDecodeOutcome<T,E>{
 let(original,progress)=issued.unwrap_or_else(||(grant,turn.result.as_ref().err().map_or(Default::default(),error_progress)));
 semio_framework_job_native_decode::OriginalNativeDecodeSource::receive_observed_work(cx,grant,fuel,semio_framework_job_native_decode::OriginalNativeDecodeIssued{value:turn.result,grant:original,progress,work:turn.work},observed)
}
fn metadata<T>(value:T)->(RetainedWorkOutcome<T,RetainedHistoryOpMetaFault>,RetainedCloneProgress){(RetainedWorkOutcome{result:Ok(value),work:1},RetainedCloneProgress{copied_items:1,..Default::default()})}

impl MemberHistoryReceiving {
 pub fn new(range:(usize,usize),limits:HistoryReadLimits,maximum:usize)->Self{Self{allocation:MemberHistoryAllocation::default(),receipt:None,recipient:NativeDecodeRetirementRecipient::new(),decoder:ManuallyDrop::new(None),partial:ManuallyDrop::new(None),fault:ManuallyDrop::new(None),inner_fault:ManuallyDrop::new(None),active:ManuallyDrop::new(None),demand:RetirementDemand{depth:2,..Default::default()},maximum,range,limits,admitted:false,closing:false,closed:false}}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.allocation.state.terminal_is_empty()&&self.decoder.is_none()&&self.partial.is_none()&&self.fault.is_none()&&self.inner_fault.is_none()&&self.active.is_none()&&self.recipient.terminal_is_empty()&&self.receipt.is_none()}
 fn failure_storage_needed(&self)->bool{!self.recipient.has_failure_storage()&&(self.active.is_some()||self.decoder.as_ref().is_some_and(|owner|!owner.terminal_is_empty())||self.recipient.has_owner())}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.failure_storage_needed(){return Ok(semio_framework_value::original_failure_storage_birth_demand())}
  if let Some(active)=self.active.as_ref(){return semio_os_store_read::artifact_retirement_box_demands(active,body)}
  if self.fault.is_some(){return semio_os_store_read::artifact_retirement_owned_birth_demands(&self.fault)}
  if self.inner_fault.is_some(){return semio_os_store_read::artifact_retirement_owned_birth_demands(&self.inner_fault)}
  if !self.closing&&self.recipient.has_owner(){return Ok(RetirementDemand{copy_bytes:self.recipient.next_copy_byte_demand()?,capacity_bytes:self.recipient.next_capacity_byte_demand(body)?,release_bytes:self.recipient.next_release_byte_demand()?,depth:self.recipient.next_depth_demand()?})}
  if self.decoder.is_some(){return Ok(RetirementDemand{depth:if self.closing{self.demand.depth.max(2)}else{self.demand.depth.max(1)},..self.demand})}
  if self.partial.is_some(){return semio_os_store_read::artifact_retirement_owned_birth_demands(&self.partial)}
  if !self.recipient.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.recipient.next_copy_byte_demand()?,capacity_bytes:self.recipient.next_capacity_byte_demand(body)?,release_bytes:self.recipient.next_release_byte_demand()?,depth:self.recipient.next_depth_demand()?})}
  Ok(RetirementDemand{depth:if self.closed{0}else if !self.admitted&&!self.closing{2}else{1},..Default::default()})
 }

 fn accept(&mut self,outcome:semio_framework_job_native_decode::OriginalNativeDecodeOutcome<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>)->Result<RetainedHistorySourceStep,OriginalTurnFailure>{
  let semio_framework_job_native_decode::OriginalNativeDecodeOutcome{value,progress,receiving_refusal,..}=outcome;
  match(value,receiving_refusal){
   (Ok(step),None)=>Ok(step),
   (Ok(_),Some(mut refusal))=>{self.closing=true;refusal.effects=progress;Err(refusal)},
   (Err(RetainedHistoryOpMetaFault::Value(primary)),None)=>{self.closing=true;Err(primary.with_retained_progress(progress).into())},
   (Err(RetainedHistoryOpMetaFault::Value(primary)),Some(OriginalTurnFailure{primary:semio_framework_job::OriginalTurnPrimary::Value(quote),receipt_refusal,context_refusal,work_refusal,..}))=>{
    self.closing=true;Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(semio_framework_job::OriginalReceivingPrimary{retirement_failure:None,work_refusal:None,primary:Some(primary),quote_refusal:Some(quote),allocation_refusal:None,allocation_failure:None,native_allocation:None,receipt_refusal:None,progress}),receipt_refusal,context_refusal,work_refusal})
   },
   (Err(primary),refusal)=>{*self.fault=Some(primary);self.closing=true;match refusal{Some(mut refusal)=>{refusal.effects=progress;Err(refusal)},None=>Ok(RetainedHistorySourceStep::Fault)}},
  }
 }
 /// 🧷️ Retains the complete retirement cause while shared settlement reports physical and computational effects once.
 fn receive_retirement(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,turn:semio_framework_value::OriginalRetirementOutcome,observed:u64)->Result<RetainedCloneStep,OriginalTurnFailure>{
  let physical=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
  let outcome=receive(cx,grant,fuel,turn,Some((grant,physical)),observed,semio_framework_value::OriginalRetirementFailure::retained_progress);
  let outcome=semio_framework_job_native_decode::OriginalNativeDecodeOutcome{value:outcome.value.map(|_|RetainedHistorySourceStep::Progress).map_err(Into::into),progress:outcome.progress,work:outcome.work,reported_work:outcome.reported_work,observed_native_work:outcome.observed_native_work,receiving_refusal:outcome.receiving_refusal};
  let progress=outcome.progress;self.accept(outcome)?;Ok(RetainedCloneStep::Progress(progress))
 }
 fn refill_failure_storage(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant,closing:bool)->Result<RetainedCloneStep,OriginalTurnFailure>{
  let fuel=cx.fuel_remaining();let(raw,observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,closing,|control|control.with_retirement(grant,|same|Ok::<_,ValueError>(semio_framework_value::refill_original_failure_storage(same,grant))))?;
  let turn=raw.unwrap_or_else(|error|semio_framework_value::OriginalRetirementOutcome{result:Err(error.into()),work:0});self.receive_retirement(cx,grant,fuel,turn,observed)
 }
 pub fn step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,cx:&mut StepContext<'_>,grant:RetainedCloneGrant,history:&mut Option<HistoryLog>)->Result<RetainedHistorySourceStep,OriginalTurnFailure>{
  if self.closed{return Ok(RetainedHistorySourceStep::Closed)}
  if self.closing||self.fault.is_some(){return Ok(RetainedHistorySourceStep::Fault)}
  if paused(cx,false)||cx.is_cancelled()||grant.maximum_items==0{return Ok(RetainedHistorySourceStep::Blocked)}
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedHistorySourceStep::Blocked)}
  if self.failure_storage_needed(){self.refill_failure_storage(cx,grant,false)?;return Ok(RetainedHistorySourceStep::Progress)}
  let finish_storage=self.admitted&&self.decoder.is_none()&&!self.recipient.terminal_is_empty();
  let fuel=cx.fuel_remaining();let decoder=&mut self.decoder;let admitted=&mut self.admitted;let demand=&mut self.demand;let range=self.range;let limits=self.limits;
  let((turn,issued),observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,false,|control|{
   if control.has_retirement_owner()||finish_storage{let raw=control.close_retirement_recipient(grant);let progress=match &raw.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};return(RetainedWorkOutcome{result:raw.result.map(|_|RetainedHistorySourceStep::Progress).map_err(Into::into),work:raw.work},Some((grant,progress)))}
   let mut own_receipt=None;
   let turn=if !*admitted{
    let raw=RetainedHistorySourceDecode::admit(source,range,limits,true,control,grant);
    RetainedWorkOutcome{result:raw.result.map(|owner|match owner{Some(owner)=>{**decoder=Some(owner);*admitted=true;RetainedHistorySourceStep::Progress},None=>RetainedHistorySourceStep::Blocked}),work:raw.work}
   }else if let Some(owner)=decoder.as_mut(){
    if owner.terminal_is_empty(){decoder.take();let(raw,progress)=metadata(RetainedHistorySourceStep::Closed);own_receipt=Some((grant,progress));raw}
    else if owner.is_ready(){let raw=owner.take_ready_into(history,grant);RetainedWorkOutcome{result:raw.result.map(|ready|if ready{RetainedHistorySourceStep::Ready}else{RetainedHistorySourceStep::Blocked}),work:raw.work}}
    else{owner.step(source,control,grant)}
   }else{RetainedWorkOutcome{result:Ok(RetainedHistorySourceStep::Ready),work:0}};
   let issued=own_receipt.or_else(||decoder.as_mut().and_then(|owner|owner.take_receipt()));
   if let Some(owner)=decoder.as_ref(){if let Ok(next)=owner.demands(source,control,grant.maximum_copy_bytes){*demand=next}}
   (turn,issued)
  })?;
  let outcome=receive(cx,grant,fuel,turn,issued,observed,history_progress);let progress=outcome.progress;
  let step=self.accept(outcome)?;
  if self.fault.is_some(){return Ok(RetainedHistorySourceStep::Fault)}
  if self.admitted&&self.decoder.is_none()&&self.recipient.terminal_is_empty()&&issued.is_none()&&!paused(cx,false){
   self.receipt.take();self.closed=true;let(raw,progress)=metadata(RetainedHistorySourceStep::Ready);let fuel=cx.fuel_remaining();let outcome=receive(cx,grant,fuel,raw,Some((grant,progress)),0,history_progress);return self.accept(outcome)
  }
  Ok(if progress.copied_items>0{RetainedHistorySourceStep::Progress}else{step})
 }
 pub fn close_step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  self.closing=true;
  if paused(cx,true)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.failure_storage_needed(){return self.refill_failure_storage(cx,grant,true)}
  let fuel=cx.fuel_remaining();
  if self.active.is_some(){
   let active=&mut self.active;
   let(raw,observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,true,|control|control.with_retirement(grant,|same|Ok::<_,ValueError>(semio_framework_value::close_factory_ticket_controlled(active,grant,same))))?;
   let turn=raw.unwrap_or_else(|error|semio_framework_value::OriginalRetirementOutcome{result:Err(error.into()),work:0});
   return self.receive_retirement(cx,grant,fuel,turn,observed)
  }
  if self.fault.is_some()||self.inner_fault.is_some()||self.decoder.is_none()&&self.partial.is_some(){
   let active=&mut self.active;let fault=&mut self.fault;let inner_fault=&mut self.inner_fault;let partial=&mut self.partial;
   let((raw,work_refusal),observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,true,|control|controlled_retirement(control,Ok(1),|control|{if let Err(error)=control.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}if fault.is_some(){semio_os_store_read::artifact_retirement_admit_owned(fault,active,grant)}else if inner_fault.is_some(){semio_os_store_read::artifact_retirement_admit_owned(inner_fault,active,grant)}else{semio_os_store_read::artifact_retirement_admit_owned(partial,active,grant)}}))?;
   let turn=semio_framework_value::settle_original_retirement_work(raw,work_refusal.map_or(Ok(()),Err));
   return self.receive_retirement(cx,grant,fuel,turn,observed)
  }
  if self.decoder.is_some(){
   let decoder=&mut self.decoder;let partial=&mut self.partial;let fault=&mut self.inner_fault;let demand=&mut self.demand;
   let((turn,issued),observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,true,|control|{
    let owner=decoder.as_mut().unwrap();owner.cancel();
    if owner.terminal_is_empty(){decoder.take();let(raw,progress)=metadata(RetainedHistorySourceStep::Closed);return(raw,Some((grant,progress)))}
    let turn=if owner.fault().is_some(){let raw=owner.take_fault_into(fault,grant);RetainedWorkOutcome{result:raw.result.map(|_|RetainedHistorySourceStep::Progress),work:raw.work}}
    else if owner.partial_is_receivable(){let raw=owner.take_partial_into(partial,grant);RetainedWorkOutcome{result:raw.result.map(|_|RetainedHistorySourceStep::Progress),work:raw.work}}
    else{owner.close_step(source,control,grant)};
    let issued=owner.take_receipt();if let Ok(next)=owner.demands(source,control,grant.maximum_copy_bytes){*demand=next}(turn,issued)
   })?;
   let outcome=receive(cx,grant,fuel,turn,issued,observed,history_progress);let progress=outcome.progress;self.accept(outcome)?;return Ok(RetainedCloneStep::Progress(progress))
  }
  if !self.recipient.terminal_is_empty(){
   let(raw,observed)=with_original_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,cx,true,|control|control.close_retirement_recipient(grant))?;
   return self.receive_retirement(cx,grant,fuel,raw,observed)
  }
  self.receipt.take();self.closed=true;let(raw,progress)=metadata(RetainedHistorySourceStep::Closed);let outcome=receive(cx,grant,fuel,raw,Some((grant,progress)),0,history_progress);self.accept(outcome)?;Ok(RetainedCloneStep::Complete(progress))
 }

}
impl Drop for MemberHistoryReceiving{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"Member History abandoned original decoding custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.decoder);ManuallyDrop::drop(&mut self.partial);ManuallyDrop::drop(&mut self.fault);ManuallyDrop::drop(&mut self.inner_fault);ManuallyDrop::drop(&mut self.active)}}}}



#[cfg(test)]
mod actual_work_laws{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
 #[test]
 fn original_member_work_binds_actual_budget_and_posts_independent_observations_once(){
  let(_,positive)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut bytes=Vec::<u8>::with_capacity(73);bytes.resize(73,0);std::hint::black_box(&bytes);drop(bytes);});assert_eq!(positive.requested_bytes,73);assert_eq!(positive.released_bytes,73);
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/member-work.json")).unwrap();
  for(index,row)in law["cases"].as_array().unwrap().iter().enumerate(){
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(870+index as u64),Generation(1),stock(),||Some(1),u64::MAX,1<<20);
   let mut receipt=None;let mut recipient=NativeDecodeRetirementRecipient::new();let mut allocation=MemberHistoryAllocation::default();
   caller.turn(stock(),|cx|{
    let deadline=cx.deadline_us();cx.with_original_work_loan(law["budget"].as_u64().unwrap(),deadline,|cx|{
     let before=cx.fuel_remaining();let mut effects=0;
     let(raw,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||with_original_control(&mut receipt,&mut recipient,&mut allocation,1<<20,cx,false,|control|{
      if control.prepare_work(row["request"].as_u64().unwrap())?{effects+=1;control.commit_work(1)?;}
      if row["primary"].as_bool().unwrap(){Err(invariant("original member primitive primary"))}else{Ok(())}
     }));
     let(value,observed)=raw?;assert_eq!(effects,row["effects"].as_u64().unwrap());assert_eq!(observed,row["observed"].as_u64().unwrap());assert_eq!(cx.fuel_remaining(),before);assert_eq!(heap.requested_bytes,0);
     let result=semio_framework_job_native_decode::OriginalNativeDecodeSource::receive_observed_work(cx,stock(),before,semio_framework_job_native_decode::OriginalNativeDecodeIssued{value,grant:stock(),progress:Default::default(),work:row["report"].as_u64().unwrap()},observed);
     assert_eq!(result.work,row["posted"].as_u64().unwrap());assert_eq!(before-cx.fuel_remaining(),result.work);assert_eq!(result.receiving_refusal.is_some(),row["refused"].as_bool().unwrap());assert_eq!(result.value.is_err(),row["primary"].as_bool().unwrap());
     if let Err(primary)=result.value{assert_eq!(primary.message,"original member primitive primary");}
     Ok(())
    })
   }).unwrap();
   assert!(recipient.terminal_is_empty());receipt.take();caller.retire(stock());
  }
 }
 #[test]
 fn original_member_retirement_preflights_real_birth_and_posts_actual_work_on_cleanup(){
  let(_,positive)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut bytes=Vec::<u8>::with_capacity(73);bytes.resize(73,0);std::hint::black_box(&bytes);drop(bytes);});assert_eq!(positive.requested_bytes,73);assert_eq!(positive.released_bytes,73);
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(899),Generation(1),stock(),||Some(1),u64::MAX,1<<20);
  let mut receipt=None;let mut recipient=NativeDecodeRetirementRecipient::new();let mut allocation=MemberHistoryAllocation::default();
  let mut owner=Some(RetainedHistoryOpMetaFault::Value(invariant("original Member retirement owner")));
  let mut active:Option<Box<dyn ErasedSnapshotRetirement>>=None;
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/member-work.json")).unwrap();assert_eq!(law["retirement"]["quote"],1);
  for row in law["retirement"]["cases"].as_array().unwrap(){
   let budget=row["budget"].as_u64().unwrap();let expected=row["work"].as_u64().unwrap();
   caller.turn(stock(),|cx|{let deadline=cx.deadline_us();cx.with_original_work_loan(budget,deadline,|cx|{
    let fuel=cx.fuel_remaining();let demand=semio_os_store_read::artifact_retirement_owned_birth_demands(&owner)?;
    let(raw,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||with_original_control(&mut receipt,&mut recipient,&mut allocation,1<<20,cx,true,|control|controlled_retirement(control,Ok(1),|control|{
     if let Err(error)=control.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}
     semio_os_store_read::artifact_retirement_admit_owned(&mut owner,&mut active,stock())
    })));
    let((turn,refusal),observed)=raw?;assert!(refusal.is_none());let progress=turn.result.as_ref().unwrap().progress();
    assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(observed,expected);assert_eq!(turn.work,expected);
    let outcome=receive(cx,stock(),fuel,turn,Some((stock(),progress)),observed,ValueError::retained_progress);assert!(outcome.value.is_ok());assert!(outcome.receiving_refusal.is_none());assert_eq!(fuel-cx.fuel_remaining(),expected);
    assert_eq!(owner.is_some(),row["owner"].as_bool().unwrap());assert_eq!(active.is_some(),row["active"].as_bool().unwrap());Ok(())
   })}).unwrap();
  }
  for _ in 0..256{
   if active.is_none()&&recipient.terminal_is_empty(){break}
   let refill=active.is_some()&&!recipient.has_failure_storage();
   caller.turn(stock(),|cx|{let deadline=cx.deadline_us();cx.with_original_work_loan(64,deadline,|cx|{
    let fuel=cx.fuel_remaining();
    let(raw,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||with_original_control(&mut receipt,&mut recipient,&mut allocation,1<<20,cx,true,|control|if refill{control.with_retirement(stock(),|same|Ok::<_,ValueError>(semio_framework_value::refill_original_failure_storage(same,stock())))}else if active.is_some(){control.with_retirement(stock(),|same|Ok::<_,ValueError>(semio_framework_value::close_factory_ticket_controlled(&mut active,stock(),same)))}else{Ok::<_,ValueError>(control.close_retirement_recipient(stock()))}));
    let(turn,observed)=raw?;let turn=turn?;assert_eq!(turn.work,observed);let progress=turn.result.as_ref().unwrap().progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(heap.requested_bytes,heap.released_bytes));
    let outcome=receive(cx,stock(),fuel,turn,Some((stock(),progress)),observed,semio_framework_value::OriginalRetirementFailure::retained_progress);assert!(outcome.value.is_ok());assert!(outcome.receiving_refusal.is_none());assert_eq!(fuel-cx.fuel_remaining(),observed);assert!(observed>0);Ok(())
   })}).unwrap();
  }
  assert!(owner.is_none()&&active.is_none()&&recipient.terminal_is_empty());receipt.take();caller.retire(stock());
 }

 #[test]
 fn member_allocation_events_preserve_pending_quotes_and_actual_physical_extent(){
  let corpus:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/member-allocation.json")).unwrap();
  for row in corpus["cases"].as_array().unwrap(){
   let maximum=row["maximum"].as_u64().unwrap()as usize;let bytes=row["bytes"].as_u64().unwrap()as usize;
   let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:maximum,maximum_depth:1,..Default::default()};
   let request=NativeDecodeAllocation{bytes,owned_bytes:0,next_owned_bytes:bytes,maximum_bytes:100};
   let mut state=NativeDecodeAllocationBudget::default();let quoted=state.receive(NativeDecodeAllocationEvent::Quote(request),grant,true);assert_eq!(quoted.is_err(),row["quoteRefused"].as_bool().unwrap());
   if quoted.is_ok(){let foreign=NativeDecodeAllocation{bytes:bytes+1,next_owned_bytes:bytes+1,..request};let event=match row["action"].as_str().unwrap(){"post"=>NativeDecodeAllocationEvent::Post{request,actual:row["actual"].as_u64().unwrap()as usize},"cancel"=>NativeDecodeAllocationEvent::Cancel(request),"foreign"=>NativeDecodeAllocationEvent::Post{request:foreign,actual:row["actual"].as_u64().unwrap()as usize},_=>NativeDecodeAllocationEvent::Quote(request)};assert_eq!(state.receive(event,grant,true).is_err(),row["settlementRefused"].as_bool().unwrap());}
   assert_eq!(state.spent(),row["spent"].as_u64().unwrap()as usize);assert_eq!(state.pending().is_some(),row["pending"].as_bool().unwrap());if let Some((request,grant))=state.pending(){state.receive(NativeDecodeAllocationEvent::Cancel(request),grant,true).unwrap();}
  }
 }

}
