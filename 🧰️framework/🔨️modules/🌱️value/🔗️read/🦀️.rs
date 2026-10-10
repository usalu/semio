//! 🔗️ Typed fixed read leases retain original roots through independently admitted ownership closure.

use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
use std::{cell::UnsafeCell,mem::ManuallyDrop,sync::{Arc,atomic::{AtomicBool,AtomicU64,AtomicUsize,Ordering}}};

#[path="🪪️authority/🦀️.rs"]
mod authority;
pub use authority::ReadAuthority;

#[path="🪪️lease/🦀️.rs"]
mod lease;
pub use lease::ErasedReadLease;
#[path="🧬️clone/🦀️.rs"] mod original_clone;
pub use original_clone::{OriginalReadSource,OriginalErasedReadSource};

pub const READ_LEASE_CAPACITY:usize=1024;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ReadLeaseId {pub index:u16,pub generation:u64}

struct Slot<T> {generation:u64,owner:Arc<T>,returned:bool}
struct State<T> {
    slots:Option<Box<[Option<Slot<T>>;READ_LEASE_CAPACITY]>>,
    occupied:[u64;READ_LEASE_CAPACITY/64],free:[u16;READ_LEASE_CAPACITY],free_read:usize,free_len:usize,generation:u64,cursor:usize,
}
impl<T> State<T> {
    fn contains(&self,id:ReadLeaseId)->bool {self.slots.as_ref().and_then(|slots|slots.get(id.index as usize)).and_then(Option::as_ref).is_some_and(|slot|slot.generation==id.generation)}
    fn next_index(&self)->Option<usize> {
        (0..=self.occupied.len()).find_map(|offset|{let word=(self.cursor/64+offset)%self.occupied.len();let mut occupied=self.occupied[word];if offset==0 {occupied&=u64::MAX<<(self.cursor%64);}else if offset==self.occupied.len(){occupied&=(1u64<<(self.cursor%64))-1;}(occupied!=0).then(||word*64+occupied.trailing_zeros()as usize)})
    }
    fn take(&mut self,index:usize)->Slot<T> {
        let slot=self.slots.as_mut().unwrap()[index].take().unwrap();self.occupied[index/64]&=!(1u64<<(index%64));let write=(self.free_read+self.free_len)%READ_LEASE_CAPACITY;self.free[write]=index as u16;self.free_len+=1;slot
    }
}

struct StateLock<T> {held:AtomicBool,poisoned:AtomicBool,state:UnsafeCell<State<T>>}
/// 🔒️ Acquire and Release guards exclusively borrow typed Send state without platform mutex allocation.
unsafe impl<T:Send+Sync> Sync for StateLock<T> {}
impl<T> StateLock<T> {
    fn try_lock(&self)->Result<StateGuard<'_,T>,ValueError> {
        if self.held.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry state is busy"));}
        let guard=StateGuard {lock:self,panicking:std::thread::panicking(),not_send:std::marker::PhantomData};
        if self.poisoned.load(Ordering::Relaxed){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"read registry state is poisoned"));}Ok(guard)
    }
}
struct StateGuard<'a,T> {lock:&'a StateLock<T>,panicking:bool,not_send:std::marker::PhantomData<*mut()>}
impl<T> std::ops::Deref for StateGuard<'_,T> {type Target=State<T>;fn deref(&self)->&Self::Target {unsafe{&*self.lock.state.get()}}}
impl<T> std::ops::DerefMut for StateGuard<'_,T> {fn deref_mut(&mut self)->&mut Self::Target {unsafe{&mut*self.lock.state.get()}}}
impl<T> Drop for StateGuard<'_,T> {fn drop(&mut self){if !self.panicking&&std::thread::panicking(){self.lock.poisoned.store(true,Ordering::Relaxed);}self.lock.held.store(false,Ordering::Release);}}

