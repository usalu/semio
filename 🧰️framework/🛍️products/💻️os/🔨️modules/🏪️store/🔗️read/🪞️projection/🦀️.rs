//! 🔗️ Captured context roots retain the exact original Store read issuer.
use super as store;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::{mem::ManuallyDrop,sync::Arc};

/// 📸️ An immutable projection cannot retire before its exact original read is bound.
pub struct SnapshotReadProjection<T:Send+Sync+'static>{projection:ManuallyDrop<Option<Arc<T>>>,read:ManuallyDrop<Option<store::ErasedSnapshotRead>>}
impl<T:Send+Sync+'static> SnapshotReadProjection<T>{
 pub fn new(projection:Arc<T>)->Self{Self{projection:ManuallyDrop::new(Some(projection)),read:ManuallyDrop::new(None)}}
 pub fn bind(&mut self,read:store::SnapshotRead<T>)->Result<(),store::SnapshotRead<T>>{if self.read.is_some()||!self.projection.as_ref().is_some_and(|value|std::ptr::eq(value.as_ref(),read.get())){return Err(read);}*self.read=Some(read.into_erased());Ok(())}
 pub fn get(&self)->&T{self.projection.as_deref().expect("original context projection transferred")}
 pub fn has_original_issuer(&self)->bool{self.projection.as_ref().zip(self.read.as_ref()).is_some_and(|(value,read)|read.get::<T>().is_some_and(|original|std::ptr::eq(value.as_ref(),original)))}
}
impl<T:Send+Sync+'static> std::ops::Deref for SnapshotReadProjection<T>{type Target=T;fn deref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> AsRef<T> for SnapshotReadProjection<T>{fn as_ref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> Drop for SnapshotReadProjection<T>{fn drop(&mut self){assert!(std::thread::panicking()||(self.projection.is_none()&&self.read.is_none()),"context root retains its original projection or Store issuer");if self.projection.is_none()&&self.read.is_none(){unsafe{ManuallyDrop::drop(&mut self.projection);ManuallyDrop::drop(&mut self.read);}}}}

struct Cursor<T:Send+Sync+'static>{root:SnapshotReadProjection<T>,active:ManuallyDrop<Option<ControlledRetirement<store::ErasedSnapshotRead>>>}
impl<T:Send+Sync+'static> Cursor<T>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.root.projection.is_some(){return if self.root.has_original_issuer(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"context projection retains no matching original Store issuer"))};}
  if self.root.read.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  self.active.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
  if self.root.projection.is_some(){drop(self.root.projection.take());return Ok(RetainedCloneProgress{copied_items:1,..Default::default()});}
  if let Some(read)=self.root.read.take(){return match ControlledRetirement::new(read){Ok(owner)=>{*self.active=Some(owner);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})},Err((error,read))=>{*self.root.read=Some(read);Err(error)}};}
  let Some(owner)=self.active.as_mut()else{return Ok(Default::default())};if owner.terminal_is_empty(){drop(self.active.take());return Ok(RetainedCloneProgress{copied_items:1,..Default::default()});}owner.step(grant).map(|step|step.progress())
 }
}
impl<T:Send+Sync+'static> RetireOwned for SnapshotReadProjection<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{root:self,active:ManuallyDrop::new(None)})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+Sync+'static> RetirementCursor for Cursor<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.step(grant){Ok(progress)=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.root.projection.is_none()&&self.root.read.is_none()&&self.active.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"context cursor retains original source custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.active);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;