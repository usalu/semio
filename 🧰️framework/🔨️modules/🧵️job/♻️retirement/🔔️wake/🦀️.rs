//! 🔔️ Original typed wake custody keeps the Waker lease, shared allocation and payload on separate paid frontiers.
use super::*;
use semio_framework_value::{ValueError,retirement::{RetireOwned,controlled::ControlledRetirement,shared::shared_retirement_allocation_bytes}};

pub trait RetainedWorkerWake:Send{
 fn wake_by_ref(&self);
 fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn frame_release_bytes(&self)->usize;
}
struct OriginalWorkerWake<T:std::task::Wake+RetireOwned+Sync>{waker:ManuallyDrop<Option<Waker>>,source:ManuallyDrop<Option<Arc<T>>>,payload:ManuallyDrop<Option<ControlledRetirement<T>>>}
impl<T:std::task::Wake+RetireOwned+Sync> RetainedWorkerWake for OriginalWorkerWake<T>{
 fn wake_by_ref(&self){if let Some(waker)=self.waker.as_ref(){waker.wake_by_ref();}}
 fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  if self.waker.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Waker>>()+size_of::<Waker>(),depth:1,..Default::default()})}
  if self.source.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<T>()+size_of::<Option<Arc<T>>>()+size_of::<ControlledRetirement<T>>(),release_bytes:shared_retirement_allocation_bytes::<T>(),depth:1,..Default::default()})}
  if let Some(payload)=self.payload.as_ref(){return if payload.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:payload.next_copy_byte_demand()?,capacity_bytes:payload.next_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:payload.next_release_byte_demand()?,depth:payload.next_depth_demand()?})}}
  Ok(Default::default())
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.waker.is_some(){drop(self.waker.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
  if let Some(source)=self.source.as_ref(){if Arc::strong_count(source)!=1||Arc::weak_count(source)!=0{return Ok(RetainedCloneStep::Progress(Default::default()))}match Arc::try_unwrap(self.source.take().unwrap()){Ok(original)=>{*self.payload=Some(ControlledRetirement::new(original).unwrap_or_else(|_|panic!("admitted original wake payload retains its authority")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))},Err(original)=>{*self.source=Some(original);return Ok(RetainedCloneStep::Progress(Default::default()))}}}
  let payload=self.payload.as_mut().unwrap();
  if payload.terminal_is_empty(){drop(self.payload.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
  let step=payload.step(grant)?;let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,payload.terminal_is_empty(),"original worker wake payload")?.progress();Ok(RetainedCloneStep::Progress(progress))
 }
 fn terminal_is_empty(&self)->bool{self.waker.is_none()&&self.source.is_none()&&self.payload.is_none()}
 fn frame_release_bytes(&self)->usize{size_of::<Self>()}
}
impl<T:std::task::Wake+RetireOwned+Sync> Drop for OriginalWorkerWake<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original worker wake abandoned typed custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.waker);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.payload);}}}}

/// 📏️ Borrows the original typed frame admission before obtaining any shared lease.
pub fn original_worker_wake_admission_demands<T:std::task::Wake+RetireOwned+Sync>()->Result<RetirementDemand,ValueError>{
 if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original typed wake lacks retirement authority"))}
 let capacity=size_of::<OriginalWorkerWake<T>>();Ok(RetirementDemand{copy_bytes:capacity+size_of::<Option<Arc<T>>>(),capacity_bytes:capacity,depth:1,..Default::default()})
}

/// 🎟️ Admits the exact typed wake frame before moving its original shared owner.
pub fn admit_original_worker_wake<T:std::task::Wake+RetireOwned+Sync>(original:&mut Option<Arc<T>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn RetainedWorkerWake>,RetainedCloneProgress)>,ValueError>{
 if original.is_none(){return Ok(None)}
 if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original typed wake lacks retirement authority"))}
 let demand=original_worker_wake_admission_demands::<T>()?;let capacity=demand.capacity_bytes;let copy=demand.copy_bytes;
 if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity||grant.maximum_depth==0{return Ok(None)}
 let source=original.take().unwrap();let waker=Waker::from(Arc::clone(&source));let owner=Box::new(OriginalWorkerWake{waker:ManuallyDrop::new(Some(waker)),source:ManuallyDrop::new(Some(source)),payload:ManuallyDrop::new(None)});
 Ok(Some((owner,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()})))
}

/// 🌉️ First-party embedding supplier returns the original typed frame and its complete receipt.
pub type OriginalWorkerWakeIssuer=fn(&Waker,RetainedCloneGrant)->Result<Option<(Box<dyn RetainedWorkerWake>,RetainedCloneProgress)>,ValueError>;
/// 🧵️ Admits this actual native Thread issuer's lease and concrete Worker frame as one original wake element.
#[cfg(not(target_arch="wasm32"))]
pub fn admit_original_thread_worker_wake(original:&Waker,grant:RetainedCloneGrant)->Result<Option<(Box<dyn RetainedWorkerWake>,RetainedCloneProgress)>,ValueError>{
 use semio_framework_async::original_thread_wake::{OriginalThreadWake,admit_original_thread_wake,original_thread_wake_lease_copy_bytes,original_thread_wake_is_issued};
 if !original_thread_wake_is_issued(original){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign wake has no original native Thread authority"))}
 let demand=original_worker_wake_admission_demands::<OriginalThreadWake>()?;let alias_copy=original_thread_wake_lease_copy_bytes();
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes+alias_copy||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
 let Some((source,lease_receipt))=admit_original_thread_wake(original,grant)?else{return Ok(None)};
 let mut source=Some(source);let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-lease_receipt.copied_bytes,..grant};
 let (owner,mut receipt)=admit_original_worker_wake(&mut source,child)?.expect("native Thread issuer admitted its original full frame");receipt.copied_bytes+=lease_receipt.copied_bytes;Ok(Some((owner,receipt)))
}

pub(super) fn session_retained_wake_demands<J>(inner:&WorkerJobSessionInner<J>,copy:usize)->Result<Option<RetirementDemand>,ValueError>{
 let Some(original)=(unsafe{(&*inner.retained_wake.get()).as_ref()})else{return Ok(None)};
 if original.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn RetainedWorkerWake>>>(),release_bytes:original.frame_release_bytes(),depth:1,..Default::default()}))}
 original.retirement_demands(copy).map(Some)
}
struct WakeGate<'a>(&'a AtomicBool);
impl Drop for WakeGate<'_>{fn drop(&mut self){self.0.store(false,Ordering::Release)}}
pub(super) fn close_session_retained_wake<J>(inner:&WorkerJobSessionInner<J>,grant:RetainedCloneGrant)->Option<WorkerJobCloseStep>{
 let demand=match session_retained_wake_demands(inner,grant.maximum_copy_bytes){Ok(Some(demand))=>demand,Ok(None)=>return None,Err(error)=>return Some(WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()})};
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Some(WorkerJobCloseStep::Pending{progress:Default::default()})}
 if inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Some(WorkerJobCloseStep::Blocked)}
 let _gate=WakeGate(&inner.wake_guard);let slot=unsafe{&mut *inner.retained_wake.get()};let original=slot.as_mut().expect("original retained wake admitted by its exclusive gate");
 if original.terminal_is_empty(){drop(slot.take());return Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}})}
 Some(match original.close_step(grant){Ok(step)=>WorkerJobCloseStep::Pending{progress:step.progress()},Err(error)=>WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
