//! 🔗️ Immutable clone sources capture actual original authority and expose sealed projection leases.
use super::{RetainedCloneBinding,RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneProjection,RetainedCloneRef,RetainedCloneStep,RetainedOwnedProjection,RETAINED_CLONE_SOURCE_IDS};
use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}}};
use std::{mem::ManuallyDrop,ptr::NonNull,sync::atomic::Ordering};

pub(super) struct RetainedCloneLeaseOwner{pub(super) id:u64,payload:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
/// 🧷️ Immutable projection leases expose their payload only after unique original transfer.
unsafe impl Sync for RetainedCloneLeaseOwner{}
struct SourcePayload<A:RetireOwned+Sync,B:RetireOwned>{source:ControlledRetirement<SealedShared<A>>,authority:ControlledRetirement<B>}
impl<A:RetireOwned+Sync,B:RetireOwned> ErasedSnapshotRetirement for SourcePayload<A,B>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.source.terminal_is_empty(){self.source.step(grant)}else{self.authority.step(grant)}}
 fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()&&self.authority.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_copy_byte_demand()}else{self.authority.next_copy_byte_demand()}}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_capacity_byte_demand(body)}else{self.authority.next_capacity_byte_demand(body)}}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_release_byte_demand()}else{self.authority.next_release_byte_demand()}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_depth_demand()}else{self.authority.next_depth_demand()}}
}
struct LeaseRetirement(RetainedCloneLeaseOwner);
impl RetirementCursor for LeaseRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        match crate::close_factory_ticket(&mut self.0.payload,grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool {self.0.payload.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.0.payload.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.copy_bytes))}
    fn next_birth_bytes(&self,body:usize)->Option<usize> {self.0.payload.as_ref().map_or(Some(0),|owner|crate::factory_ticket_demands(owner,body).ok().map(|demand|demand.capacity_bytes))}
    fn next_close_byte_demand(&self)->Option<usize> {self.0.payload.as_ref().map_or(Some(0),|owner|crate::factory_ticket_demands(owner,0).ok().map(|demand|demand.release_bytes))}
    fn next_depth_demand(&self)->Result<usize,ValueError> {self.0.payload.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.depth))}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for RetainedCloneLeaseOwner{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(LeaseRetirement(self))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<LeaseRetirement>())}fn controlled_retirement_supported()->bool{true}}
impl Drop for RetainedCloneLeaseOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.payload.is_none(),"original source projection payload remains retained");if self.payload.is_none(){unsafe{ManuallyDrop::drop(&mut self.payload);}}}}


/// 🎟️ Returns either one funded alias-closure turn or the exact original authority after every projection alias has returned.
pub enum RetainedCloneSourceTake<A>{Pending(RetainedCloneProgress),Ready(A,RetainedCloneProgress)}

/// 🔗️ Captures one original authority in a strong-only shared handle and lends a typed projection of it through sealed leases.
pub struct RetainedCloneSource<T:Sync+'static,A:RetireOwned+Sync=T>{owner:ManuallyDrop<Option<SealedShared<A>>>,value:NonNull<T>,lease:Option<RetainedCloneBinding>,owner_close:ManuallyDrop<Option<ControlledRetirement<SealedShared<A>>>>}
/// 🧷️ The projected value lives inside the immutable shared authority that the source or its lease keeps alive.
unsafe impl<T:Sync+'static,A:RetireOwned+Sync> Send for RetainedCloneSource<T,A>{}
/// 🪢️ Shared access exposes only immutable references into the sealed authority.
unsafe impl<T:Sync+'static,A:RetireOwned+Sync> Sync for RetainedCloneSource<T,A>{}

fn identity<A>(value:&A)->&A{value}

fn constructor_capacity<A:RetireOwned+Sync,B:RetireOwned>()->usize{SealedShared::<A>::birth_bytes()+SealedShared::<RetainedCloneLeaseOwner>::birth_bytes()+size_of::<SourcePayload<A,B>>()}

