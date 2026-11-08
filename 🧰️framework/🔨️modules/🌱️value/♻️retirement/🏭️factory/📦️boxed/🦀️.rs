//! 📦️ Erased boxed mutations retain their concrete allocation and installed issuer.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::{mem::ManuallyDrop,sync::Arc};
/// 🧳️ An original typed box carries the factory that owns its payload retirement.
pub struct FactoryBoxedValue<T:Send+'static>{pub original:Box<T>,pub factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>}
/// 📭️ Publication moves the payload while retaining both original box allocations and its issuer.
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
pub struct FactoryBoxedPublication<T:Send+'static>{payload:OriginalBoxSlot<T>,carrier:OriginalBoxSlot<FactoryBoxedValue<T>>,issuer:CapturedIssuer<T>}
struct CapturedIssuer<T:Send+'static>{factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>}
impl<T:Send+'static> RetireOwned for CapturedIssuer<T>{type RetirementCursor=FactoryAuthority;fn retirement_seed(self)->Self::RetirementCursor{let factory:Arc<dyn FactoryRetirement>=self.factory;FactoryAuthority::new(factory)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<FactoryAuthority>())}fn controlled_retirement_supported()->bool{true}}
struct OriginalBoxSlot<T:Send+'static>{allocation:ManuallyDrop<Option<Box<std::mem::MaybeUninit<T>>>>}
impl<T:Send+'static> FactoryBoxedValue<T>{
 pub fn take_for_publication(self:Box<Self>)->(T,FactoryBoxedPublication<T>){
  let carrier=Box::into_raw(self);let owned=unsafe{carrier.read()};let payload=Box::into_raw(owned.original);let value=unsafe{payload.read()};
  (value,FactoryBoxedPublication{payload:OriginalBoxSlot{allocation:ManuallyDrop::new(Some(unsafe{Box::from_raw(payload.cast())}))},carrier:OriginalBoxSlot{allocation:ManuallyDrop::new(Some(unsafe{Box::from_raw(carrier.cast())}))},issuer:CapturedIssuer{factory:owned.factory}})
 }
}
impl<T:Send+'static> FactoryBoxedPublication<T>{
 /// 🔁️ Rejected publication restores the same original carrier, payload slot and issuer without allocation.
 pub fn restore(mut self,value:T)->Box<FactoryBoxedValue<T>>{let payload=Box::into_raw(self.payload.allocation.take().unwrap()).cast::<T>();let carrier=Box::into_raw(self.carrier.allocation.take().unwrap()).cast::<FactoryBoxedValue<T>>();unsafe{payload.write(value);carrier.write(FactoryBoxedValue{original:Box::from_raw(payload),factory:self.issuer.factory});Box::from_raw(carrier)}}
}
impl<T:Send+'static> RetireOwned for OriginalBoxSlot<T>{type RetirementCursor=Self;fn retirement_seed(self)->Self::RetirementCursor{self}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+'static> RetirementCursor for OriginalBoxSlot<T>{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.allocation.is_some()))}
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:if self.allocation.is_some(){size_of::<Option<Box<std::mem::MaybeUninit<T>>>>()}else{0},release_bytes:if self.allocation.is_some(){size_of::<T>()}else{0},depth:usize::from(self.allocation.is_some()),..Default::default()})}
 
 fn terminal_is_empty(&self)->bool{self.allocation.is_none()}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.terminal_is_empty(){0}else{size_of::<T>()})}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+'static> OriginalBoxSlot<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if self.terminal_is_empty(){return RetirementStep::Complete(0);}let bytes=size_of::<T>();let copy=size_of::<Option<Box<std::mem::MaybeUninit<T>>>>();if effect.maximum_work_units()==0||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0||grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted;}let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};drop(self.allocation.take());RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:bytes,..Default::default()},work)}
}

impl<T:Send+'static> Drop for OriginalBoxSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original boxed allocation abandoned before funded release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.allocation);}}}}
#[doc(hidden)]
pub struct Cursor<T:Send+'static>{boxed:ManuallyDrop<Option<Box<T>>>,vacant:ManuallyDrop<Option<OriginalBoxSlot<T>>>,owned:crate::owned::FactoryOwnedRetirement<T>}
impl<T:Send+'static> Cursor<T>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.boxed.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<T>>>()+size_of::<T>()+size_of::<Option<T>>()+size_of::<Option<OriginalBoxSlot<T>>>(),depth:1,..Default::default()})}if self.vacant.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<OriginalBoxSlot<T>>>()+size_of::<Option<Box<std::mem::MaybeUninit<T>>>>(),release_bytes:size_of::<T>(),depth:1,..Default::default()})}self.owned.demands(body)}
}
impl<T:Send+'static> RetireOwned for FactoryBoxedValue<T>{type RetirementCursor=Cursor<T>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{boxed:ManuallyDrop::new(Some(self.original)),vacant:ManuallyDrop::new(None),owned:crate::owned::FactoryOwnedRetirement::from_captured(None,self.factory)}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+'static> RetirementCursor for Cursor<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.boxed.is_some()||self.vacant.is_some(){Ok(1)}else{self.owned.next_computational_work(body)}}
 fn requires_original_birth(&self)->bool{self.boxed.is_none()&&self.vacant.is_none()&&self.owned.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{crate::authority_cursor_turn(self.owned.close_original_birth_step(original,grant))}
 fn requires_original_control(&self)->bool{self.boxed.is_none()&&self.vacant.is_none()&&self.owned.requires_original_control()}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{crate::authority_cursor_turn(self.owned.close_step_controlled(body,effect,control))}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{crate::authority_cursor_turn(self.owned.close_original_step(original,body,effect))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 
 fn terminal_is_empty(&self)->bool{self.boxed.is_none()&&self.vacant.is_none()&&self.owned.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+'static> Cursor<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{
  if self.boxed.is_none()&&self.vacant.is_none(){return crate::authority_cursor_turn(self.owned.close_step(body,effect))}let demand=match self.demands(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};if let Some(original)=self.boxed.take(){let pointer=Box::into_raw(original);self.owned.receive_original(Some(unsafe{pointer.read()}));*self.vacant=Some(OriginalBoxSlot{allocation:ManuallyDrop::new(Some(unsafe{Box::from_raw(pointer.cast())}))});return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}let vacant=self.vacant.as_mut().expect("funded original box allocation");drop(vacant.allocation.take());drop(self.vacant.take());RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)
 }
}

impl<T:Send+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"boxed factory retains original custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.boxed);ManuallyDrop::drop(&mut self.vacant)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;




