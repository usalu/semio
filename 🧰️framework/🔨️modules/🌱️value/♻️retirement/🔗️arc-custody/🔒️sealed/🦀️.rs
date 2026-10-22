//! 🔒️ Shared immutable values expose strong handles and retain their original issuer.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,RetireOwned,retirement,factory_ticket_demands,close_factory_ticket};
use retirement::{RetirementCursor,RetirementStep};
use std::{mem::ManuallyDrop,sync::atomic::{AtomicPtr,AtomicU8,AtomicU64,Ordering}};
use retirement::arc_custody::original::{SharedOriginal,SharedOriginalStorage,SharedOriginalRelease};

static NEXT_ISSUANCE:AtomicU64=AtomicU64::new(1);

#[cfg(test)]
#[path="🚚️frontier/🧪️tests/🦀️.rs"]
mod frontier_tests;
/// 🎟️ The original producer supplies admission and demand functions without an erased allocation.
enum OriginalDemand{Native,Shared}
enum OriginalSlot<'a,T:Send+Sync+'static>{Native(&'a mut Option<T>),Shared(&'a mut Option<SharedOriginal<T>>)}
pub struct SharedIssuer<T:Send+Sync+'static>{pub transfer_copy_bytes:fn(&T)->usize,demand:fn(&T,OriginalDemand)->Result<Option<RetirementDemand>,ValueError>,admit:fn(OriginalSlot<'_,T>,RetainedCloneGrant)->Result<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>,ValueError>}
impl<T:Send+Sync+'static> Copy for SharedIssuer<T>{}
impl<T:Send+Sync+'static> Clone for SharedIssuer<T>{fn clone(&self)->Self{*self}}
impl<T:RetireOwned+Sync> SharedIssuer<T>{
 pub fn owned()->Self{Self{transfer_copy_bytes:|_|size_of::<T>(),demand:|value,mode|match mode{OriginalDemand::Native=>native_payload_demand(value).map(Some),OriginalDemand::Shared=>{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no controlled shared issuer"));}Ok(value.shared_original_birth_bytes().map(|capacity_bytes|RetirementDemand{capacity_bytes,depth:1,..Default::default()}))}},admit:|slot,grant|match slot{OriginalSlot::Shared(slot)=>T::shared_original_retirement(slot,grant),OriginalSlot::Native(slot)=>{let Some(value)=slot.as_ref()else{return Ok(None)};let demand=native_payload_demand(value)?;if !retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native payload cursor birth is not funded"));}let value=slot.take().expect("validated original native payload");Ok(Some((value.retirement(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))}}}}
}

fn native_payload_demand<T:RetireOwned>(value:&T)->Result<RetirementDemand,ValueError>{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no native owned issuer"));}let capacity_bytes=value.retirement_birth_bytes().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no native cursor birth"))?;Ok(RetirementDemand{capacity_bytes,depth:1,..Default::default()})}


/// 🔐️ The private Arc has no raw, Weak, mutable or consuming escape hatch.
pub struct SealedShared<T:Send+Sync+'static>{source:ManuallyDrop<Option<SharedOriginalStorage<T>>>,issuer:SharedIssuer<T>,issuance:u64}
impl<T:std::fmt::Debug+Send+Sync+'static> std::fmt::Debug for SealedShared<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.get().fmt(f)}}
impl<T:PartialEq+Send+Sync+'static> PartialEq for SealedShared<T>{fn eq(&self,other:&Self)->bool{self.get()==other.get()}}
impl<T:Eq+Send+Sync+'static> Eq for SealedShared<T>{}