impl<T:Sync+'static,A:RetireOwned+Sync> RetainedCloneSource<T,A>{
 /// 📐️ Measures the alias copy that hands the original authority to its projection lease.
 pub fn constructor_copy_bytes()->usize{size_of::<SealedShared<A>>()}
 /// 📐️ Measures both shared headers and the lease payload scaffold before anything is allocated.
 pub fn constructor_capacity_bytes<B:RetireOwned>()->usize{constructor_capacity::<A,B>()}
 pub fn constructor_demand<B:RetireOwned>()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:Self::constructor_capacity_bytes::<B>(),depth:1}}
 /// 📐️ Measures the constructor of an owned source with a distinct authority.
 pub fn owned_constructor_capacity_bytes<B:RetireOwned>()->usize{Self::constructor_capacity_bytes::<B>()}
 pub fn owned_constructor_demand<B:RetireOwned>()->RetainedCloneBirthDemand{Self::constructor_demand::<B>()}
 /// 📐️ Measures the constructor that borrows a projection from an original authority of another type.
 pub fn borrowed_constructor_capacity_bytes<X:RetireOwned+Sync>()->usize{constructor_capacity::<X,()>()}
 pub fn borrowed_constructor_copy_bytes<X:RetireOwned+Sync>()->usize{size_of::<SealedShared<X>>()}
 pub fn borrowed_constructor_demand<X:RetireOwned+Sync>()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:Self::borrowed_constructor_capacity_bytes::<X>(),depth:1}}
 fn admit_with<B:RetireOwned,F>(authority:A,extra:B,project:F,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,A,B)> where F:for<'source>FnOnce(&'source A)->&'source T{
  if !A::controlled_retirement_supported()||!B::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"source or authority has no controlled typed retirement"),authority,extra));}
  let demand=Self::constructor_demand::<B>();
  if let Err(error)=demand.admit(grant){return Err((error,authority,extra));}
  let copy=Self::constructor_copy_bytes();
  if grant.maximum_copy_bytes<copy{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"source constructor exceeds admitted alias copy"),authority,extra));}
  let birth=match SealedShared::<RetainedCloneLeaseOwner>::prepare_birth(grant){Ok(birth)=>birth,Err(error)=>return Err((error,authority,extra))};
  let owner=match SealedShared::admit(authority,SharedIssuer::owned(),grant){Ok((owner,_))=>owner,Err((error,authority))=>return Err((error,authority,extra))};
  let(alias,_)=owner.try_duplicate(grant).unwrap_or_else(|error|unreachable!("admitted source alias refused: {error}"));
  let value=NonNull::from(project(owner.get()));
  let payload=SourcePayload{source:ControlledRetirement::new(alias).unwrap_or_else(|(error,_)|unreachable!("admitted source alias lost its retirement: {error}")),authority:ControlledRetirement::new(extra).unwrap_or_else(|(error,_)|unreachable!("admitted source authority refused: {error}"))};
  let(lease,_)=birth.materialize(RetainedCloneLeaseOwner{id:RETAINED_CLONE_SOURCE_IDS.fetch_add(1,Ordering::Relaxed),payload:ManuallyDrop::new(Some(Box::new(payload)))},SharedIssuer::owned());
  let address=value.as_ptr()as*const()as usize;
  Ok((Self{owner:ManuallyDrop::new(Some(owner)),value,lease:Some(RetainedCloneBinding::new(lease,RetainedCloneProjection{parent:address,address,discriminator:0})),owner_close:ManuallyDrop::new(None)},RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
 }
 pub fn borrow(&self)->RetainedCloneRef<'_,T>{self.try_borrow().unwrap_or_else(|error|panic!("retained source is closed: {error}"))}
 /// 🛂️ Borrows only a live original authority and lease without allocating or transferring custody.
 pub fn try_borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{
  if self.owner.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained source original owner is absent"));}
  let binding=self.lease.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained source original lease is absent"))?;
  let lease=binding.lease.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained source original lease is closed"))?;
  Ok(RetainedCloneRef{value:unsafe{self.value.as_ref()},lease,projection:binding.projection})
 }
 pub fn project_owned<U:?Sized+Sync,F>(&self,discriminator:usize,project:F,grant:RetainedCloneGrant)->Result<(RetainedOwnedProjection<U>,RetainedCloneProgress),ValueError> where F:for<'source>FnOnce(&'source T)->&'source U{unsafe{RetainedOwnedProjection::from_source(self.try_borrow()?.project(discriminator,project),grant)}}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(owner)=self.owner_close.as_ref(){owner.next_copy_byte_demand()}else if self.owner.is_some(){Ok(0)}else{RetainedCloneBinding::copy_demand(&self.lease)}}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if let Some(owner)=self.owner_close.as_ref(){owner.next_capacity_byte_demand(body)}else if self.owner.is_some(){Ok(0)}else{RetainedCloneBinding::capacity_demand(&self.lease,body)}}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(owner)=self.owner_close.as_ref(){owner.next_release_byte_demand()}else if self.owner.is_some(){Ok(0)}else{RetainedCloneBinding::release_demand(&self.lease)}}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(owner)=self.owner_close.as_ref(){owner.next_depth_demand()}else if self.owner.is_some(){Ok(1)}else{RetainedCloneBinding::depth_demand(&self.lease)}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.owner_close.is_none(){if let Some(owner)=self.owner.take(){*self.owner_close=Some(ControlledRetirement::new(owner).unwrap_or_else(|(error,_)|unreachable!("sealed source authority lost its retirement: {error}")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}}
  if let Some(owner)=self.owner_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){*self.owner_close=None;}return Ok(step);}
  RetainedCloneBinding::close_one(&mut self.lease,grant)
 }
 pub fn next_take_copy_byte_demand(&self)->Result<usize,ValueError>{if self.lease.is_some(){RetainedCloneBinding::copy_demand(&self.lease)}else{Ok(size_of::<A>())}}
 pub fn next_take_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if self.lease.is_some(){RetainedCloneBinding::capacity_demand(&self.lease,body)}else{Ok(0)}}
 pub fn next_take_release_byte_demand(&self)->Result<usize,ValueError>{if self.lease.is_some(){RetainedCloneBinding::release_demand(&self.lease)}else{Ok(SealedShared::<A>::birth_bytes())}}
 pub fn next_take_depth_demand(&self)->Result<usize,ValueError>{if self.lease.is_some(){RetainedCloneBinding::depth_demand(&self.lease)}else{Ok(1)}}
 /// 📤️ Closes the source's own projection alias, then returns the exact original authority once no other alias remains.
 pub fn take_authority(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneSourceTake<A>,ValueError>{
  if self.owner_close.is_some()||self.owner.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"retained source original authority is already closing or transferred"));}
  if self.lease.is_some(){return RetainedCloneBinding::close_one(&mut self.lease,grant).map(|step|RetainedCloneSourceTake::Pending(step.progress()));}
  let owner=self.owner.take().unwrap_or_else(||unreachable!("source authority presence was checked"));
  match owner.try_unwrap(grant){Ok((authority,progress))=>Ok(RetainedCloneSourceTake::Ready(authority,progress)),Err((error,owner))=>{*self.owner=Some(owner);Err(error)}}
 }
 pub fn terminal_is_empty(&self)->bool{self.owner.is_none()&&self.owner_close.is_none()&&self.lease.is_none()}
}
impl<T:Sync+'static,A:RetireOwned+Sync> Drop for RetainedCloneSource<T,A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"source must finish full granted ownership closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);ManuallyDrop::drop(&mut self.owner_close);}}}}

