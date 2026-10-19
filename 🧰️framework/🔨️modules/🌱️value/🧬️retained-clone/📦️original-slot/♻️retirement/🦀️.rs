//! ♻️ Original slot retirement returns its immutable source before paying the captured payload issuer.
use super::*;
use crate::{retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::RetainedCloneClose,FactoryAuthority,ErasedSnapshotRetirement};
struct OriginalSlotRetirement<T:RetireOwned+Sync>{slot:ManuallyDrop<Option<RetainedOriginalSlot<T>>>,payload:ManuallyDrop<Option<T>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,intrinsic:RetainedCloneClose,factory_close:Option<FactoryAuthority>}
impl<T:RetireOwned+Sync> OriginalSlotRetirement<T>{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(slot)=self.slot.as_ref(){return if slot.has_read(){slot.return_demands(slot.key(),body)}else{Ok(RetirementDemand{copy_bytes:size_of::<RetainedOriginalSlot<T>>()+size_of::<T>(),depth:1,..Default::default()})};}
  if let Some(active)=self.active.as_ref(){return crate::factory_ticket_demands(active,body);}
  if !self.intrinsic.is_empty(){return self.intrinsic.next_owner_demands::<T>(false,&None,body);}
  if self.payload.is_some(){return if self.factory.is_some(){Ok(RetirementDemand{capacity_bytes:crate::FactoryOwnedRetirement::<T>::constructor_capacity_bytes(),depth:1,..Default::default()})}else{self.intrinsic.next_owner_demands::<T>(true,&None,body)};}
  if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<FactoryAuthority>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(),depth:1,..Default::default()});}
  self.factory_close.as_ref().map_or(Ok(Default::default()),|factory|factory.demands(body))
 }
 fn step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(slot)=self.slot.as_mut(){if slot.has_read(){return slot.return_read(slot.key(),&mut self.factory,g);}let original=self.slot.take().unwrap().try_into_original().unwrap_or_else(|_|unreachable!("quoted original slot returned every immutable reader"));*self.payload=Some(original);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if self.active.is_some(){return crate::close_factory_ticket(&mut self.active,g);}
  if !self.intrinsic.is_empty(){return self.intrinsic.step_granted(g);}
  if self.payload.is_some(){if let Some(factory)=self.factory.as_ref(){let Some((active,p))=crate::FactoryOwnedRetirement::<T>::admit_original(&mut self.payload,factory,g)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.active=Some(active);return Ok(RetainedCloneStep::Progress(p));}if let Some(step)=self.intrinsic.begin_granted(&mut self.payload,g)?{return Ok(step);}}
  if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(factory)=self.factory_close.as_mut(){let step=factory.step(g)?;if factory.terminal_is_empty(){self.factory_close=None;}return Ok(step);}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn empty(&self)->bool{self.slot.is_none()&&self.payload.is_none()&&self.factory.is_none()&&self.active.is_none()&&self.intrinsic.is_empty()&&self.factory_close.is_none()}
}
impl<T:RetireOwned+Sync> RetireOwned for RetainedOriginalSlot<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OriginalSlotRetirement{slot:ManuallyDrop::new(Some(self)),payload:ManuallyDrop::new(None),factory:ManuallyDrop::new(None),active:ManuallyDrop::new(None),intrinsic:Default::default(),factory_close:None})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalSlotRetirement<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:RetireOwned+Sync> RetirementCursor for OriginalSlotRetirement<T>{fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{match self.step(g){Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}fn terminal_is_empty(&self)->bool{self.empty()}fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}fn next_birth_bytes(&self,b:usize)->Option<usize>{self.demand(b).ok().map(|d|d.capacity_bytes)}fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}fn terminal_release_bytes(&self)->Option<usize>{self.empty().then_some(size_of::<Self>())}}
impl<T:RetireOwned+Sync> Drop for OriginalSlotRetirement<T>{fn drop(&mut self){assert!(self.empty()||std::thread::panicking(),"original slot retirement dropped before same issuer payload closure");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.slot);ManuallyDrop::drop(&mut self.payload);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.active);}}}}
