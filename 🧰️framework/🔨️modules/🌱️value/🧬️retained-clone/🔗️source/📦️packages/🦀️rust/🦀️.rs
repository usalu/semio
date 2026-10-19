//! 🔗️ Immutable original source/ref/binding/projection custody owns every native alias.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement,factory_ticket_demands,close_factory_ticket};
pub use semio_framework_value_shared_issuance as shared_issuance;
use semio_framework_value_retirement::RetireOwned;
use std::{mem::ManuallyDrop,sync::atomic::AtomicU64};
static RETAINED_CLONE_SOURCE_IDS:AtomicU64=AtomicU64::new(1);
#[path="../../🦀️.rs"]
mod source_custody;
pub use source_custody::{RetainedCloneSource,RetainedCloneSourceTake};
use source_custody::RetainedCloneLeaseOwner;
#[path="../../../🔗️projection/🦀️.rs"]
mod owned_projection;
pub use owned_projection::RetainedOwnedProjection;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RetainedCloneProjection {
    parent: usize,
    address: usize,
    discriminator: usize,
}

pub struct RetainedCloneRef<'a,T:?Sized>{value:&'a T,lease:&'a crate::shared_issuance::SealedShared<RetainedCloneLeaseOwner>,projection:RetainedCloneProjection}
impl<'a,T:?Sized> Copy for RetainedCloneRef<'a,T>{}
impl<'a,T:?Sized> Clone for RetainedCloneRef<'a,T>{fn clone(&self)->Self{*self}}
impl<'a,T:?Sized> RetainedCloneRef<'a,T>{
 pub fn get(self)->&'a T{self.value}
 /// 🪪️ Observes the already issued sealed source without duplicating a lease or granting authority.
 pub fn source_identity(self)->u64{self.lease.get().id}
 pub fn project<U:?Sized,F>(self,discriminator:usize,project:F)->RetainedCloneRef<'a,U>where F:for<'source>FnOnce(&'source T)->&'source U{let value=project(self.value);RetainedCloneRef{value,lease:self.lease,projection:RetainedCloneProjection{parent:self.projection.address,address:value as*const U as*const()as usize,discriminator}}}
 pub fn binding_copy_bytes(self)->usize{0}
 /// 🪪️ Checks the original issued source and exact projected path without granting a new alias.
 pub fn check_binding(self,binding:&Option<RetainedCloneBinding>)->Result<bool,crate::ValueError>{self.binding_demand(binding).map(|demand|demand.is_none())}
 /// 🪪️ Validates one original lease and projected path without exposing private custody.
 pub fn matches_binding(self,binding:&RetainedCloneBinding)->bool{binding.lease.as_ref().is_some_and(|lease|lease.get().id==self.lease.get().id)&&binding.projection==self.projection}
 pub fn binding_demand(self,binding:&Option<RetainedCloneBinding>)->Result<Option<crate::RetirementDemand>,crate::ValueError>{if let Some(expected)=binding{if expected.lease.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"projected source binding already closing"))?.get().id!=self.lease.get().id||expected.projection!=self.projection{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original clone source or projected path changed"));}Ok(None)}else{Ok(Some(crate::RetirementDemand{copy_bytes:self.binding_copy_bytes(),depth:1,..Default::default()}))}}
 pub fn bind(self,binding:&mut Option<RetainedCloneBinding>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,crate::ValueError>{
  if let Some(expected)=binding{if expected.lease.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"projected source binding already closing"))?.get().id!=self.lease.get().id||expected.projection!=self.projection{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original clone source or projected path changed"));}return Ok(None);}
  if grant.maximum_items==0||grant.maximum_copy_bytes<self.binding_copy_bytes()||grant.maximum_depth==0{return Ok(Some(Default::default()));}
  let(alias,progress)=self.lease.try_duplicate(grant)?;*binding=Some(RetainedCloneBinding::new(alias,self.projection));Ok(Some(progress))
 }
}
pub struct RetainedCloneBinding{lease:ManuallyDrop<Option<crate::shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>,close:ManuallyDrop<Option<crate::retirement::controlled::ControlledRetirement<crate::shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>>,projection:RetainedCloneProjection}
impl RetainedCloneBinding{
 pub fn alias_copy_bytes()->usize{0}
 fn new(lease:crate::shared_issuance::SealedShared<RetainedCloneLeaseOwner>,projection:RetainedCloneProjection)->Self{Self{lease:ManuallyDrop::new(Some(lease)),close:ManuallyDrop::new(None),projection}}
 pub fn close_one(binding:&mut Option<Self>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{
  let Some(owner)=binding.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if owner.close.is_none(){if grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(lease)=owner.lease.take(){*owner.close=Some(crate::retirement::controlled::ControlledRetirement::new(lease).unwrap_or_else(|_|unreachable!("sealed projection retains its typed issuer")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}}
  let close=owner.close.as_mut().unwrap();let step=close.step(grant)?;if close.terminal_is_empty(){*owner.close=None;*binding=None;}Ok(step)
 }
 pub fn copy_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
 pub fn capacity_demand(binding:&Option<Self>,body:usize)->Result<usize,crate::ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.next_capacity_byte_demand(body))}
 pub fn release_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.next_release_byte_demand())}
 pub fn depth_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().map_or(Ok(0),|owner|owner.close.as_ref().map_or(Ok(1),|owner|owner.next_depth_demand()))}
 pub fn close_demand(binding:&Option<Self>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{binding.as_ref().map_or(Ok(Default::default()),|owner|owner.close.as_ref().map_or(Ok(crate::RetirementDemand{depth:1,..Default::default()}),|owner|owner.selected_demand(body)))}
 pub fn terminal_is_empty(&self)->bool{self.lease.is_none()&&self.close.is_none()}
}
impl Drop for RetainedCloneBinding{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"projected original source requires genuine alias closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.lease);ManuallyDrop::drop(&mut self.close);}}}}

pub struct RetainedCloneBorrowAuthority<A:RetireOwned+Sync>{source:RetainedCloneSource<A>}
impl<A:RetireOwned+Sync> RetainedCloneBorrowAuthority<A>{
 pub fn constructor_capacity_bytes()->usize{RetainedCloneSource::<A>::owned_constructor_capacity_bytes::<()>()}
 pub fn constructor_copy_bytes()->usize{RetainedCloneSource::<A>::constructor_copy_bytes()}
 pub fn admit(authority:A,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,A)>{RetainedCloneSource::admit_owned(authority,(),grant).map(|(source,progress)|(Self{source},progress)).map_err(|(error,authority,())|(error,authority))}
 pub fn borrow<T:?Sized,F>(&self,discriminator:usize,project:F)->RetainedCloneRef<'_,T>where F:for<'source>FnOnce(&'source A)->&'source T{self.source.borrow().project(discriminator,project)}
 /// 🧾️ Borrows the one actual source close quote with every independent physical currency.
 pub fn next_close_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.source.next_close_demand(body)}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_copy_byte_demand()}
 pub fn next_close_capacity_byte_demand(&self,work:usize)->Result<usize,crate::ValueError>{self.source.next_close_capacity_byte_demand(work)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_release_byte_demand()}
 pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_depth_demand()}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{self.source.close_step(grant)}
 pub fn next_take_copy_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_take_copy_byte_demand()}
 pub fn next_take_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{self.source.next_take_capacity_byte_demand(body)}
 pub fn next_take_release_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_take_release_byte_demand()}
 pub fn next_take_depth_demand(&self)->Result<usize,crate::ValueError>{self.source.next_take_depth_demand()}
 pub fn take_authority(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneSourceTake<A>,crate::ValueError>{self.source.take_authority(grant)}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
#[path="../../🧪️tests/🔐️custody/🦀️.rs"]
mod original_sealed_custody_tests;

#[cfg(test)]
#[path="../../🧪️tests/🎟️header/🦀️.rs"]
mod original_zero_header_tests;
