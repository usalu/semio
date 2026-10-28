//! 🔒️ Store registry shares expose strong custody without raw Arc or Weak capabilities.
use super::{SnapshotReadLeaseRegistry,SnapshotReadLease,SnapshotReadLeaseSlot,SNAPSHOT_READ_LEASE_CAPACITY};
use std::sync::Arc;
#[path="🌱️birth/🦀️.rs"]
pub mod birth;

#[repr(transparent)]
pub struct SnapshotReadRegistryHandle {inner:Arc<SnapshotReadLeaseRegistry>}
impl Clone for SnapshotReadRegistryHandle {fn clone(&self)->Self{Self{inner:self.inner.clone()}}}
impl std::ops::Deref for SnapshotReadRegistryHandle {type Target=SnapshotReadLeaseRegistry;fn deref(&self)->&Self::Target{&self.inner}}
impl SnapshotReadRegistryHandle {
 pub fn new()->Self{Self{inner:Arc::new(SnapshotReadLeaseRegistry::new())}}
 pub fn identity(&self)->usize{Arc::as_ptr(&self.inner) as usize}
 pub fn strong_count(&self)->usize{Arc::strong_count(&self.inner)}
 pub fn ptr_eq(&self,other:&Self)->bool{Arc::ptr_eq(&self.inner,&other.inner)}
 pub fn try_unwrap(self)->Result<SnapshotReadLeaseRegistry,Self>{Arc::try_unwrap(self.inner).map_err(|inner|Self{inner})}
 pub fn into_inner(self)->Option<SnapshotReadLeaseRegistry>{Arc::into_inner(self.inner)}
    pub fn try_issue<T:Send+Sync+'static>(&self,owner:Arc<T>)->Result<SnapshotReadLease,Arc<T>>{self.try_issue_original(owner).map_err(|(_,original)|original)}
    /// 🎟️ The exact issuer returns both its allocation-free refusal and the unchanged original owner on denial.
    pub fn try_issue_original<T:Send+Sync+'static>(&self,owner:Arc<T>)->Result<SnapshotReadLease,(super::SnapshotReadLeaseRefusal,Arc<T>)>{
        let mut state=match self.state.try_lock(){Ok(state)=>state,Err(std::sync::TryLockError::WouldBlock)=>return Err((super::SnapshotReadLeaseRefusal::Busy,owner)),Err(std::sync::TryLockError::Poisoned(_))=>return Err((super::SnapshotReadLeaseRefusal::Poisoned,owner))};
        if state.free_len==0||!state.slots.resident(){return Err((super::SnapshotReadLeaseRefusal::Admission(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"snapshot read issuer has no admitted vacant slot")),owner))}
        let generation=match state.next_generation.checked_add(1){Some(generation)if generation<1u64<<63=>generation,_=>return Err((super::SnapshotReadLeaseRefusal::Admission(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"snapshot read issuer generation exhausted")),owner))};
        let read=state.free_read;let index=state.free[read]as usize;state.free_read=(state.free_read+1)%SNAPSHOT_READ_LEASE_CAPACITY;state.free_len-=1;state.next_generation=generation;state.slots[index].write(SnapshotReadLeaseSlot{generation,owner});self.lease_generations[index].store(generation,std::sync::atomic::Ordering::Release);state.occupied[index/64]|=1<<(index%64);
        Ok(SnapshotReadLease{registry:self.clone(),index:index as u16,generation})
    }

}
