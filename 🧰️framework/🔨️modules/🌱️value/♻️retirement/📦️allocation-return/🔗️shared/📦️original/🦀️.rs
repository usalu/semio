//! 📦️ Shared initialized originals elect one destructive owner without moving the complete payload.
use super::shared_allocation_bytes as arc_bytes;
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::{cell::UnsafeCell,mem::{ManuallyDrop,MaybeUninit},sync::{Arc,atomic::{AtomicUsize,Ordering}}};

struct Allocation<T>{aliases:AtomicUsize,value:UnsafeCell<MaybeUninit<T>>}
/// 🪢️ Live aliases expose immutable payloads; only the elected zero-alias owner mutates initialized fields.
unsafe impl<T:Send+Sync> Sync for Allocation<T>{}

pub struct SharedOriginalStorage<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>}
/// 🎫️ A real fallibly allocated backing retains its empty payload until publication or paid release.
pub struct SharedOriginalBirth<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>}
/// 📤️ One currently funded unique backing permits exactly one initialized payload publication.
pub struct SharedOriginalPublication<'a,T:Send+Sync+'static>{birth:&'a mut SharedOriginalBirth<T>}
impl<T:Send+Sync+'static> SharedOriginalPublication<'_,T>{
 pub fn commit(self,value:T)->crate::RetainedWorkOutcome<(SharedOriginalStorage<T>,RetainedCloneProgress)>{
  let mut source=self.birth.source.take().expect("funded original publication retains its backing");let storage=Arc::get_mut(&mut source).expect("unpublished original backing is unique");storage.value.get_mut().write(value);storage.aliases.store(1,Ordering::Relaxed);
  crate::RetainedWorkOutcome{result:Ok((SharedOriginalStorage{source:ManuallyDrop::new(Some(source))},RetainedCloneProgress{copied_items:1,copied_bytes:SharedOriginalBirth::<T>::publication_demand().copy_bytes,..Default::default()})),work:u128::from(Self::work_demand())}
 }
 pub const fn work_demand()->u64{4}
}
impl<T:Send+Sync+'static> SharedOriginalBirth<T>{
 pub fn publication_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Arc<Allocation<T>>>>()+std::mem::size_of::<T>()+std::mem::size_of::<AtomicUsize>()+std::mem::size_of::<SharedOriginalStorage<T>>(),depth:1,..Default::default()}}
 pub fn publication(&mut self,grant:RetainedCloneGrant)->Result<Option<SharedOriginalPublication<'_,T>>,ValueError>{if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original birth storage already transferred"))}if !crate::retained_clone::retained_clone_grant_admits(grant,Self::publication_demand()){return Ok(None)}Ok(Some(SharedOriginalPublication{birth:self}))}
 pub fn next_close_demand(&self)->crate::RetirementDemand{if self.source.is_none(){Default::default()}else{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Arc<Allocation<T>>>>(),release_bytes:SharedOriginalStorage::<T>::allocation_bytes(),depth:1,..Default::default()}}}
 pub fn close_work_demand(&self)->u64{u64::from(self.source.is_some())}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{if self.source.is_none(){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}let d=self.next_close_demand();if !crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}drop(self.source.take());crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
}
impl<T:Send+Sync+'static> Drop for SharedOriginalBirth<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"empty original birth requires granted publication or release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

pub enum SharedOriginalRelease<T:Send+Sync+'static>{Alias,Original(SharedOriginal<T>)}
pub struct SharedOriginal<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>,initialized:bool}

