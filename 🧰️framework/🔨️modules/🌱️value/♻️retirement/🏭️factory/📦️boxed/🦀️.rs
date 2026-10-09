//! 📦️ Erased boxed mutations retain their concrete allocation and installed issuer.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::{mem::ManuallyDrop,sync::Arc};
/// 🧳️ An original typed box carries the factory that owns its payload retirement.
pub struct FactoryBoxedValue<T:Send+'static>{pub original:Box<T>,pub factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>}
/// 📭️ Publication moves the payload while retaining both original box allocations and its issuer.
#[derive(crate::RetireOwned)]
pub struct FactoryBoxedPublication<T:Send+'static>{payload:OriginalBoxSlot<T>,carrier:OriginalBoxSlot<FactoryBoxedValue<T>>,issuer:CapturedIssuer<T>}
struct CapturedIssuer<T:Send+'static>{factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>}
impl<T:Send+'static> RetireOwned for CapturedIssuer<T>{fn retirement(self)->Box<dyn RetirementCursor>{let factory:Arc<dyn FactoryRetirement>=self.factory;FactoryAuthority::new(factory).retirement()}fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<FactoryAuthority>())}fn controlled_retirement_supported()->bool{true}}
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
impl<T:Send+'static> RetireOwned for OriginalBoxSlot<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+'static> RetirementCursor for OriginalBoxSlot<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}let bytes=std::mem::size_of::<T>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted;}drop(self.allocation.take());RetirementStep::Bytes(bytes)}
 fn terminal_is_empty(&self)->bool{self.allocation.is_none()}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.terminal_is_empty(){0}else{std::mem::size_of::<T>()})}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<T:Send+'static> Drop for OriginalBoxSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original boxed allocation abandoned before funded release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.allocation);}}}}
struct Cursor<T:Send+'static>{boxed:ManuallyDrop<Option<Box<T>>>,vacant:ManuallyDrop<Option<OriginalBoxSlot<T>>>,original:ManuallyDrop<Option<T>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,unique:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,closing:ManuallyDrop<Option<FactoryAuthority>>}
impl<T:Send+'static> Cursor<T>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.boxed.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()});}
  if self.vacant.is_some(){return Ok(RetirementDemand{release_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()});}
  if let Some(original)=self.original.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().unwrap().retirement_birth_bytes(original),depth:2,..Default::default()});}
  let mut demand=if let Some(unique)=self.unique.as_ref(){crate::factory_ticket_demands(unique,body)?}else if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Arc<dyn FactoryRetirement>>(),depth:1,..Default::default()});}else if let Some(closing)=self.closing.as_ref(){closing.demands(body)?}else{return Ok(Default::default());};
  demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"boxed mutation retirement depth overflow"))?;Ok(demand)
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
  if self.boxed.is_some(){let pointer=Box::into_raw(self.boxed.take().unwrap());*self.original=Some(unsafe{pointer.read()});*self.vacant=Some(OriginalBoxSlot{allocation:ManuallyDrop::new(Some(unsafe{Box::from_raw(pointer.cast())}))});return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});}
  if let Some(vacant)=self.vacant.as_mut(){drop(vacant.allocation.take());drop(self.vacant.take());return Ok(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()});}
  let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(original)=self.original.take(){return match self.factory.as_ref().unwrap().retire_owned(original,child){Ok((owner,progress))=>{*self.unique=Some(owner);if !progress.fits(child)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"boxed payload issuer changed its admitted receipt"));}Ok(progress)},Err((error,original))=>{*self.original=Some(original);Err(error)}};}
  if self.unique.is_some(){return crate::close_factory_ticket(&mut self.unique,child).map(|step|step.progress());}
  if self.factory.is_some(){let factory:Arc<dyn FactoryRetirement>=self.factory.take().unwrap();*self.closing=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});}
  if let Some(closing)=self.closing.as_mut(){let step=closing.step(child)?;if closing.terminal_is_empty(){drop(self.closing.take());}return Ok(step.progress());}Ok(Default::default())
 }
}
impl<T:Send+'static> RetireOwned for FactoryBoxedValue<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{boxed:ManuallyDrop::new(Some(self.original)),vacant:ManuallyDrop::new(None),original:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(self.factory)),unique:ManuallyDrop::new(None),closing:ManuallyDrop::new(None)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Cursor<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:Send+'static> RetirementCursor for Cursor<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.step(grant){Ok(progress)=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.boxed.is_none()&&self.vacant.is_none()&&self.original.is_none()&&self.factory.is_none()&&self.unique.is_none()&&self.closing.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<T:Send+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"boxed mutation abandoned original payload or issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.boxed);ManuallyDrop::drop(&mut self.vacant);ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.unique);ManuallyDrop::drop(&mut self.closing);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
