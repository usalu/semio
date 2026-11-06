//! 🎟️ One original encoder action retains independent work, physical and allocation causes.
use super::{NativeEncodeAllocationOutcome,NativeEncodeAllocationProgress,NativeEncodeControl};
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
#[path="🧭️settlement/🦀️.rs"] mod settlement;
pub use settlement::{NativeEncodeEffectSettlement,NativeEncodeEffectPostPhase,NativeEncodeEffectReceipt,NativeEncodeEffectCancellation,NativeEncodeEffectRecovery};

/// 🧭️ Caller-retained action and allocation state survives a partial effect or unwind.
#[derive(Default)]
pub struct NativeEncodeEffectState{pub performed:bool,pub physical:RetainedCloneProgress,pub actual:Option<usize>,pub allocation:Option<NativeEncodeAllocationProgress>,pub outcome:Option<NativeEncodeAllocationOutcome>,pub settlement:Option<NativeEncodeEffectSettlement>}
/// 🔗️ Borrows the caller's existing effect and allocation slots without moving them.
pub struct NativeEncodeEffectProgress<'a>{pub performed:&'a mut bool,pub physical:&'a mut RetainedCloneProgress,pub actual:&'a mut Option<usize>,pub allocation:&'a mut Option<NativeEncodeAllocationProgress>,pub outcome:&'a mut Option<NativeEncodeAllocationOutcome>,pub settlement:&'a mut Option<NativeEncodeEffectSettlement>}
/// 🧭️ Describes a retained allocation outcome without moving its original causes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeAllocationOutcomeStatus{Absent,Canceled,Accepted,Refused}
/// 👁️ Inspects the caller-owned original allocation outcome.
pub fn native_encode_allocation_outcome_status(slot:&Option<NativeEncodeAllocationOutcome>)->NativeEncodeAllocationOutcomeStatus{let Some(outcome)=slot.as_ref()else{return NativeEncodeAllocationOutcomeStatus::Absent};if allocation_failed(outcome){NativeEncodeAllocationOutcomeStatus::Refused}else if outcome.admitted{NativeEncodeAllocationOutcomeStatus::Accepted}else{NativeEncodeAllocationOutcomeStatus::Canceled}}
/// 🔎️ Describes the retained original settlement without moving any refusal.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeEffectSettlementStatus{Absent,Pending,Accepted,Refused}
/// 👁️ Inspects a caller-owned settlement without moving its original causes.
pub fn native_encode_effect_settlement_status(slot:&Option<NativeEncodeEffectSettlement>)->NativeEncodeEffectSettlementStatus{
 let Some(state)=slot.as_ref()else{return NativeEncodeEffectSettlementStatus::Absent};if state.progress().is_none()||state.work_phase()!=NativeEncodeEffectPostPhase::Completed||state.physical_phase()!=NativeEncodeEffectPostPhase::Completed{return NativeEncodeEffectSettlementStatus::Pending}let(work,physical,invariant)=state.failures();if work.is_some()||physical.is_some()||invariant.is_some(){NativeEncodeEffectSettlementStatus::Refused}else{NativeEncodeEffectSettlementStatus::Accepted}
}
impl NativeEncodeEffectProgress<'_>{
 /// 📦️ Borrows the retained original allocation status.
 pub fn allocation_status(&self)->NativeEncodeAllocationOutcomeStatus{native_encode_allocation_outcome_status(&*self.outcome)}
 /// 👁️ Borrows the original settlement status without transferring its owned causes.
 pub fn settlement_status(&self)->NativeEncodeEffectSettlementStatus{native_encode_effect_settlement_status(&*self.settlement)}
}
impl NativeEncodeEffectState{
 /// 📦️ Borrows the retained original allocation status.
 pub fn allocation_status(&self)->NativeEncodeAllocationOutcomeStatus{native_encode_allocation_outcome_status(&self.outcome)}
 /// 👁️ Borrows the retained original settlement status.
 pub fn settlement_status(&self)->NativeEncodeEffectSettlementStatus{native_encode_effect_settlement_status(&self.settlement)}
 /// 👁️ Reborrows the same retained action and allocation slots.
 pub fn view(&mut self)->NativeEncodeEffectProgress<'_>{NativeEncodeEffectProgress{performed:&mut self.performed,physical:&mut self.physical,actual:&mut self.actual,allocation:&mut self.allocation,outcome:&mut self.outcome,settlement:&mut self.settlement}}
}
impl crate::retirement::RetireOwned for NativeEncodeEffectState{
 fn retirement_ready(&self)->bool{self.allocation.as_ref().is_none_or(crate::retirement::RetireOwned::retirement_ready)&&self.settlement.as_ref().is_none_or(crate::retirement::RetireOwned::retirement_ready)}
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.allocation),crate::retirement::deferred(self.outcome),crate::retirement::deferred(self.settlement)])}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.allocation),crate::retirement::deferred_birth_bytes_for(&self.outcome),crate::retirement::deferred_birth_bytes_for(&self.settlement)])}
 fn controlled_retirement_supported()->bool{true}
}
/// 🧳️ Every independent original refusal remains owned beside its peers.
#[derive(Default,Debug,PartialEq)]
pub struct NativeEncodeEffectFailure{pub primary:Option<ValueError>,pub invariant:Option<ValueError>,pub physical_invariant:Option<ValueError>,pub capacity_invariant:Option<ValueError>,pub work:Option<ValueError>,pub local:Option<ValueError>,pub physical:Option<ValueError>,pub allocation:Option<NativeEncodeAllocationOutcome>}
impl NativeEncodeEffectFailure{
 /// 🧾️ Identifies an empty successful failure carrier.
 pub fn is_empty(&self)->bool{self.primary.is_none()&&self.invariant.is_none()&&self.physical_invariant.is_none()&&self.capacity_invariant.is_none()&&self.work.is_none()&&self.local.is_none()&&self.physical.is_none()&&self.allocation.is_none()}
 fn primary(error:ValueError)->Self{Self{primary:Some(error),..Default::default()}}
}
impl crate::retirement::RetireOwned for NativeEncodeEffectFailure{
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{use crate::retirement::*;sequence(vec![deferred(self.primary),deferred(self.invariant),deferred(self.physical_invariant),deferred(self.capacity_invariant),deferred(self.work),deferred(self.local),deferred(self.physical),deferred(self.allocation)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use crate::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.primary),deferred_birth_bytes_for(&self.invariant),deferred_birth_bytes_for(&self.physical_invariant),deferred_birth_bytes_for(&self.capacity_invariant),deferred_birth_bytes_for(&self.work),deferred_birth_bytes_for(&self.local),deferred_birth_bytes_for(&self.physical),deferred_birth_bytes_for(&self.allocation)])}
 fn controlled_retirement_supported()->bool{true}
}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn remaining(grant:RetainedCloneGrant,used:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(used.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(used.released_bytes),maximum_depth:grant.maximum_depth}}
fn admits(grant:RetainedCloneGrant,demand:RetainedCloneGrant)->bool{demand.maximum_items<=grant.maximum_items&&demand.maximum_copy_bytes<=grant.maximum_copy_bytes&&demand.maximum_capacity_bytes<=grant.maximum_capacity_bytes&&demand.maximum_release_bytes<=grant.maximum_release_bytes&&demand.maximum_depth<=grant.maximum_depth}
fn allocation_failed(outcome:&NativeEncodeAllocationOutcome)->bool{outcome.primary.is_some()||outcome.forwarded_quote.is_some()||outcome.scoped_quote.is_some()||outcome.extent_refusal.is_some()||outcome.forwarded_refusal.is_some()||outcome.scoped_refusal.is_some()}
impl NativeEncodeControl<'_>{
 /// 🎟️ Quotes the same original physical and work recipients before running one caller-retained effect.
 pub fn effect_allocated(&mut self,grant:RetainedCloneGrant,local:&mut RetainedCloneProgress,demand:RetainedCloneGrant,bytes:Option<usize>,marker:&mut NativeEncodeEffectProgress<'_>,unwind:&mut Option<NativeEncodeEffectFailure>,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeEncodeEffectFailure>{
  let fail=|error|RetainedWorkOutcome{work:0,result:Err(NativeEncodeEffectFailure::primary(error))};
  if *marker.performed||*marker.physical!=RetainedCloneProgress::default()||marker.actual.is_some()||marker.allocation.is_some()||marker.outcome.is_some()||marker.settlement.is_some()||unwind.is_some(){return fail(refusal("native encode effect requires empty retained slots"))}
  if bytes==Some(0){return fail(refusal("zero-byte backing has no allocation event"))}
  if !local.fits(grant)||!admits(remaining(grant,*local),demand)||bytes.is_some_and(|bytes|bytes>demand.maximum_capacity_bytes){return fail(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encode effect exceeds its original local grant"))}
  if let Err(error)=self.checkpoint(){return fail(error)}
  let physical_grant=match self.construction_grant(RetirementDemand{copy_bytes:demand.maximum_copy_bytes,capacity_bytes:demand.maximum_capacity_bytes,release_bytes:demand.maximum_release_bytes,depth:demand.maximum_depth}){Ok(value)=>value,Err(error)=>return fail(error)};
  if !admits(physical_grant,demand){return fail(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encode effect exceeds its original physical grant"))}
  match self.prepare_work(1){Ok(true)=>{},Ok(false)=>return fail(ValueError::literal(ValueRefusalKind::WorkLimit,"native encode effect requires its original work loan")),Err(error)=>return fail(error)}
  let mut failure=NativeEncodeEffectFailure::default();
  let operation=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||match bytes{Some(bytes)=>{let outcome=self.allocate_effect(bytes,&mut *marker.actual,&mut *marker.allocation,&mut |actual|effect(&mut *marker.performed,&mut *marker.physical,actual));if allocation_failed(&outcome){failure.allocation=Some(outcome)}else if (*marker.performed||*marker.physical!=RetainedCloneProgress::default())&&!outcome.admitted{failure.invariant=Some(refusal("native encode allocation reported an effect without an actual post"));failure.allocation=Some(outcome)}Ok(())},None=>effect(&mut *marker.performed,&mut *marker.physical,&mut *marker.actual)}));
  let performed=*marker.performed||*marker.physical!=RetainedCloneProgress::default()||marker.actual.is_some();if !marker.physical.fits(demand){failure.physical_invariant=Some(refusal("native encode physical effect exceeded its quoted demand"))}if let Some(actual)=*marker.actual{if bytes.is_none()||marker.physical.retained_capacity_bytes!=actual{failure.capacity_invariant=Some(refusal("native encode effect omitted its actual allocation from physical progress"))}}
  failure.work=if performed{self.commit_work(1).err()}else{self.cancel_work().err()};
  if performed{match local.checked_add(*marker.physical){Ok(next)=>{*local=next;if !next.fits(grant){failure.local=Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encode actual effect exceeded its local grant"))}},Err(error)=>failure.local=Some(error)}if *marker.physical!=RetainedCloneProgress::default(){failure.physical=self.submit_constructed_progress(physical_grant,*marker.physical).err()}}
  let work=u64::from(performed);match operation{Ok(result)=>{if let Err(error)=result{failure.primary=Some(error)}RetainedWorkOutcome{work,result:if failure.is_empty(){Ok(())}else{Err(failure)}}},Err(payload)=>{if !failure.is_empty(){*unwind=Some(failure)}std::panic::resume_unwind(payload)}}
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
