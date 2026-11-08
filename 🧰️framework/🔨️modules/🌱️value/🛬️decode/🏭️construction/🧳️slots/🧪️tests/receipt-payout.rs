//! 🧾️ Resident same-Source payout for a refused constructor receipt.
use super::*;
pub(super) trait Target{
 fn finish_child(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>;
 fn stage(&mut self,recipient:&OriginalConstructionLoan<'_>,payout:&OriginalConstructionAction,completion:&mut Option<OriginalConstructionCompletion>,effect:&mut RetainedAllocationEffect<'_>,published:bool)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>;
 fn publish(&mut self,control:&mut NativeDecodeControl<'_>,payout:&OriginalConstructionAction,proof:&OriginalWorkActionProof)->Result<bool,ValueError>;
}
impl<T:RetireOwned+Sync,R:RetireOwned+Sync> Target for NativeSharedSlots<T,R>{
 fn finish_child(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.finish_refused_receipt_work(control)}
 fn stage(&mut self,recipient:&OriginalConstructionLoan<'_>,payout:&OriginalConstructionAction,completion:&mut Option<OriginalConstructionCompletion>,effect:&mut RetainedAllocationEffect<'_>,published:bool)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{if published{self.stage_published_receipt_with_loan(recipient,completion,payout,effect)}else{self.stage_refused_receipt_with_loan(recipient,payout,completion,effect)}}
 fn publish(&mut self,control:&mut NativeDecodeControl<'_>,payout:&OriginalConstructionAction,proof:&OriginalWorkActionProof)->Result<bool,ValueError>{self.publish_receipt(control,payout,proof)}
}
impl Target for NativeReceivingEffect<OriginalRetirementFailure>{
 fn finish_child(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.finish_child_work_for_payout(control)}
 fn stage(&mut self,recipient:&OriginalConstructionLoan<'_>,payout:&OriginalConstructionAction,completion:&mut Option<OriginalConstructionCompletion>,effect:&mut RetainedAllocationEffect<'_>,published:bool)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{if published{self.stage_published_receipt_with_loan(recipient,completion,payout,effect)}else{self.stage_receipt_completion_with_loan(recipient,payout,completion,effect)}}
 fn publish(&mut self,control:&mut NativeDecodeControl<'_>,payout:&OriginalConstructionAction,proof:&OriginalWorkActionProof)->Result<bool,ValueError>{self.publish_receipt_completion(control,payout,proof)}
}
#[derive(Default)]
pub(super) struct ReceiptPayout{stage:u8,child_finished:bool,physical:Option<OriginalConstructionAction>,work:Option<OriginalWorkActionIdentity>,receipt:Option<OriginalWorkPerformedReceipt>,claim:Option<Result<u128,ValueError>>,partial:Option<OriginalWorkPartialOutcome>,proof:Option<OriginalWorkActionProof>,actual:Option<usize>,progress:Option<RetainedCloneProgress>,completion:Option<OriginalConstructionCompletion>}
impl ReceiptPayout{
 pub(super) fn settle<T:Target>(&mut self,bank:&mut Bank,native:&mut NativeDecodeControl<'_>,target:&mut T)->Result<bool,ValueError>{
  if self.partial.as_ref().and_then(OriginalWorkPartialOutcome::source_refusal).is_some(){return Ok(false)}if !self.child_finished{bank.run(native,|native|target.finish_child(native))?;self.child_finished=true}
  while self.stage<2{let demand=if self.stage==0{OriginalConstructionCompletion::demand()}else{semio_framework_value_native_decoding::OriginalReceivingPrimary::receipt_completion_demand()};
   let ready=bank.run(native,|native|native.with_original_work_and_construction(|work,physical|{
    if self.physical.is_none(){physical.begin(demand,&mut self.physical,work)?}
    let action=self.physical.as_mut().unwrap();if action.phase()==OriginalConstructionPhase::Fresh{physical.quote(action)?}if action.phase()==OriginalConstructionPhase::GrantStarted{match physical.recover_grant(action)?{None=>return Ok(false),Some(Err(error))=>return Err(error),Some(Ok(_))=>{}}}
    if self.work.is_none(){match work.prepare(1,&mut self.work)?{Some(true)=>{},Some(false)=>return Ok(false),None=>return Ok(false)}}
    if self.progress.is_none(){let grant=action.grant().ok_or_else(||ValueError::invariant("original payout grant absent"))?;let mut scope=work.performed_scope(self.work,&mut self.receipt,&mut self.claim)?;let mut effect=scope.effect(grant,1,&mut self.actual)?;self.progress=target.stage(physical,action,&mut self.completion,&mut effect,self.stage==1).result?;if self.progress.is_none(){return Ok(false)}}
    if let Some(claim)=self.claim.take(){claim?;}if work.partial_pending(self.work)?{if !work.settle_partial_into(self.work,&mut self.partial)?{return Ok(false)}if self.partial.as_ref().and_then(OriginalWorkPartialOutcome::source_refusal).is_some(){return Ok(false)}drop(self.partial.take())}if self.proof.is_none(){let total=work.performed_total(self.work)?;if !work.commit(total,self.work,&mut self.proof)?{return Ok(false)}}
    if action.phase()==OriginalConstructionPhase::Granted{physical.post(action,self.progress.unwrap(),self.actual)?}if action.phase()==OriginalConstructionPhase::AcceptStarted&&physical.recover_accept(action)?.is_none(){return Ok(false)}if action.recorded_accept_status()!=Some(OriginalConstructionAcceptStatus::Accepted){return Ok(false)}if self.stage==0&&!physical.publish_completion(self.completion.as_mut().unwrap(),action,work,self.proof.as_ref().unwrap())?{return Ok(false)}Ok(true)
   }))?;if !ready{return Ok(false)}if self.stage==1&&!bank.run(native,|native|target.publish(native,self.physical.as_ref().unwrap(),self.proof.as_ref().unwrap()))?{return Ok(false)}
   bank.run(native,|native|native.with_original_work_and_construction(|work,physical|{work.finish(&mut self.proof,&mut self.work)?;physical.finish_accepted(&mut self.physical)}))?;self.progress=None;self.actual=None;self.stage+=1;
  }Ok(true)
 }
}

/// ♻️ Keeps the exact receiving owner borrowed while its original receipt is released.
pub(super) struct ReceiptRelease<'a>{target:&'a mut semio_framework_value_native_decoding::OriginalReceivingPrimary,payout:ReceiptPayout}
impl<'a> ReceiptRelease<'a>{
 pub(super) fn new(target:&'a mut semio_framework_value_native_decoding::OriginalReceivingPrimary)->Self{Self{target,payout:Default::default()}}
 pub(super) fn settle(&mut self,bank:&mut Bank,native:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{let demand=self.target.receipt_completion_release_demand()?.ok_or_else(||ValueError::invariant("original receipt completion is absent"))?;let p=&mut self.payout;let ready=bank.run(native,|native|native.with_original_work_and_construction(|work,physical|{if p.physical.is_none(){physical.begin(demand,&mut p.physical,work)?}let action=p.physical.as_mut().unwrap();if action.phase()==OriginalConstructionPhase::Fresh{physical.quote(action)?}if action.phase()==OriginalConstructionPhase::GrantStarted{match physical.recover_grant(action)?{None=>return Ok(false),Some(Err(error))=>return Err(error),Some(Ok(_))=>{}}}if p.work.is_none(){match work.prepare(1,&mut p.work)?{Some(true)=>{},_=>return Ok(false)}}if p.progress.is_none(){let grant=action.grant().ok_or_else(||ValueError::invariant("receipt release grant absent"))?;let mut scope=work.performed_scope(p.work,&mut p.receipt,&mut p.claim)?;let mut effect=scope.effect(grant,1,&mut p.actual)?;p.progress=self.target.retire_receipt_completion(physical,action,&mut effect).result?;if p.progress.is_none(){return Ok(false)}}if let Some(claim)=p.claim.take(){claim?;}if work.partial_pending(p.work)?{if !work.settle_partial_into(p.work,&mut p.partial)?{return Ok(false)}if p.partial.as_ref().and_then(OriginalWorkPartialOutcome::source_refusal).is_some(){return Ok(false)}drop(p.partial.take())}if p.proof.is_none(){let total=work.performed_total(p.work)?;if !work.commit(total,p.work,&mut p.proof)?{return Ok(false)}}if action.phase()==OriginalConstructionPhase::Granted{physical.post(action,p.progress.unwrap(),p.actual)?}if action.phase()==OriginalConstructionPhase::AcceptStarted&&physical.recover_accept(action)?.is_none(){return Ok(false)}if action.recorded_accept_status()!=Some(OriginalConstructionAcceptStatus::Accepted){return Ok(false)}Ok(true)}))?;if !ready{return Ok(false)}bank.run(native,|native|native.with_original_work_and_construction(|work,physical|{work.finish(&mut p.proof,&mut p.work)?;physical.finish_accepted(&mut p.physical)}))?;Ok(true)}
}
