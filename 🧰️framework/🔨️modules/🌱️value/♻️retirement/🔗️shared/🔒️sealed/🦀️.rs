//! 🔒️ Shared immutable values expose strong handles and retain their original issuer.
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{mem::ManuallyDrop,sync::{Arc,atomic::{AtomicPtr,AtomicU8,AtomicU64,Ordering}}};

static NEXT_ISSUANCE:AtomicU64=AtomicU64::new(1);

/// 🎟️ The original producer supplies admission and demand functions without an erased allocation.
pub struct SharedIssuer<T>{pub demand:fn(&T)->Result<RetirementDemand,ValueError>,pub admit:fn(&mut Option<T>,RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>}
impl<T> Copy for SharedIssuer<T>{}
impl<T> Clone for SharedIssuer<T>{fn clone(&self)->Self{*self}}
impl<T:RetireOwned> SharedIssuer<T>{
 pub fn owned()->Self{Self{demand:|_|{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no owned issuer"));}Ok(RetirementDemand{capacity_bytes:crate::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()})},admit:|slot,grant|{let Some(value)=slot.take()else{return Ok(None)};match crate::retirement::admit_owned_retirement(value,grant){Ok(admitted)=>Ok(Some(admitted)),Err((error,value))=>{*slot=Some(value);Err(error)}}}}}
}

/// 🔐️ The private Arc has no raw, Weak, mutable or consuming escape hatch.
pub struct SealedShared<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<T>>>,issuer:SharedIssuer<T>,issuance:u64}
/// 🎟️ Reserves a genuine original issuance before any dependent candidate allocation.
pub struct SealedSharedBirth<T:Send+Sync+'static>{issuance:u64,marker:std::marker::PhantomData<T>}
impl<T:Send+Sync+'static> SealedSharedBirth<T>{pub fn materialize(self,value:T,issuer:SharedIssuer<T>)->(SealedShared<T>,RetainedCloneProgress){(SealedShared{source:ManuallyDrop::new(Some(Arc::new(value))),issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:SealedShared::<T>::birth_bytes(),..Default::default()})}}
/// 📥️ One admitted original alias has one fixed issuer return slot without heap backing.
pub struct SharedAliasReturn<T:Send+Sync+'static>{source:AtomicPtr<T>,state:AtomicU8,identity:usize,issuer:SharedIssuer<T>,issuance:u64}
impl<T:Send+Sync+'static> SharedAliasReturn<T>{
 pub fn drain(&self,grant:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{
  if self.state.load(Ordering::Acquire)!=2{return Ok(None);}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SealedShared<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original returned alias transfer is not funded"));}
  let pointer=self.source.swap(std::ptr::null_mut(),Ordering::AcqRel);if pointer.is_null(){return Ok(None);}
  let handle=SealedShared{source:ManuallyDrop::new(Some(unsafe{Arc::from_raw(pointer)})),issuer:self.issuer,issuance:self.issuance};
  self.state.store(3,Ordering::Release);
  Ok(Some((handle,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SealedShared<T>>(),..Default::default()})))
 }
 pub fn retire_unused(&self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"unused original slot retirement is not funded"));}self.state.compare_exchange(0,3,Ordering::AcqRel,Ordering::Acquire).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original slot has already entered handback"))?;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
 pub fn terminal_is_empty(&self)->bool{self.state.load(Ordering::Acquire)==3}
}
impl<T:Send+Sync+'static> Drop for SharedAliasReturn<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"issuer slot retains an original returned alias");}}
impl<T:Send+Sync+'static> SealedShared<T>{
 pub fn birth_bytes()->usize{super::arc_bytes::<T>()}
 pub fn prepare_birth(grant:RetainedCloneGrant)->Result<SealedSharedBirth<T>,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<Self::birth_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed issuance requires genuine header birth authority"));}
  let issuance=NEXT_ISSUANCE.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted"))?;
  Ok(SealedSharedBirth{issuance,marker:std::marker::PhantomData})
 }
 pub fn admit(value:T,issuer:SharedIssuer<T>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,T)>{
  let refusal=if grant.maximum_items==0{Some(ValueRefusalKind::WorkLimit)}else if grant.maximum_depth==0{Some(ValueRefusalKind::DepthLimit)}else if grant.maximum_capacity_bytes<Self::birth_bytes(){Some(ValueRefusalKind::OwnershipLimit)}else{None};
  if let Some(kind)=refusal{return Err((ValueError::literal(kind,"sealed shared source birth is not funded"),value));}
  let issuance=match NEXT_ISSUANCE.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)){Ok(issuance)=>issuance,Err(_)=>return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted"),value))};
  Ok((Self{source:ManuallyDrop::new(Some(Arc::new(value))),issuer,issuance},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:Self::birth_bytes(),..Default::default()}))
 }
 pub fn try_duplicate(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{
  let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred"))?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Self>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed alias handoff is not funded"));}
  Ok((Self{source:ManuallyDrop::new(Some(Arc::clone(source))),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),..Default::default()}))
 }
 pub fn return_slot(&self,grant:RetainedCloneGrant)->Result<(SharedAliasReturn<T>,RetainedCloneProgress),ValueError>{
  if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transferred alias cannot issue a return slot"));}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SharedAliasReturn<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixed original return slot is not funded"));}
  Ok((SharedAliasReturn{source:AtomicPtr::new(std::ptr::null_mut()),state:AtomicU8::new(0),identity:self.identity(),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SharedAliasReturn<T>>(),..Default::default()}))
 }
 pub fn return_alias(mut self,slot:&SharedAliasReturn<T>)->Result<(),Self>{
  if self.identity()!=slot.identity||self.issuance!=slot.issuance||slot.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self);}
  let pointer=Arc::into_raw(self.source.take().unwrap()).cast_mut();
  slot.source.store(pointer,Ordering::Release);slot.state.store(2,Ordering::Release);
  Ok(())
 }
 /// 📤️ Transfers the same original value after one funded atomic strong-only unwrap.
 pub fn try_unwrap(mut self,grant:RetainedCloneGrant)->Result<(T,RetainedCloneProgress),(ValueError,Self)>{
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<T>()||grant.maximum_release_bytes<Self::birth_bytes()||grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original sealed value transfer is not funded"),self));}
  let Some(source)=self.source.take()else{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"),self));};
  match Arc::try_unwrap(source){Ok(value)=>Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),released_bytes:Self::birth_bytes(),..Default::default()})),Err(source)=>{*self.source=Some(source);Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original sealed value still has live projections"),self))}}
 }
 pub fn identity(&self)->usize{self.source.as_ref().map_or(0,|source|Arc::as_ptr(source)as usize)}
 pub fn get(&self)->&T{self.source.as_deref().expect("sealed shared source has transferred")}
}
impl<T:Send+Sync+'static> std::ops::Deref for SealedShared<T>{type Target=T;fn deref(&self)->&T{self.get()}}
impl<T:Send+Sync+'static> Drop for SealedShared<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"sealed source requires original granted handback");if self.source.is_none(){unsafe{ManuallyDrop::drop(&mut self.source);}}}}