/// 🔗️ Fixed typed roots and inline return flags retain exact immutable read authority.
pub struct ReadOwnershipRegistry<T:RetireOwned+Sync> {state:StateLock<T>,returned:AtomicUsize,sequence:AtomicU64,generation:AtomicU64,revision:[AtomicU64;4]}
impl<T:RetireOwned+Sync> ReadOwnershipRegistry<T> {
    pub fn constructor_capacity_bytes()->usize {crate::retirement::shared::arc_bytes::<Self>()+size_of::<[Option<Slot<T>>;READ_LEASE_CAPACITY]>()}
    pub fn admit(grant:RetainedCloneGrant)->Result<(Arc<Self>,RetainedCloneProgress),ValueError> {
        if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"read roots require controlled typed retirement"));}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"read registry constructor requires an admitted item"));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"read registry constructor requires admitted depth"));}
        let bytes=Self::constructor_capacity_bytes();if bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry constructor exceeds admitted capacity"));}
        let free=std::array::from_fn(|index|index as u16);
        let state=State {slots:Some(Box::new([const{None};READ_LEASE_CAPACITY])),occupied:[0;READ_LEASE_CAPACITY/64],free,free_read:0,free_len:READ_LEASE_CAPACITY,generation:0,cursor:0};
        let owner=Arc::new(Self {state:StateLock {held:AtomicBool::new(false),poisoned:AtomicBool::new(false),state:UnsafeCell::new(state)},returned:AtomicUsize::new(0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64::new(0))});
        Ok((owner,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
    pub fn try_issue(self:&Arc<Self>,owner:Arc<T>,grant:RetainedCloneGrant)->Result<(ReadLease<T>,RetainedCloneProgress),(ValueError,Arc<T>)> {
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"read issue requires an admitted item"),owner));}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"read issue requires admitted depth"),owner));}
        let mut state=match self.state.try_lock(){Ok(state)=>state,Err(error)=>return Err((error,owner))};
        if state.free_len==0||state.slots.is_none(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry has no admitted slot"),owner));}
        let Some(generation)=state.generation.checked_add(1)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"read generation overflow"),owner));};
        let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state.generation=generation;
        let alias=Arc::clone(&owner);state.slots.as_mut().unwrap()[index]=Some(Slot {generation,owner,returned:false});state.occupied[index/64]|=1u64<<(index%64);
        Ok((ReadLease {owner:ManuallyDrop::new(Some(alias)),registry:ManuallyDrop::new(Some(Arc::clone(self))),closing:ManuallyDrop::new(None),id:ReadLeaseId {index:index as u16,generation}},RetainedCloneProgress {copied_items:1,..Default::default()}))
    }
    pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {Ok(self.state.try_lock()?.contains(id))}
    pub fn occupied_count(&self)->Result<usize,ValueError> {Ok(READ_LEASE_CAPACITY-self.state.try_lock()?.free_len)}
    pub fn has_returned(&self)->bool {self.returned.load(Ordering::Acquire)!=0}
    pub fn take_returned(&self,grant:RetainedCloneGrant)->Result<(Option<Arc<T>>,RetainedCloneProgress),ValueError> {
        if grant.maximum_items==0{return Ok((None,Default::default()));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"read maintenance requires admitted depth"));}
        let mut state=self.state.try_lock()?;let Some(index)=state.next_index()else{return Ok((None,Default::default()));};state.cursor=(index+1)%READ_LEASE_CAPACITY;
        let owner=if state.slots.as_ref().unwrap()[index].as_ref().unwrap().returned {let slot=state.take(index);self.returned.fetch_sub(1,Ordering::AcqRel);Some(slot.owner)}else{None};
        Ok((owner,RetainedCloneProgress {copied_items:1,..Default::default()}))
    }
    pub fn publish_authority(&self,generation:u64,revision:[u8;32])->bool {
        let sequence=self.sequence.load(Ordering::Acquire);let Some(terminal)=sequence.checked_add(2)else{return false;};if sequence&1!=0||self.sequence.compare_exchange(sequence,sequence+1,Ordering::AcqRel,Ordering::Acquire).is_err(){return false;}
        self.generation.store(generation,Ordering::Relaxed);for(index,word)in self.revision.iter().enumerate(){word.store(u64::from_le_bytes(revision[index*8..index*8+8].try_into().unwrap()),Ordering::Relaxed);}self.sequence.store(terminal,Ordering::Release);true
    }
    pub fn authority_matches(&self,generation:u64,revision:[u8;32])->bool {
        let sequence=self.sequence.load(Ordering::Acquire);if sequence==0||sequence&1!=0||self.generation.load(Ordering::Relaxed)!=generation{return false;}
        for(index,word)in self.revision.iter().enumerate(){if word.load(Ordering::Relaxed)!=u64::from_le_bytes(revision[index*8..index*8+8].try_into().unwrap()){return false;}}sequence==self.sequence.load(Ordering::Acquire)
    }
    #[cfg(test)]
    fn set_cleanup_cursor(&self,cursor:usize)->Result<(),ValueError> {self.state.try_lock()?.cursor=cursor%READ_LEASE_CAPACITY;Ok(())}
}
impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T> {fn drop(&mut self){let state=self.state.state.get_mut();assert!(std::thread::panicking()||state.free_len==READ_LEASE_CAPACITY&&state.slots.is_none(),"read registry must close every original typed root and fixed backing");}}

