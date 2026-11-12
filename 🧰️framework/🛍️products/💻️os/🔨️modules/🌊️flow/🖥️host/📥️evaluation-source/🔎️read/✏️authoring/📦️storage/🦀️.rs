//! 📦️ Original authoring child backing is allocated, moved and released through its canonical physical owners.
use semio_framework_job::{OriginalTurnFailure,StepContext,OriginalConstructionAdmission,OriginalConstructionDemand,admit_original_construction_settled};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value_native_storage::NativeOwnedStorage;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original authoring child storage changed custody")}
/// 🏭️ Holds one settled construction attempt and its born marker across turns.
#[derive(Default)]
pub(super) struct Construction{admission:OriginalConstructionAdmission,built:Option<()>}
impl Construction{
 pub(super) fn run(&mut self,cx:&mut StepContext<'_>,demand:OriginalConstructionDemand,build:impl FnOnce()->Result<((),RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
  let settled=admit_original_construction_settled(cx,&mut self.admission,&mut self.built,demand,1,|_,_|RetainedWorkOutcome{result:build(),work:1})?;
  if settled{self.built=None}Ok(settled)
 }
}
pub(super) fn storage_demands<T>(slot:&NativeOwnedStorage<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{
 if !slot.reserved(){return Ok(NativeOwnedStorage::<T>::reserve_demand())}
 if !slot.initialized(){return Ok(NativeOwnedStorage::<T>::move_demand())}
 quote(slot.get().ok_or_else(invalid)?)
}
pub(super) fn storage_ready<T>(slot:&mut NativeOwnedStorage<T>,cx:&mut StepContext<'_>,create:impl FnOnce()->T)->Result<bool,OriginalTurnFailure>{
 if !slot.reserved(){
  let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(grant,NativeOwnedStorage::<T>::reserve_demand()){return Ok(false)}
  let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|slot.reserve_empty(effect))?else{return Ok(false)};
  cx.consume_work(performed);
  if let RetainedWorkPoll::Complete(turn)=poll{cx.consume_retained(turn.result?.progress())?}
  return Ok(false)
 }
 if !slot.initialized(){
  let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(grant,NativeOwnedStorage::<T>::move_demand()){return Ok(false)}
  let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|{let mut candidate=Some(create());let turn=slot.put_from(&mut candidate,effect);assert!(candidate.is_none()||turn.result.is_err(),"admitted native storage move left its candidate");turn})?else{return Ok(false)};
  cx.consume_work(performed);cx.consume_retained(turn.result?.progress())?;return Ok(false)
 }
 Ok(true)
}
pub(super) fn storage_close_demands<T>(slot:&NativeOwnedStorage<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>,empty:impl FnOnce(&T)->bool)->Result<RetirementDemand,ValueError>{
 if let Some(child)=slot.get(){if !empty(child){return quote(child)}return Ok(NativeOwnedStorage::<T>::move_demand())}
 Ok(slot.release_demand())
}
pub(super) fn storage_close_step<T>(slot:&mut NativeOwnedStorage<T>,effect:&mut RetainedAllocationEffect<'_>,step:impl FnOnce(&mut T,RetainedCloneGrant,&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome,empty:impl FnOnce(&T)->bool)->RetainedWorkOutcome{
 if let Some(child)=slot.get_mut(){if !empty(child){let grant=effect.grant();return step(child,grant,effect)}let mut owner=None;let outcome=slot.take_into(&mut owner,effect);drop(owner);return outcome}
 slot.release(effect)
}
