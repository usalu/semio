//! 🔗️ Typed immutable source authority retains every original owner through full granted closure.
use super::{RetainedCloneBinding,RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneProjection,RetainedCloneRef,RetainedCloneStep,RetainedOwnedProjection,RETAINED_CLONE_SOURCE_IDS};
use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement,shared::SharedControlledRetirement}};
use std::{mem::ManuallyDrop,sync::{Arc,atomic::Ordering}};

pub(super) struct RetainedCloneLeaseOwner {pub(super) id:u64,payload:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
/// 🧷️ Shared metadata exposes only immutable identity; its payload is accessed after unique Arc transfer.
unsafe impl Sync for RetainedCloneLeaseOwner {}

struct SourcePayload<T:RetireOwned+Sync,A:RetireOwned> {source:SharedControlledRetirement<T>,authority:ControlledRetirement<A>}
impl<T:RetireOwned+Sync,A:RetireOwned> ErasedSnapshotRetirement for SourcePayload<T,A> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {if !self.source.terminal_is_empty(){self.source.step(grant)}else{self.authority.step(grant)}}
    fn terminal_is_empty(&self)->bool {self.source.terminal_is_empty()&&self.authority.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.source.terminal_is_empty(){self.source.next_copy_byte_demand()}else{self.authority.next_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {if !self.source.terminal_is_empty(){self.source.next_capacity_byte_demand(body)}else{self.authority.next_capacity_byte_demand(body)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.source.terminal_is_empty(){self.source.next_release_byte_demand()}else{self.authority.next_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.source.terminal_is_empty(){self.source.next_depth_demand()}else{self.authority.next_depth_demand()}}
}

struct LeaseRetirement(RetainedCloneLeaseOwner);
impl RetirementCursor for LeaseRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        match crate::close_factory_ticket(&mut self.0.payload,grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool {self.0.payload.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.0.payload.as_ref().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
    fn next_birth_bytes(&self,body:usize)->Option<usize> {self.0.payload.as_ref().map_or(Some(0),|owner|crate::factory_ticket_demands(owner,body).ok().map(|demand|demand.capacity_bytes))}
    fn next_close_byte_demand(&self)->Option<usize> {self.0.payload.as_ref().map_or(Some(0),|owner|crate::factory_ticket_demands(owner,0).ok().map(|demand|demand.release_bytes))}
    fn next_depth_demand(&self)->Result<usize,ValueError> {self.0.payload.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.depth))}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for RetainedCloneLeaseOwner {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(LeaseRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<LeaseRetirement>())}
    fn controlled_retirement_supported()->bool {true}
}
impl Drop for RetainedCloneLeaseOwner {fn drop(&mut self){assert!(std::thread::panicking()||self.payload.is_none(),"retained source metadata abandoned original typed authority");if self.payload.is_none(){unsafe{ManuallyDrop::drop(&mut self.payload);}}}}


pub struct RetainedCloneSource<T:RetireOwned+Sync> {owner:ManuallyDrop<Option<Arc<T>>>,lease:Option<RetainedCloneBinding>,owner_close:ManuallyDrop<Option<SharedControlledRetirement<T>>>}
impl<T:RetireOwned+Sync> RetainedCloneSource<T> {
    pub fn constructor_capacity_bytes<A:RetireOwned>()->usize {crate::retirement::shared::arc_bytes::<RetainedCloneLeaseOwner>()+size_of::<SourcePayload<T,A>>()}
    /// 📐️ Measures the original owner's Arc and source lease before either is allocated.
    pub fn owned_constructor_capacity_bytes<A:RetireOwned>()->usize {crate::retirement::shared::arc_bytes::<T>()+Self::constructor_capacity_bytes::<A>()}
    pub fn owned_constructor_demand<A:RetireOwned>()->RetainedCloneBirthDemand {RetainedCloneBirthDemand{capacity_bytes:Self::owned_constructor_capacity_bytes::<A>(),depth:1}}
    /// 🎟️ Moves original owned values only after one complete constructor capacity grant.
    pub fn admit_owned<A:RetireOwned>(owner:T,authority:A,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,T,A)> {
        if !T::controlled_retirement_supported()||!A::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"owned source or authority has no controlled typed retirement"),owner,authority));}
        if let Err(error)=Self::owned_constructor_demand::<A>().admit(grant){return Err((error,owner,authority));}
        let arc_bytes=crate::retirement::shared::arc_bytes::<T>();
        let source_grant=RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-arc_bytes,..grant};
        match Self::admit(Arc::new(owner),authority,source_grant){
            Ok((source,mut progress))=>{progress.retained_capacity_bytes+=arc_bytes;Ok((source,progress))},
            Err((error,owner,authority))=>Err((error,Arc::try_unwrap(owner).unwrap_or_else(|_|unreachable!("unadmitted owned source has no aliases")),authority)),
        }
    }
    pub fn admit<A:RetireOwned>(owner:Arc<T>,authority:A,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,Arc<T>,A)> {
        if !T::controlled_retirement_supported()||!A::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"source or authority has no controlled typed retirement"),owner,authority));}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"source constructor requires one admitted item"),owner,authority));}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"source constructor requires admitted depth"),owner,authority));}
        let bytes=Self::constructor_capacity_bytes::<A>();
        if bytes>grant.maximum_capacity_bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"source constructor exceeds admitted capacity"),owner,authority));}
        let payload=SourcePayload{source:SharedControlledRetirement::lease(Arc::clone(&owner)),authority:ControlledRetirement::new(authority).unwrap_or_else(|(error,_)|panic!("admitted source authority refused: {error}"))};
        let lease=Arc::new(RetainedCloneLeaseOwner{id:RETAINED_CLONE_SOURCE_IDS.fetch_add(1,Ordering::Relaxed),payload:ManuallyDrop::new(Some(Box::new(payload)))});
        let address=owner.as_ref()as*const T as usize;
        Ok((Self{owner:ManuallyDrop::new(Some(owner)),lease:Some(RetainedCloneBinding::new(lease,RetainedCloneProjection{parent:address,address,discriminator:0})),owner_close:ManuallyDrop::new(None)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
    pub fn borrow(&self)->RetainedCloneRef<'_,T> {let owner=self.owner.as_ref().expect("retained source is closed");let binding=self.lease.as_ref().expect("retained source lease is closed");RetainedCloneRef{value:owner.as_ref(),lease:binding.lease.as_ref().unwrap(),projection:binding.projection}}
    pub fn project_owned<U:?Sized+Sync,F>(&self,discriminator:usize,project:F)->RetainedOwnedProjection<U> where F:for<'source>FnOnce(&'source T)->&'source U {unsafe{RetainedOwnedProjection::from_source(self.borrow().project(discriminator,project))}}
    pub fn take_owner(&mut self)->Option<Arc<T>> {if self.owner_close.is_some(){None}else{self.owner.take()}}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {if let Some(owner)=self.owner_close.as_ref(){owner.next_copy_byte_demand()}else if self.owner.is_some(){Ok(0)}else{RetainedCloneBinding::copy_demand(&self.lease)}}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {if let Some(owner)=self.owner_close.as_ref(){owner.next_capacity_byte_demand(body)}else if self.owner.is_some(){Ok(0)}else{RetainedCloneBinding::capacity_demand(&self.lease,body)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {if let Some(owner)=self.owner_close.as_ref(){owner.next_release_byte_demand()}else if self.owner.is_some(){Ok(crate::retirement::shared::arc_bytes::<T>())}else{RetainedCloneBinding::release_demand(&self.lease)}}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {if let Some(owner)=self.owner_close.as_ref(){owner.next_depth_demand()}else if self.owner.is_some(){Ok(1)}else{RetainedCloneBinding::depth_demand(&self.lease)}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth<self.next_close_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"source close requires admitted depth"));}
        if self.owner.is_some()&&grant.maximum_release_bytes<crate::retirement::shared::arc_bytes::<T>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.owner_close.is_none(){if let Some(owner)=self.owner.take(){*self.owner_close=Some(SharedControlledRetirement::lease(owner));}}
        if let Some(owner)=self.owner_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){*self.owner_close=None;}return Ok(step);}
        RetainedCloneBinding::close_one(&mut self.lease,grant)
    }
    pub fn terminal_is_empty(&self)->bool {self.owner.is_none()&&self.owner_close.is_none()&&self.lease.is_none()}
}
impl<T:RetireOwned+Sync> Drop for RetainedCloneSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"source must finish full granted ownership closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);ManuallyDrop::drop(&mut self.owner_close);}}}}

