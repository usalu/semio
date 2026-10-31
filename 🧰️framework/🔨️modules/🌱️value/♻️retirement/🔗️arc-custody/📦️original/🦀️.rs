//! 📦️ Shared initialized originals elect one destructive owner without moving the complete payload.
use super::arc_bytes;
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
 pub fn commit(self,value:T)->crate::RetainedWorkOutcome<(SharedOriginalStorage<T>,RetainedCloneProgress)>{let mut source=self.birth.source.take().expect("funded original backing remains retained");let storage=Arc::get_mut(&mut source).expect("unpublished original backing is unique");storage.value.get_mut().write(value);storage.aliases.store(1,Ordering::Relaxed);crate::RetainedWorkOutcome{result:Ok((SharedOriginalStorage{source:ManuallyDrop::new(Some(source))},RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
}
impl<T:Send+Sync+'static> SharedOriginalBirth<T>{
 pub fn publication(&mut self,grant:RetainedCloneGrant)->Result<Option<SharedOriginalPublication<'_,T>>,ValueError>{if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original birth storage already transferred"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}Ok(Some(SharedOriginalPublication{birth:self}))}
 pub fn next_close_demand(&self)->crate::RetirementDemand{if self.source.is_none(){Default::default()}else{crate::RetirementDemand{release_bytes:SharedOriginalStorage::<T>::allocation_bytes(),depth:1,..Default::default()}}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{if self.source.is_none(){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}let d=self.next_close_demand();if !crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}drop(self.source.take());crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
}
impl<T:Send+Sync+'static> Drop for SharedOriginalBirth<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"empty original birth requires granted publication or release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

pub enum SharedOriginalRelease<T:Send+Sync+'static>{Alias,Original(SharedOriginal<T>)}
pub struct SharedOriginal<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>,initialized:bool}

impl<T:Send+Sync+'static> SharedOriginalStorage<T>{
 pub fn admit(value:T,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,T)>{
  let turn=super::admit_shared_allocation(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))},grant);
  let result=match turn.result{Ok((source,progress))=>Ok((Self{source:ManuallyDrop::new(Some(source))},progress)),Err((error,allocation))=>Err((error,unsafe{allocation.value.into_inner().assume_init()}))};
  crate::RetainedWorkOutcome{result,work:turn.work}
 }
 /// 🧾️ Admits actual empty shared storage without taking a future payload.
 pub fn prepare_birth(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(SharedOriginalBirth<T>,RetainedCloneProgress)>{let turn=super::admit_shared_allocation(Allocation{aliases:AtomicUsize::new(0),value:UnsafeCell::new(MaybeUninit::uninit())},grant);crate::RetainedWorkOutcome{result:turn.result.map(|(source,p)|(SharedOriginalBirth{source:ManuallyDrop::new(Some(source))},p)).map_err(|(error,_)|error),work:turn.work}}

 pub fn allocation_bytes()->usize{arc_bytes::<Allocation<T>>()}
 pub fn get(&self)->&T{let source=self.source.as_ref().expect("live original alias");assert!(source.aliases.load(Ordering::Acquire)>0);unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original alias").value.get().cast::<T>()as usize}
 pub fn aliases(&self)->usize{self.source.as_ref().expect("live original alias").aliases.load(Ordering::Acquire)}
 pub fn duplicate(&self)->crate::RetainedWorkOutcome<Self>{
  let source=self.source.as_ref().expect("live original alias");
  let result=source.aliases.fetch_update(Ordering::AcqRel,Ordering::Acquire,|count|(count>0).then(||count.checked_add(1)).flatten()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias space exhausted")).map(|_|Self{source:ManuallyDrop::new(Some(Arc::clone(source)))});
  crate::RetainedWorkOutcome{result,work:1}
 }

 /// 🔗️ Adds one counted alias without allocating: the shared strong count is the only state touched.
 pub fn alias(&self)->Self{let source=self.source.as_ref().expect("live original alias");let strong=Arc::clone(source);source.aliases.fetch_add(1,Ordering::AcqRel);Self{source:ManuallyDrop::new(Some(strong))}}
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