struct RegistryRetirement<T:RetireOwned+Sync> {owner:ManuallyDrop<ReadOwnershipRegistry<T>>,active:Option<SharedControlledRetirement<T>>}
impl<T:RetireOwned+Sync> RetirementCursor for RegistryRetirement<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
        if let Some(active)=self.active.as_mut(){
            if active.terminal_is_empty(){self.active=None;return RetirementStep::Advanced;}
            return match active.step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)};
        }
        let state=self.owner.state.state.get_mut();
        if let Some(index)=state.next_index(){let slot=state.take(index);if slot.returned{self.owner.returned.fetch_sub(1,Ordering::AcqRel);}self.active=Some(SharedControlledRetirement::lease(slot.owner));return RetirementStep::Advanced;}
        let bytes=size_of::<[Option<Slot<T>>;READ_LEASE_CAPACITY]>();if grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted;}drop(state.slots.take());RetirementStep::Bytes(bytes)
    }
    fn terminal_is_empty(&self)->bool {self.active.is_none()&&unsafe{&*self.owner.state.state.get()}.slots.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.active.as_ref().map_or(Ok(0),SharedControlledRetirement::next_copy_byte_demand)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.active.as_ref().map_or(Some(0),|active|active.next_capacity_byte_demand(copy).ok())}
    fn next_close_byte_demand(&self)->Option<usize> {self.active.as_ref().map_or_else(||Some(if unsafe{&*self.owner.state.state.get()}.free_len==READ_LEASE_CAPACITY&&!self.terminal_is_empty(){size_of::<[Option<Slot<T>>;READ_LEASE_CAPACITY]>()}else{0}),|active|active.next_release_byte_demand().ok())}
    fn next_depth_demand(&self)->Result<usize,ValueError> {self.active.as_ref().map_or(Ok(1),SharedControlledRetirement::next_depth_demand)}
    fn allows_admitted_narrow_work(&self)->bool {true}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned+Sync> Drop for RegistryRetirement<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed registry cursor abandoned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}
impl<T:RetireOwned+Sync> RetireOwned for ReadOwnershipRegistry<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(RegistryRetirement {owner:ManuallyDrop::new(self),active:None})}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<RegistryRetirement<T>>())}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}

impl<T:RetireOwned+Sync> crate::FactoryPayloadRetirement for ReadOwnershipRegistry<T> {
    type CloseState=Option<crate::retirement::controlled::ControlledRetirement<Self>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn close_state_constructor_copy_bytes(&self)->usize {0}
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState) {*state=Some(crate::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("admitted typed registry lost its retirement support: {error}")));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError> {state.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))}
    fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let Some(owner)=state.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if owner.terminal_is_empty(){if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}state.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        Ok(RetainedCloneStep::Progress(owner.step(grant)?.progress()))
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}