struct SourceRetirement<T:Sync+'static,A:RetireOwned+Sync>(ManuallyDrop<RetainedCloneSource<T,A>>);
impl<T:Sync+'static,A:RetireOwned+Sync> RetirementCursor for SourceRetirement<T,A>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.next_close_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.next_close_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.0.next_close_depth_demand()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Sync+'static,A:RetireOwned+Sync> Drop for SourceRetirement<T,A>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty(),"source cursor abandoned its original authority");if self.0.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.0);}}}}
impl<T:Sync+'static,A:RetireOwned+Sync> RetireOwned for RetainedCloneSource<T,A>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SourceRetirement(ManuallyDrop::new(self)))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SourceRetirement<T,A>>())}fn controlled_retirement_supported()->bool{true}}

impl<T:RetireOwned+Sync> RetainedCloneSource<T,T>{
 /// 🎟️ Moves one original authority and its projection into sealed custody only after one complete constructor grant.
 pub fn admit_borrowed<A:RetireOwned+Sync,F>(authority:A,project:F,grant:RetainedCloneGrant)->Result<(RetainedCloneSource<T,A>,RetainedCloneProgress),(ValueError,A)> where F:for<'source>FnOnce(&'source A)->&'source T{
  RetainedCloneSource::<T,A>::admit_with(authority,(),project,grant).map_err(|(error,authority,())|(error,authority))
 }
}
impl<A:RetireOwned+Sync> RetainedCloneSource<A,A>{
 /// 🎟️ Moves the original owner and a distinct authority into sealed custody only after one complete constructor grant.
 pub fn admit_owned<B:RetireOwned>(owner:A,authority:B,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,A,B)>{Self::admit_with(owner,authority,identity,grant)}
}

#[cfg(test)]
pub(crate) struct FixtureSource<T:RetireOwned+Sync>(RetainedCloneSource<T>);
#[cfg(test)]
impl<T:RetireOwned+Sync> std::ops::Deref for FixtureSource<T>{type Target=RetainedCloneSource<T>;fn deref(&self)->&Self::Target{&self.0}}
#[cfg(test)]
impl<T:RetireOwned+Sync> FixtureSource<T>{fn drain(&mut self){for _ in 0..100000{if self.0.terminal_is_empty(){return;}let copy=self.0.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.0.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:self.0.next_close_release_byte_demand().unwrap(),maximum_depth:self.0.next_close_depth_demand().unwrap()};let(step,heap)=crate::value::observe_retirement_allocations(||self.0.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}panic!("original source fixture did not physically close");}}
#[cfg(test)]
impl<T:RetireOwned+Sync> Drop for FixtureSource<T>{fn drop(&mut self){if !std::thread::panicking(){self.drain();}}}
#[cfg(test)]
impl<T:RetireOwned+Sync> RetainedCloneSource<T>{pub(crate) fn from_owner(owner:T)->FixtureSource<T>{let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:Self::constructor_copy_bytes(),maximum_capacity_bytes:Self::owned_constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};FixtureSource(Self::admit_owned(owner,(),grant).unwrap_or_else(|_|panic!("original fixture source admission")).0)}}
