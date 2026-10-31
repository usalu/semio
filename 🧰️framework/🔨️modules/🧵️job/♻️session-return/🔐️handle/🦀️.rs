//! 🔐️ Seals the original session Arc behind strong-only first-party custody.
use super::super::WorkerJobSessionInner;
use std::sync::Arc;
pub(crate) struct SessionHandle<J>{inner:Arc<WorkerJobSessionInner<J>>}
impl<J> SessionHandle<J>{
 pub(crate) fn admit_empty(generation:semio_framework_job::Generation,grant:semio_framework_job::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(Self,semio_framework_job::RetainedCloneProgress)>>{
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<Self::frame_bytes(){return semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0}}
  let value=WorkerJobSessionInner{generation,phase:std::sync::atomic::AtomicU8::new(super::super::SESSION_EMPTY),authority:std::mem::ManuallyDrop::new(std::cell::UnsafeCell::new(None)),rejection_kind:std::sync::atomic::AtomicU8::new(u8::MAX),close_requested:std::sync::atomic::AtomicBool::new(false),terminal_intent:std::sync::atomic::AtomicU8::new(0),wake_pending:std::sync::atomic::AtomicBool::new(false),wake_sequence:std::sync::atomic::AtomicU64::new(0),wake_exhausted:std::sync::atomic::AtomicBool::new(false),wake_guard:std::sync::atomic::AtomicBool::new(false),waker:std::mem::ManuallyDrop::new(std::cell::UnsafeCell::new(None)),retained_wake:std::mem::ManuallyDrop::new(std::cell::UnsafeCell::new(None)),last_step_end_us:std::sync::atomic::AtomicU64::new(u64::MAX)};
  let turn=semio_framework_value::retirement::allocation_return::admit_shared_allocation(value,grant);let result=match turn.result{Ok((inner,p))=>Ok(Some((Self{inner},p))),Err((error,empty))=>{drop(empty);Err(error)}};semio_framework_value::RetainedWorkOutcome{result,work:turn.work}
 }
 pub(crate) fn frame_bytes()->usize{semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>()}
 pub(crate) fn is_unique(&self)->bool{Arc::strong_count(&self.inner)==1&&Arc::weak_count(&self.inner)==0}
 pub(crate) fn try_return(self)->Result<WorkerJobSessionInner<J>,Self>{Arc::try_unwrap(self.inner).map_err(|inner|Self{inner})}
}
impl<J> Clone for SessionHandle<J>{fn clone(&self)->Self{Self{inner:Arc::clone(&self.inner)}}}
impl<J> std::ops::Deref for SessionHandle<J>{type Target=WorkerJobSessionInner<J>;fn deref(&self)->&Self::Target{&self.inner}}
