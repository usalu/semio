//! 📦️ Original authoring child backing is allocated, moved and released through its canonical physical owners.
use semio_framework_job::{OriginalTurnFailure,StepContext,OriginalConstructionDemand,admit_original_construction,receive_original_retained_work};
use semio_framework_value::{RetainedCloneGrant,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value_native_storage::NativeOwnedStorage;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original authoring child storage changed custody")}
pub(super) fn storage_demands<T>(slot:&NativeOwnedStorage<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{
 if !slot.reserved(){return Ok(NativeOwnedStorage::<T>::reserve_demand())}
 if !slot.initialized(){return Ok(NativeOwnedStorage::<T>::move_demand())}
 quote(slot.get().ok_or_else(invalid)?)
}
pub(super) fn storage_ready<T>(slot:&mut NativeOwnedStorage<T>,cx:&mut StepContext<'_>,create:impl FnOnce()->T)->Result<bool,OriginalTurnFailure>{
 if !slot.reserved(){
  let d=NativeOwnedStorage::<T>::reserve_demand();let grant=cx.retained_grant();let mut admitted=None;
  admit_original_construction(cx,&mut admitted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|{
   let turn=slot.reserve(grant);RetainedWorkOutcome{result:turn.result.map(|step|((),step.progress())),work:turn.work}
  })?;
  return Ok(false)
 }
 if !slot.initialized(){
  let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(grant,NativeOwnedStorage::<T>::move_demand()){return Ok(false)}
  let mut candidate=Some(create());let turn=slot.put_from(&mut candidate,grant);receive_original_retained_work(cx,turn)?;return Ok(false)
 }
 Ok(true)
}
pub(super) fn storage_close_demands<T>(slot:&NativeOwnedStorage<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>,empty:impl FnOnce(&T)->bool)->Result<RetirementDemand,ValueError>{
 if let Some(child)=slot.get(){if !empty(child){return quote(child)}return Ok(NativeOwnedStorage::<T>::move_demand())}
 Ok(slot.release_demand())
}
pub(super) fn storage_close_step<T>(slot:&mut NativeOwnedStorage<T>,grant:RetainedCloneGrant,step:impl FnOnce(&mut T,RetainedCloneGrant)->RetainedWorkOutcome,empty:impl FnOnce(&T)->bool)->RetainedWorkOutcome{
 if let Some(child)=slot.get_mut(){if !empty(child){return step(child,grant)}let mut owner=None;let outcome=slot.take_into(&mut owner,grant);drop(owner);return outcome}
 slot.release(grant)
}