/// 🎟️ Reserves a genuine original issuance before any dependent candidate allocation.
pub struct SealedSharedBirth<T:Send+Sync+'static>{issuance:u64,grant:RetainedCloneGrant,marker:std::marker::PhantomData<T>}
impl<T:Send+Sync+'static> SealedSharedBirth<T>{pub fn materialize(self,value:T,issuer:SharedIssuer<T>)->(SealedShared<T>,RetainedCloneProgress){(SealedShared{source:ManuallyDrop::new(Some(SharedOriginalStorage::admit(value,self.grant).unwrap_or_else(|_|unreachable!("issued shared storage birth")).0)),issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:SealedShared::<T>::birth_bytes(),..Default::default()})}}
/// 📥️ One admitted original alias has one fixed issuer return slot without heap backing.
pub struct SharedAliasReturn<T:Send+Sync+'static>{source:AtomicPtr<()>,state:AtomicU8,identity:usize,issuer:SharedIssuer<T>,issuance:u64}
impl<T:Send+Sync+'static> SharedAliasReturn<T>{
 pub fn drain(&self,grant:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{
  if self.state.load(Ordering::Acquire)!=2{return Ok(None);}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SealedShared<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original returned alias transfer is not funded"));}
  let pointer=self.source.swap(std::ptr::null_mut(),Ordering::AcqRel);if pointer.is_null(){return Ok(None);}
  let handle=SealedShared{source:ManuallyDrop::new(Some(unsafe{SharedOriginalStorage::from_raw(pointer.cast_const())})),issuer:self.issuer,issuance:self.issuance};
  self.state.store(3,Ordering::Release);
  Ok(Some((handle,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SealedShared<T>>(),..Default::default()})))
 }
 pub fn retire_unused(&self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"unused original slot retirement is not funded"));}self.state.compare_exchange(0,3,Ordering::AcqRel,Ordering::Acquire).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original slot has already entered handback"))?;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
 pub fn terminal_is_empty(&self)->bool{self.state.load(Ordering::Acquire)==3}
}
impl<T:Send+Sync+'static> Drop for SharedAliasReturn<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"issuer slot retains an original returned alias");}}
impl<T:Send+Sync+'static> SealedShared<T>{
 pub fn birth_bytes()->usize{SharedOriginalStorage::<T>::allocation_bytes()}
 pub fn prepare_birth(grant:RetainedCloneGrant)->Result<SealedSharedBirth<T>,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<Self::birth_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed issuance requires genuine header birth authority"));}
  let issuance=NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted"))?;
  Ok(SealedSharedBirth{issuance,grant,marker:std::marker::PhantomData})
 }
 pub fn admit(value:T,issuer:SharedIssuer<T>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,T)>{
  let refusal=if grant.maximum_items==0{Some(ValueRefusalKind::WorkLimit)}else if grant.maximum_depth==0{Some(ValueRefusalKind::DepthLimit)}else if grant.maximum_capacity_bytes<Self::birth_bytes(){Some(ValueRefusalKind::OwnershipLimit)}else{None};
  if let Some(kind)=refusal{return Err((ValueError::literal(kind,"sealed shared source birth is not funded"),value));}
  let issuance=match NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)){Ok(issuance)=>issuance,Err(_)=>return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted"),value))};
  Ok((Self{source:ManuallyDrop::new(Some(SharedOriginalStorage::admit(value,grant).unwrap_or_else(|_|unreachable!("validated shared storage birth")).0)),issuer,issuance},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:Self::birth_bytes(),..Default::default()}))
 }
 pub fn try_duplicate(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{
  let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred"))?;
  if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed alias handoff is not funded"));}
  Ok((Self{source:ManuallyDrop::new(Some(source.duplicate()?)),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 /// 🪢️ Returns one paid alias while the matching original strong parent remains borrowed and live.
 pub fn return_borrowed_alias(&mut self,witness:&Self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let Some(original)=self.source.as_ref()else{return Ok(Default::default())};
  let Some(parent)=witness.source.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original sealed parent has already transferred"))};
  if self.issuance!=witness.issuance||original.identity()!=parent.identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed strong parent does not witness the original sealed alias"))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(Default::default())}
  let Some(source)=self.source.take()else{unreachable!("borrowed original presence checked")};match source.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(_)=>unreachable!("live borrowed parent retains the original")};Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 🕊️ Observes the genuine strong source after all issued aliases have returned.
 pub fn original_aliases_are_returned(&self)->bool{self.source.as_ref().is_none_or(|source|source.aliases()==1)}
 pub fn return_slot(&self,grant:RetainedCloneGrant)->Result<(SharedAliasReturn<T>,RetainedCloneProgress),ValueError>{
  if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transferred alias cannot issue a return slot"));}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SharedAliasReturn<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixed original return slot is not funded"));}
  Ok((SharedAliasReturn{source:AtomicPtr::new(std::ptr::null_mut()),state:AtomicU8::new(0),identity:self.identity(),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SharedAliasReturn<T>>(),..Default::default()}))
 }
 pub fn return_alias(mut self,slot:&SharedAliasReturn<T>)->Result<(),Self>{
  if self.identity()!=slot.identity||self.issuance!=slot.issuance||slot.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self);}
  let pointer=self.source.take().unwrap().into_raw().cast_mut();
  slot.source.store(pointer,Ordering::Release);slot.state.store(2,Ordering::Release);
  Ok(())
 }
 /// 📤️ Transfers the same original value after one funded atomic strong-only unwrap.
 pub fn original_transfer_demand(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"))?;Ok(RetirementDemand{copy_bytes:(self.issuer.transfer_copy_bytes)(source.get()),release_bytes:Self::birth_bytes(),depth:1,..Default::default()})}
 pub fn try_unwrap(mut self,grant:RetainedCloneGrant)->Result<(T,RetainedCloneProgress),(ValueError,Self)>{
  let demand=match self.original_transfer_demand(){Ok(demand)=>demand,Err(error)=>return Err((error,self))};
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original sealed value transfer is not funded"),self));}
  let Some(source)=self.source.take()else{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"),self));};
  match source.try_owner(){Ok(mut owner)=>{let value=owner.take_value().expect("unique initialized original handback");Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))},Err(source)=>{*self.source=Some(source);Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original sealed value still has live projections"),self))}}
 }
 pub fn identity(&self)->usize{self.source.as_ref().map_or(0,|source|source.identity())}
 pub fn get(&self)->&T{self.source.as_ref().expect("sealed shared source has transferred").get()}
}
impl<T:Send+Sync+'static> std::ops::Deref for SealedShared<T>{type Target=T;fn deref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> Drop for SealedShared<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"sealed source requires original granted handback");if self.source.is_none(){unsafe{ManuallyDrop::drop(&mut self.source);}}}}

struct Cursor<T:Send+Sync+'static>{handle:SealedShared<T>,pending:ManuallyDrop<Option<T>>,original:Option<SharedOriginal<T>>}
impl<T:Send+Sync+'static> Cursor<T>{
 fn original_demands(&self,original:&T)->Result<RetirementDemand,ValueError>{if let Some(demand)=(self.handle.issuer.demand)(original,OriginalDemand::Shared)?{return Ok(demand);}let copy_bytes=size_of::<Option<SharedOriginalStorage<T>>>().checked_add(size_of::<Option<T>>().checked_mul(2).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared payload transfer footprint overflow"))?).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared payload transfer footprint overflow"))?;Ok(RetirementDemand{copy_bytes,release_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()})}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(source)=self.handle.source.as_ref(){return if source.aliases()>1{Ok(RetirementDemand{depth:1,..Default::default()})}else{self.original_demands(source.get())};}
  if let Some(original)=self.original.as_ref(){return self.original_demands(original.get());}
  if let Some(value)=self.pending.as_ref(){return (self.handle.issuer.demand)(value,OriginalDemand::Native)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native payload has no issuer"));}
  let _=body;Ok(Default::default())
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetirementStep,ValueError>{let demand=self.demands(grant.maximum_copy_bytes)?;self.step_selected(grant,demand)}
 fn step_selected(&mut self,grant:RetainedCloneGrant,mut demand:RetirementDemand)->Result<RetirementStep,ValueError>{
  if !retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetirementStep::BudgetExhausted);}
  let mut elected=false;
  if let Some(source)=self.handle.source.take(){match source.retire(){SharedOriginalRelease::Alias=>return Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),SharedOriginalRelease::Original(original)=>{self.original=Some(original);elected=true;}}}
  if let Some(original)=self.original.as_ref(){demand=self.original_demands(original.get())?;if !retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Self::election_progress(elected));}}
  if let Some(original)=self.original.as_ref(){
   if (self.handle.issuer.demand)(original.get(),OriginalDemand::Shared)?.is_some(){return match(self.handle.issuer.admit)(OriginalSlot::Shared(&mut self.original),grant)?{Some((owner,progress))=>{if self.original.is_some()||!progress.fits(grant)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"shared original issuer changed its admitted receipt"));}Ok(RetirementStep::ChildProgress(owner,progress))},None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"shared original issuer omitted its initialized allocation"))};}
   let Some(value)=self.original.as_mut().unwrap().take_value()else{return Ok(Self::election_progress(elected))};self.original=None;*self.pending=Some(value);return Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));
  }
  if self.pending.is_some(){return match(self.handle.issuer.admit)(OriginalSlot::Native(&mut self.pending),grant)?{Some((owner,progress))=>{if self.pending.is_some()||!progress.fits(grant)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sealed original issuer changed its admitted receipt"));}Ok(RetirementStep::ChildProgress(owner,progress))},None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sealed original issuer omitted its payload"))};}
  Ok(RetirementStep::Complete)
 }
 fn election_progress(elected:bool)->RetirementStep{if elected{RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementStep::BudgetExhausted}}
}
impl<T:Send+Sync+'static> RetireOwned for SealedShared<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{handle:self,pending:ManuallyDrop::new(None),original:None})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+Sync+'static> RetirementCursor for Cursor<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}self.step(grant).unwrap_or_else(RetirementStep::Failure)}
 fn terminal_is_empty(&self)->bool{self.handle.source.is_none()&&self.original.is_none()&&self.pending.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{false}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"sealed cursor retains its original payload and issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pending);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