impl<T:Send+Sync+'static> SharedOriginalStorage<T>{
 /// 📥️ Quotes allocation and typed placement before the original slot can be taken.
 pub fn source_admission_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Allocation<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<T>>())).and_then(|n|n.checked_add(std::mem::size_of::<Option<Self>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared source copy extent overflow"))?;Ok(d)}
 pub const fn source_admission_work_demand()->u64{4}
 pub fn admit_from(source:&mut Option<T>,target:&mut Option<Self>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome{
  if target.is_some()||actual.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original shared recipient or marker is occupied")),work:0}}
  if source.is_none(){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}
  let d=match Self::source_admission_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}
  let turn=super::try_uninitialized_arc::<Allocation<T>>(grant,actual);let mut backing=match turn.result{Ok((backing,_))=>backing,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:turn.work}};
  let value=source.take().expect("original shared source validated before allocation");Arc::get_mut(&mut backing).expect("original shared backing is unique").write(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))});*target=Some(Self{source:ManuallyDrop::new(Some(unsafe{backing.assume_init()}))});
  crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+3}
 }

 /// 📐️ Quotes the actual private allocation payload and its returned typed storage wrapper.
 pub fn value_admission_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Allocation<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<Self>())).ok_or_else(||ValueError::invariant("original shared value copy extent overflow"))?;Ok(d)}
 pub const fn value_admission_work_demand()->u64{3}
 pub fn admit(value:T,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,T)>{
  let d=match Self::value_admission_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err((error,value)),work:0}};if actual.is_some()||!crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared value birth is not fully funded or marker occupied"),value)),work:0}}let turn=super::try_uninitialized_arc::<Allocation<T>>(grant,actual);let mut backing=match turn.result{Ok((backing,_))=>backing,Err(error)=>return crate::RetainedWorkOutcome{result:Err((error,value)),work:turn.work}};Arc::get_mut(&mut backing).expect("original shared value backing is unique").write(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))});let source=unsafe{backing.assume_init()};crate::RetainedWorkOutcome{result:Ok((Self{source:ManuallyDrop::new(Some(source))},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+2}
 }
 pub fn prepare_birth_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<AtomicUsize>()).and_then(|n|n.checked_add(std::mem::size_of::<SharedOriginalBirth<T>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original empty backing copy extent overflow"))?;Ok(d)}
 pub const fn prepare_birth_work_demand()->u64{3}
 pub fn prepare_birth(grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<(SharedOriginalBirth<T>,RetainedCloneProgress)>{
  let d=match Self::prepare_birth_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if actual.is_some()||!crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original empty backing is not funded or marker occupied")),work:0}}
  let turn=super::try_uninitialized_arc::<Allocation<T>>(grant,actual);let mut source=match turn.result{Ok((source,_))=>source,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:turn.work}};
  let pointer=Arc::get_mut(&mut source).expect("fresh original backing has one owner").as_mut_ptr();unsafe{std::ptr::addr_of_mut!((*pointer).aliases).write(AtomicUsize::new(0));}
  let source=unsafe{source.assume_init()};crate::RetainedWorkOutcome{result:Ok((SharedOriginalBirth{source:ManuallyDrop::new(Some(source))},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+2}
 }

 pub fn allocation_bytes()->usize{arc_bytes::<Allocation<T>>()}
 pub fn get(&self)->&T{let source=self.source.as_ref().expect("live original alias");assert!(source.aliases.load(Ordering::Acquire)>0);unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original alias").value.get().cast::<T>()as usize}
 pub fn aliases(&self)->usize{self.source.as_ref().expect("live original alias").aliases.load(Ordering::Acquire)}
 /// 🔍️ A unique original requires both logical and physical strong counts to be one.
 pub fn physically_unique(&self)->bool{self.source.as_ref().is_some_and(|source|source.aliases.load(Ordering::Acquire)==1&&Arc::strong_count(source)==1)}
 pub fn duplicate_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*std::mem::size_of::<AtomicUsize>()+std::mem::size_of::<Self>(),depth:1,..Default::default()}}
 pub const fn duplicate_work_demand()->u64{3}
 pub fn duplicate(&self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Self>{
  let source=match self.source.as_ref(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original alias source is empty")),work:0}};let d=Self::duplicate_demand();if !crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias duplicate metadata is not funded")),work:0}}
  if source.aliases.try_update(Ordering::AcqRel,Ordering::Acquire,|count|(count>0).then(||count.checked_add(1)).flatten()).is_err(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias space exhausted")),work:1}}
  let value=Self{source:ManuallyDrop::new(Some(Arc::clone(source)))};crate::RetainedWorkOutcome{result:Ok(value),work:3}
 }

 /// 🔗️ Adds one counted alias without allocating: the shared strong count is the only state touched.
 pub fn alias(&self)->Self{let source=self.source.as_ref().expect("live original alias");let strong=Arc::clone(source);source.aliases.fetch_add(1,Ordering::AcqRel);Self{source:ManuallyDrop::new(Some(strong))}}
 /// 📏️ Bounds both actual alias-election outcomes before any logical or physical counter changes.
 pub fn retire_demand(&self)->Result<crate::RetirementDemand,ValueError>{let Some(source)=self.source.as_ref()else{return Ok(Default::default())};if source.aliases.load(Ordering::Acquire)==0{return Err(ValueError::invariant("original alias election has no live logical owner"))}let copy=std::mem::size_of::<Option<Arc<Allocation<T>>>>().checked_add(2*std::mem::size_of::<AtomicUsize>()).and_then(|n|n.checked_add(std::mem::size_of::<SharedOriginalRelease<T>>())).ok_or_else(||ValueError::invariant("original election copy extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})}
 pub fn retire_work_demand(&self)->Result<u64,ValueError>{self.retire_demand()?;Ok(if self.source.is_some(){4}else{0})}
 /// 🗳️ Retains this exact alias until the complete election and possible physical alias return are funded.
 pub fn retire_step(&mut self,grant:RetainedCloneGrant,maximum:u64)->crate::RetainedWorkOutcome<Option<(SharedOriginalRelease<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.retire_demand()?;if self.source.is_none(){return Ok(None)}if maximum<4||!crate::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let source=self.source.take().expect("funded original election retains its source");let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0,"funded original election owns a logical alias");let(output,copy)=if previous>1{drop(source);work=4;(SharedOriginalRelease::Alias,d.copy_bytes)}else{work=3;(SharedOriginalRelease::Original(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true}),d.copy_bytes-std::mem::size_of::<AtomicUsize>())};Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
 pub fn retire(mut self)->SharedOriginalRelease<T>{let source=self.source.take().expect("live original alias");let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0);if previous>1{drop(source);SharedOriginalRelease::Alias}else{SharedOriginalRelease::Original(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true})}}
 pub fn try_owner(mut self)->Result<SharedOriginal<T>,Self>{let source=self.source.as_ref().expect("live original alias");if Arc::strong_count(source)!=1||source.aliases.compare_exchange(1,0,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self);}Ok(SharedOriginal{source:ManuallyDrop::new(self.source.take()),initialized:true})}
 /// 📮️ Transfers exactly one counted alias into a sealed return slot without releasing it.
 pub fn into_raw(mut self)->*const(){Arc::into_raw(self.source.take().expect("live original alias")).cast()}
 /// 📨️ Recovers the exact initialized counted alias previously transferred by into_raw.
 /// # Safety
 /// The pointer must be live, issued for T by into_raw, and consumed once.
 pub unsafe fn from_raw(pointer:*const())->Self{Self{source:ManuallyDrop::new(Some(unsafe{Arc::from_raw(pointer.cast::<Allocation<T>>())}))}}
}
impl<T:Send+Sync+'static> Drop for SharedOriginalStorage<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"shared original alias must retire or transfer");}}

