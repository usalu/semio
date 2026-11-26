//! 📦️ Original authoring child backing is allocated, moved and released through its canonical physical owners.
use semio_framework_job::{OriginalTurnFailure,StepContext,OriginalConstructionDemand};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value_native_storage::{NativeOwnedStorage,NativeSeedTemplate};
use std::mem::{ManuallyDrop,MaybeUninit};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original authoring child storage changed custody")}
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

/// 🧊️ One immutable vacant work owns no resource, so its bytes are the exact template every resident backing is seeded from.
pub(super) struct VacantCell<T>(ManuallyDrop<T>);
unsafe impl<T> Sync for VacantCell<T>{}
unsafe impl<T> Send for VacantCell<T>{}
impl<T> VacantCell<T>{pub(super) fn new(value:T)->Self{Self(ManuallyDrop::new(value))}}
/// 🧬️ Names the one immutable vacant instance of a large retained work.
pub(super) trait Vacant:Sized+'static{fn vacant()->&'static VacantCell<Self>;}
macro_rules! vacant{($type:ty,$new:expr)=>{impl super::storage::Vacant for $type{fn vacant()->&'static super::storage::VacantCell<Self>{static CELL:std::sync::OnceLock<super::storage::VacantCell<$type>>=std::sync::OnceLock::new();CELL.get_or_init(||super::storage::VacantCell::new($new))}}}}
pub(super) use vacant;
unsafe fn seed_part<T:Vacant>(target:*mut MaybeUninit<T>,offset:usize,bytes:usize){unsafe{let source=(&raw const *T::vacant().0).cast::<MaybeUninit<u8>>();target.cast::<MaybeUninit<u8>>().add(offset).copy_from_nonoverlapping(source.add(offset),bytes)}}
fn template<T:Vacant>()->NativeSeedTemplate<T>{unsafe{NativeSeedTemplate::new(seed_part::<T>)}}
/// 📦️ A large work lives in native backing seeded in bounded parts and returned through its box, so no single event copies the whole work.
pub(super) struct Slot<T>{storage:NativeOwnedStorage<T>,closed:ManuallyDrop<Option<Box<T>>>}
impl<T> Slot<T>{
 pub(super) const fn empty()->Self{Self{storage:NativeOwnedStorage::empty(),closed:ManuallyDrop::new(None)}}
 pub(super) fn reserved(&self)->bool{self.storage.reserved()}
 pub(super) fn get(&self)->Option<&T>{self.storage.get()}
 pub(super) fn get_mut(&mut self)->Option<&mut T>{self.storage.get_mut()}
 pub(super) fn terminal_is_empty(&self)->bool{self.storage.terminal_is_empty()&&self.closed.is_none()}
}
impl<T> Drop for Slot<T>{fn drop(&mut self){if self.storage.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.closed)}}}}
fn box_release_demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<T>>>(),release_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()}}
pub(super) fn work_demands<T:Vacant>(slot:&Slot<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{
 if !slot.storage.reserved(){return Ok(NativeOwnedStorage::<T>::reserve_demand())}
 if !slot.storage.initialized(){return slot.storage.template_seed_demand(template::<T>())}
 quote(slot.storage.get().ok_or_else(invalid)?)
}
pub(super) fn work_ready<T:Vacant>(slot:&mut Slot<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if !slot.storage.reserved(){
  let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(grant,NativeOwnedStorage::<T>::reserve_demand()){return Ok(false)}
  let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|slot.storage.reserve_empty(effect))?else{return Ok(false)};
  cx.consume_work(performed);
  if let RetainedWorkPoll::Complete(turn)=poll{cx.consume_retained(turn.result?.progress())?}
  return Ok(false)
 }
 if !slot.storage.initialized(){
  let grant=cx.retained_grant();let demand=slot.storage.template_seed_demand(template::<T>())?;if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(grant,demand){return Ok(false)}
  let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|slot.storage.initialize_template(template::<T>(),effect))?else{return Ok(false)};
  cx.consume_work(performed);cx.consume_retained(turn.result?.progress())?;return Ok(false)
 }
 Ok(true)
}
pub(super) fn work_close_demands<T>(slot:&Slot<T>,quote:impl FnOnce(&T)->Result<RetirementDemand,ValueError>,empty:impl FnOnce(&T)->bool)->Result<RetirementDemand,ValueError>{
 if let Some(child)=slot.storage.get(){if !empty(child){return quote(child)}return Ok(NativeOwnedStorage::<T>::box_demand())}
 if slot.closed.is_some(){return Ok(box_release_demand::<T>())}
 Ok(slot.storage.release_demand())
}
pub(super) fn work_close_step<T>(slot:&mut Slot<T>,effect:&mut RetainedAllocationEffect<'_>,step:impl FnOnce(&mut T,RetainedCloneGrant,&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome,empty:impl FnOnce(&T)->bool)->RetainedWorkOutcome{
 if let Some(child)=slot.storage.get_mut(){if !empty(child){let grant=effect.grant();return step(child,grant,effect)}return match slot.storage.take_box_into(&mut slot.closed,effect){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
 if slot.closed.is_some(){let d=box_release_demand::<T>();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original authoring child release exceeded its authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}drop(slot.closed.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}}
 slot.storage.release(effect)
}

/// 🏭️ Funds one physical reservation of a work inside its own original effect, so no decoder, return table or Source origin is ever left owing a refresh between caller turns.
#[derive(Default)]
pub(super) struct Construction;
impl Construction{
 /// 🧾️ Runs `build` as one funded original effect and posts its exact receipt; `None` while the turn cannot fund the whole quoted demand.
 pub(super) fn make<T>(&mut self,cx:&mut StepContext<'_>,demand:OriginalConstructionDemand,build:impl FnOnce()->Result<(T,RetainedCloneProgress),ValueError>)->Result<Option<T>,OriginalTurnFailure>{
  if cx.should_yield()||cx.is_cancelled()||!demand.fits(cx.retained_grant()){return Ok(None)}
  let funded=RetainedCloneGrant{maximum_items:demand.items,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  let Some((built,performed))=cx.with_original_effect(funded,|_,effect|match effect.performed(1){Ok(true)=>build(),Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original authoring reservation exceeded its authority")),Err(error)=>Err(error)})?else{return Ok(None)};
  cx.consume_work(performed);let(value,progress)=built?;cx.consume_retained(progress)?;Ok(Some(value))
 }
 /// 🧾️ Like [`Self::make`] for a reservation that moves no value; `true` once it was funded and posted.
 pub(super) fn run(&mut self,cx:&mut StepContext<'_>,demand:OriginalConstructionDemand,build:impl FnOnce()->Result<((),RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{Ok(self.make(cx,demand,build)?.is_some())}
}
