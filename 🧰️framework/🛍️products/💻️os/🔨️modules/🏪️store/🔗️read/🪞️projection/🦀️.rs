//! 🔗️ Captured context roots retain the exact original Store read issuer.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedAllocationEffect,RetainedWorkPoll,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement},retained_clone::RetainedCloneProgress};
use std::{mem::ManuallyDrop,sync::Arc};

/// 📸️ An immutable projection cannot retire before its exact original read is bound.
pub struct SnapshotReadProjection<T:Send+Sync+'static>{projection:ManuallyDrop<Option<Arc<T>>>,read:ManuallyDrop<Option<crate::ErasedSnapshotRead>>}
impl<T:Send+Sync+'static> SnapshotReadProjection<T>{
 pub fn new(projection:Arc<T>)->Self{Self{projection:ManuallyDrop::new(Some(projection)),read:ManuallyDrop::new(None)}}
 pub fn bind(&mut self,read:crate::SnapshotRead<T>)->Result<(),crate::SnapshotRead<T>>{if self.read.is_some()||!self.projection.as_ref().is_some_and(|value|std::ptr::eq(value.as_ref(),read.get())){return Err(read);}*self.read=Some(read.into_erased());Ok(())}
 pub fn get(&self)->&T{self.projection.as_deref().expect("original context projection transferred")}
 pub fn has_original_issuer(&self)->bool{self.projection.as_ref().zip(self.read.as_ref()).is_some_and(|(value,read)|read.get::<T>().is_some_and(|original|std::ptr::eq(value.as_ref(),original)))}
}
impl<T:Send+Sync+'static> std::ops::Deref for SnapshotReadProjection<T>{type Target=T;fn deref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> AsRef<T> for SnapshotReadProjection<T>{fn as_ref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> Drop for SnapshotReadProjection<T>{fn drop(&mut self){assert!(std::thread::panicking()||(self.projection.is_none()&&self.read.is_none()),"context root retains its original projection or Store issuer");if self.projection.is_none()&&self.read.is_none(){unsafe{ManuallyDrop::drop(&mut self.projection);ManuallyDrop::drop(&mut self.read);}}}}

#[doc(hidden)]
pub struct Cursor<T:Send+Sync+'static>{root:SnapshotReadProjection<T>,active:ManuallyDrop<Option<ControlledRetirement<crate::ErasedSnapshotRead>>>}
impl<T:Send+Sync+'static> Cursor<T>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.root.projection.is_some(){return if self.root.has_original_issuer(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"context projection retains no matching original Store issuer"))};}
  if self.root.read.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  self.active.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))
 }
 fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();let mut work=0;
  let demand=match self.demands(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return RetirementStep::Progress(Default::default(),0);}
  let moved=|work:u128|RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},work);
  if self.root.projection.is_some(){if let Err(error)=crate::pay_pure_work(effect,&mut work){return RetirementStep::Failure(error.into(),work);}drop(self.root.projection.take());return moved(work);}
  if let Some(read)=self.root.read.take(){
   if let Err(error)=crate::pay_pure_work(effect,&mut work){*self.root.read=Some(read);return RetirementStep::Failure(error.into(),work);}
   return match ControlledRetirement::new(read){Ok(owner)=>{*self.active=Some(owner);moved(work)},Err((error,read))=>{*self.root.read=Some(read);RetirementStep::Failure(error.into(),work)}};
  }
  let Some(owner)=self.active.as_mut()else{return RetirementStep::Progress(Default::default(),0)};
  if owner.terminal_is_empty(){if let Err(error)=crate::pay_pure_work(effect,&mut work){return RetirementStep::Failure(error.into(),work);}drop(self.active.take());return moved(work);}
  match owner.step(body,effect){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 }
}
impl<T:Send+Sync+'static> RetireOwned for SnapshotReadProjection<T>{type RetirementCursor=Cursor<T>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{root:self,active:ManuallyDrop::new(None)}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+Sync+'static> RetirementCursor for Cursor<T>{
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.demands(body) }
 fn close_into<'r>(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Ok(reply.complete(Ok(self.close_sync(body,effect))))}
    fn recover_close_into<'r>(&mut self,_body:usize,_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::invariant("synchronous close has no unrecorded callback"))}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}match self.active.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.next_computational_work(body),_=>Ok(1)}}
 fn terminal_is_empty(&self)->bool{self.root.projection.is_none()&&self.root.read.is_none()&&self.active.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Cursor<T>{
 fn close_sync(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0);}self.step(body,effect)}
}
impl<T:Send+Sync+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"context cursor retains original source custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.active);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
