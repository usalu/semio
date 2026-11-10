//! 🌱️ Initializes real read-frame backing before capturing the original payload.
use super::*;
use std::{alloc::{alloc,dealloc,Layout},mem::MaybeUninit,ptr::NonNull};
pub(super) struct OriginalReadBirth<T>{backing:Option<NonNull<u8>>,layout:Layout,initialized:usize,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,work:OriginalWorkBindingIdentity,closing:bool}
/// 🚚️ The typed constructor admits only Send originals; partial backing contains no payload.
unsafe impl<T:Send> Send for OriginalReadBirth<T>{}
/// 🧷️ Shared queries inspect frozen metadata and the immutable factory; backing mutation is exclusive.
unsafe impl<T:Sync> Sync for OriginalReadBirth<T>{}
impl<T> OriginalReadBirth<T>{
 fn authenticate(&self,effect:&RetainedAllocationEffect<'_>)->Result<(),ValueError>{if !self.work.matches_effect(effect){return Err(fault("original read birth changed its actual Source or Work"))}Ok(())}
 pub(super) fn matches_factory(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->bool{!self.closing&&self.factory.as_ref().is_some_and(|actual|Arc::ptr_eq(actual,factory))}
 pub(super) fn issuer(&self)->Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>{self.factory.as_ref()}
 pub(super) fn terminal_is_empty(&self)->bool{self.factory.is_none()&&self.backing.is_none()}
 pub(super) fn return_demand(&self)->RetirementDemand{if !self.closing{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}else if self.factory.is_some(){RetirementDemand{copy_bytes:2*size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>(),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:size_of::<Option<NonNull<u8>>>()+size_of::<Option<Self>>(),release_bytes:usize::from(self.backing.is_some())*self.layout.size(),depth:1,..Default::default()}}}
 pub(super) fn return_into<'r>(&mut self,output:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{
  self.authenticate(effect)?;let d=self.return_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.closing&&self.factory.is_some()&&output.is_some(){return Err(fault("original read birth issuer return is occupied"))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(fault("original read birth cancellation exceeded Work").into())))}
  if !self.closing{self.closing=true;}else if self.factory.is_some(){*output=self.factory.take();}else if let Some(backing)=self.backing.take(){unsafe{dealloc(backing.as_ptr(),self.layout)}}
  let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()};Ok(reply.complete(Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})))
 }
}
impl<T:Send+Sync+'static> OriginalReadBirth<T>{
 const BLOCK:usize=1024;
 pub(super) fn allocation_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Self>>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>()+size_of::<usize>()+size_of::<Option<usize>>(),capacity_bytes:size_of::<RetainedOriginalRead<T>>(),depth:1,..Default::default()}}
 pub(super) fn start_into<'r>(target:&mut Option<Self>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{
  if target.is_some()||effect.actual().is_some()||effect.work_source_turn().is_none(){return Err(fault("original read birth inputs or actual Source changed"))}let d=Self::allocation_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(fault("original read allocation exceeded Work").into())))}*effect.actual()=Some(0);let layout=Layout::new::<RetainedOriginalRead<T>>();let Some(backing)=NonNull::new(unsafe{alloc(layout)})else{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original read frame allocation failed").into())))};*effect.actual()=Some(layout.size());
  *target=Some(Self{backing:Some(backing),layout,initialized:0,factory:ManuallyDrop::new(Some(Arc::clone(factory))),work:OriginalWorkBindingIdentity::from_effect(effect),closing:false});
  Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))))
 }
 pub(super) fn next_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault("original read birth is returning"))}Ok(RetirementDemand{copy_bytes:if self.initialized<self.layout.size(){Self::BLOCK.min(self.layout.size()-self.initialized)+size_of::<usize>()}else{2*size_of::<Option<T>>()+2*size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>()+2*size_of::<Option<FactoryOwnedSourceAuthority<T>>>()+size_of::<Option<OriginalWorkBindingIdentity>>()+size_of::<Option<Box<dyn RetainedOriginalReadOwner<T>>>>()+size_of::<Option<Self>>()},depth:1,..Default::default()})}
 pub(super) fn advance_into<'r>(&mut self,original:&mut Option<T>,output:&mut Option<Box<dyn RetainedOriginalReadOwner<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{
  self.authenticate(effect)?;if output.is_some()||original.is_none()||effect.actual().is_some(){return Err(fault("original read birth resident slots changed"))}let d=self.next_demand()?;if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let backing=self.backing.ok_or_else(||fault("original read backing absent"))?;
  if !effect.performed(1)?{return Ok(reply.complete(Err(fault("original read initialization exceeded Work").into())))}
  if self.initialized<self.layout.size(){let count=Self::BLOCK.min(self.layout.size()-self.initialized);let template=&RetainedOriginalRead::<T>::TEMPLATE;unsafe{std::ptr::copy_nonoverlapping((template as*const ManuallyDrop<RetainedOriginalRead<T>>).cast::<MaybeUninit<u8>>().add(self.initialized),backing.as_ptr().cast::<MaybeUninit<u8>>().add(self.initialized),count)}self.initialized+=count;}
  else{let owner=backing.cast::<RetainedOriginalRead<T>>();let value=original.take().expect("funded original read payload");let factory=self.factory.take().expect("funded original read issuer");unsafe{std::ptr::addr_of_mut!((*owner.as_ptr()).staged).write(ManuallyDrop::new(Some(FactoryOwnedSourceAuthority::new(value,factory))));std::ptr::addr_of_mut!((*owner.as_ptr()).work).write(Some(self.work));*output=Some(Box::from_raw(owner.as_ptr()));}self.backing=None;}
  Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))
 }
}
impl<T> Drop for OriginalReadBirth<T>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.terminal_is_empty(),"original read birth retains backing or captured issuer");unsafe{ManuallyDrop::drop(&mut self.factory)}}}