impl<T:Send+Sync+'static> SharedOriginal<T>{
 /// 🕳️ Releases the allocation of a last owner that was dropped without retirement; the initialized payload is leaked, never dropped uncontrolled.
 pub fn abandon(mut self){self.initialized=false;drop(self.source.take());}
 pub fn allocation_bytes()->usize{SharedOriginalStorage::<T>::allocation_bytes()}
 pub fn get(&self)->&T{assert!(self.initialized);let source=self.source.as_ref().expect("live original owner");unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original owner").value.get().cast::<T>()as usize}
 pub fn physical_is_unique(&self)->bool{self.source.as_ref().is_some_and(|source|Arc::strong_count(source)==1)}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 📏️ Quotes payload movement, both control fields and the actual final strong/weak counter return.
 pub fn take_value_demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.source.is_none(){if self.initialized{return Err(ValueError::invariant("empty original owner remains initialized"))}return Ok(Default::default())}if !self.initialized{return Err(ValueError::invariant("partially transferred original cannot return its whole payload"))}let copy=std::mem::size_of::<T>().checked_add(std::mem::size_of::<Option<Arc<Allocation<T>>>>()).and_then(|n|n.checked_add(std::mem::size_of::<bool>())).and_then(|n|n.checked_add(2*std::mem::size_of::<AtomicUsize>())).ok_or_else(||ValueError::invariant("original payload handback copy extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes:copy,release_bytes:Self::allocation_bytes(),depth:1,..Default::default()})}
 pub fn take_value_work_demand(&self)->Result<u64,ValueError>{self.take_value_demand()?;Ok(if self.source.is_some(){4}else{0})}
 /// 📤️ Returns the unique initialized payload only after its exact movement and backing release are funded.
 pub fn take_value_step(&mut self,grant:RetainedCloneGrant,maximum:u64)->crate::RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_value_demand()?;if self.source.is_none()||maximum<4||!crate::retained_clone::retained_clone_grant_admits(grant,d)||!self.physical_is_unique(){return Ok(None)}let source=self.source.take().expect("funded unique original retains its allocation");self.initialized=false;let value=unsafe{source.value.get().cast::<T>().read()};drop(source);work=4;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
 pub fn take_value(&mut self)->Option<T>{if !self.initialized||!self.physical_is_unique(){return None;}let source=self.source.take().unwrap();self.initialized=false;let value=unsafe{source.value.get().cast::<T>().read()};drop(source);Some(value)}
 /// 🔎️ Borrows one still-initialized aligned field of this exact allocation.
 /// # Safety
 /// The projection must remain inside T and must never name an already transferred field.
 pub unsafe fn field<F>(&self,project:fn(*const T)->*const F)->&F{let source=self.source.as_ref().expect("live original owner");unsafe{&*project(source.value.get().cast::<T>())}}
 /// 📤️ Transfers one initialized nonoverlapping aligned field once.
 /// # Safety
 /// The projection must name that field inside T; subsequent operations must never read or drop the transferred field.
 pub unsafe fn take_field<F>(&mut self,project:fn(*mut T)->*mut F)->F{let source=self.source.as_ref().expect("live original owner");self.initialized=false;unsafe{project(source.value.get().cast::<T>()).read()}}
 /// 🧹️ Releases storage after every owned field has transferred; physical aliases defer release without effects.
 /// # Safety
 /// The caller must prove no initialized payload remains.
 pub unsafe fn release_empty(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{if self.source.is_none()||!self.physical_is_unique()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<Self::allocation_bytes(){return Default::default();}self.initialized=false;drop(self.source.take());RetainedCloneProgress{copied_items:1,released_bytes:Self::allocation_bytes(),..Default::default()}}
}
impl<T:Send+Sync+'static> Drop for SharedOriginal<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"shared original owner must drain its initialized payload");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