struct RegistryAuthority<T:RetireOwned+Sync>(Arc<ReadOwnershipRegistry<T>>);
impl<T:RetireOwned+Sync> RetireOwned for RegistryAuthority<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(SharedControlledRetirement::lease(self.0))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<SharedControlledRetirement<ReadOwnershipRegistry<T>>>())}
    fn controlled_retirement_supported()->bool {true}
}

/// 👁️ An immutable typed root closes its exact read alias and registry authority through full grants.
pub struct ReadLease<T:RetireOwned+Sync> {owner:ManuallyDrop<Option<Arc<T>>>,registry:ManuallyDrop<Option<Arc<ReadOwnershipRegistry<T>>>>,closing:ManuallyDrop<Option<SharedControlledRetirement<ReadOwnershipRegistry<T>>>>,id:ReadLeaseId}
impl<T:RetireOwned+Sync> ReadLease<T> {
    pub fn get(&self)->&T {self.owner.as_deref().expect("read root is unavailable after granted return")}
    pub fn id(&self)->ReadLeaseId {self.id}
    pub fn commit_authority_matches(&self,generation:u64,revision:[u8;32])->bool {self.registry.as_ref().is_some_and(|registry|registry.authority_matches(generation,revision))}
    pub fn source_capacity_bytes(&self)->usize{RetainedCloneSource::<T>::borrowed_constructor_capacity_bytes::<OriginalReadSource<T>>()}
    pub fn source_copy_bytes(&self)->usize{RetainedCloneSource::<T>::borrowed_constructor_copy_bytes::<OriginalReadSource<T>>()}
    pub fn admit_source(self,grant:RetainedCloneGrant)->Result<(RetainedCloneSource<T,OriginalReadSource<T>>,RetainedCloneProgress),(ValueError,Self)>{
        if self.owner.is_none(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"returned read cannot issue an original source"),self));}
        RetainedCloneSource::admit_borrowed(OriginalReadSource::new(self),|read:&OriginalReadSource<T>|read.get(),grant).map_err(|(error,mut read)|{let original=read.take_refused();(error,original)})
    }
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {Ok(RetirementDemand {copy_bytes:self.next_copy_byte_demand()?,capacity_bytes:self.next_capacity_byte_demand(copy)?,release_bytes:self.next_release_byte_demand()?,depth:self.next_depth_demand()?})}
}
impl<T:RetireOwned+Sync> ErasedSnapshotRetirement for ReadLease<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"read closure exceeds admitted depth"));}
        if self.owner.is_some(){let registry=self.registry.as_ref().unwrap();let mut state=registry.state.try_lock()?;if !state.contains(self.id){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"read generation lost original registered root"));}let slot=state.slots.as_mut().unwrap()[self.id.index as usize].as_mut().unwrap();assert!(Arc::ptr_eq(&slot.owner,self.owner.as_ref().unwrap()));drop(self.owner.take());slot.returned=true;registry.returned.fetch_add(1,Ordering::AcqRel);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        if self.closing.is_none(){*self.closing=Some(SharedControlledRetirement::lease(self.registry.take().unwrap()));}
        let close=self.closing.as_mut().unwrap();let step=close.step(grant)?;if close.terminal_is_empty(){*self.closing=None;}Ok(step)
    }
    fn terminal_is_empty(&self)->bool {self.owner.is_none()&&self.registry.is_none()&&self.closing.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.closing.as_ref().map_or(Ok(0),SharedControlledRetirement::next_copy_byte_demand)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.closing.as_ref().map_or(Ok(0),|close|close.next_capacity_byte_demand(copy))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.owner.is_some(){return Ok(0);}self.closing.as_ref().map_or_else(||Ok(if self.registry.is_some(){crate::retirement::shared::arc_bytes::<ReadOwnershipRegistry<T>>()}else{0}),SharedControlledRetirement::next_release_byte_demand)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {self.closing.as_ref().map_or(Ok(usize::from(!self.terminal_is_empty())),SharedControlledRetirement::next_depth_demand)}
}
impl<T:RetireOwned+Sync> Drop for ReadLease<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"read lease must finish explicitly granted closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.closing);}}}}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