struct Cursor<T:Send+Sync+'static>{handle:SealedShared<T>,pending:ManuallyDrop<Option<T>>,active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<T:Send+Sync+'static> Cursor<T>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.handle.source.is_some(){return Ok(RetirementDemand{release_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()});}
  if let Some(value)=self.pending.as_ref(){return (self.handle.issuer.demand)(value);}
  self.active.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
  if let Some(source)=self.handle.source.take(){
   let value=Arc::into_inner(source);let last=value.is_some();*self.pending=value;
   return Ok(RetainedCloneProgress{copied_items:1,released_bytes:if last{demand.release_bytes}else{0},..Default::default()});
  }
  if self.pending.is_some(){return match (self.handle.issuer.admit)(&mut self.pending,grant)?{Some((owner,progress))=>{*self.active=Some(owner);if !progress.fits(grant)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sealed original issuer changed its admitted receipt"));}Ok(progress)},None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sealed original issuer omitted its payload"))};}
  crate::close_factory_ticket(&mut self.active,grant).map(|step|step.progress())
 }
}
impl<T:Send+Sync+'static> RetireOwned for SealedShared<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{handle:self,pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None)})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+Sync+'static> RetirementCursor for Cursor<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.step(grant){Ok(progress)=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.handle.source.is_none()&&self.pending.is_none()&&self.active.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"sealed cursor retains its original payload and issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.active);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