#[cfg(test)]
pub(crate) struct FixtureSource<T:RetireOwned+Sync>(RetainedCloneSource<T>);
#[cfg(test)]
impl<T:RetireOwned+Sync> std::ops::Deref for FixtureSource<T>{type Target=RetainedCloneSource<T>;fn deref(&self)->&Self::Target{&self.0}}
#[cfg(test)]
impl<T:RetireOwned+Sync> FixtureSource<T>{pub(crate) fn into_owner(mut self)->Arc<T>{let owner=self.0.take_owner().unwrap();self.drain();owner}fn drain(&mut self){for _ in 0..100000{if self.0.terminal_is_empty(){return;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:self.0.next_close_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:self.0.next_close_release_byte_demand().unwrap(),maximum_depth:self.0.next_close_depth_demand().unwrap()};let(step,heap)=crate::value::observe_retirement_allocations(||self.0.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}panic!("fixture source custody did not close");}}
#[cfg(test)]
impl<T:RetireOwned+Sync> Drop for FixtureSource<T>{fn drop(&mut self){if !std::thread::panicking(){self.drain();}}}
#[cfg(test)]
impl<T:RetireOwned+Sync> RetainedCloneSource<T>{pub(crate) fn from_owner(owner:T)->FixtureSource<T>{Self::fixture_from_authority(Arc::new(owner),())}pub(crate) fn fixture_from_authority<A:RetireOwned>(owner:Arc<T>,authority:A)->FixtureSource<T>{let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:Self::constructor_capacity_bytes::<A>(),maximum_depth:1,..Default::default()};FixtureSource(Self::admit(owner,authority,grant).unwrap_or_else(|(error,_,_)|panic!("typed source fixture admission: {error}")).0)}}
