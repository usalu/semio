//! 🚦️ Only this executor's native wake issuer can hand its original atomic signal lease to a Worker.
use super::*;
use std::mem::{ManuallyDrop,size_of};
use semio_framework_job::{RetainedWorkerWake,admit_original_worker_wake,original_worker_wake_admission_demands};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,RetirementStep}};

struct ExecutorSignalLease{source:ManuallyDrop<Option<Arc<ExecutorWakeSignal>>>,id:Option<TaskId>}
impl RetireOwned for WakerData{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ExecutorSignalLease{source:ManuallyDrop::new(Some(self.signal)),id:Some(self.id)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ExecutorSignalLease>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl RetirementCursor for ExecutorSignalLease{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete}
  let copy=self.next_work_byte_demand().unwrap();let release=self.next_close_byte_demand().unwrap();
  if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}
  if Arc::weak_count(self.source.as_ref().unwrap())!=0{return RetirementStep::BudgetExhausted}
  let released=Arc::into_inner(self.source.take().unwrap()).map_or(0,|original|{drop(original);release});self.id.take();
  RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:released,..Default::default()})
 }
 fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.id.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.terminal_is_empty(){0}else{size_of::<Option<Arc<ExecutorWakeSignal>>>()+size_of::<Arc<ExecutorWakeSignal>>()+size_of::<Option<TaskId>>()})}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.terminal_is_empty(){0}else{semio_framework_value::shared_retirement_allocation_bytes::<ExecutorWakeSignal>()})}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ExecutorSignalLease{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"executor atomic signal lease abandoned original custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);}}}}

/// 🎟️ Obtains an original first-party wake lease only after all unchanged admission axes permit its concrete frame.
pub fn admit_original_executor_wake(original:&Waker,grant:RetainedCloneGrant)->Result<Option<(Box<dyn RetainedWorkerWake>,RetainedCloneProgress)>,ValueError>{
 if !std::ptr::eq(original.vtable(),&VTABLE){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign executor wake has no original typed authority"))}
 let demand=original_worker_wake_admission_demands::<WakerData>()?;let alias_copy=size_of::<Arc<WakerData>>();
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes+alias_copy||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
 let pointer=original.data().cast::<WakerData>();unsafe{Arc::increment_strong_count(pointer);}
 let mut source=Some(unsafe{Arc::from_raw(pointer)});let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-alias_copy,..grant};
 let (owner,mut receipt)=admit_original_worker_wake(&mut source,child)?.expect("original issuer admitted its exact typed frame");receipt.copied_bytes+=alias_copy;Ok(Some((owner,receipt)))
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
