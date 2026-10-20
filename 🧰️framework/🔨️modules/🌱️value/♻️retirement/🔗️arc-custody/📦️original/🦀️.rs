//! 📦️ Shared initialized originals elect one destructive owner without moving the complete payload.
use super::arc_bytes;
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::{cell::UnsafeCell,mem::{ManuallyDrop,MaybeUninit},sync::{Arc,atomic::{AtomicUsize,Ordering}}};

struct Allocation<T>{aliases:AtomicUsize,value:UnsafeCell<MaybeUninit<T>>}
/// 🪢️ Live aliases expose immutable payloads; only the elected zero-alias owner mutates initialized fields.
unsafe impl<T:Send+Sync> Sync for Allocation<T>{}

pub struct SharedOriginalStorage<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>}
pub enum SharedOriginalRelease<T:Send+Sync+'static>{Alias,Original(SharedOriginal<T>)}
pub struct SharedOriginal<T:Send+Sync+'static>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>,initialized:bool}

impl<T:Send+Sync+'static> SharedOriginalStorage<T>{
 fn new(value:T)->Self{Self{source:ManuallyDrop::new(Some(Arc::new(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))})))}}
 pub fn admit(value:T,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,T)>{if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<Self::allocation_bytes(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"initialized original storage birth is not funded"),value));}Ok((Self::new(value),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:Self::allocation_bytes(),..Default::default()}))}
 pub fn allocation_bytes()->usize{arc_bytes::<Allocation<T>>()}
 pub fn get(&self)->&T{let source=self.source.as_ref().expect("live original alias");assert!(source.aliases.load(Ordering::Acquire)>0);unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original alias").value.get().cast::<T>()as usize}
 pub fn aliases(&self)->usize{self.source.as_ref().expect("live original alias").aliases.load(Ordering::Acquire)}
 pub fn duplicate(&self)->Result<Self,ValueError>{let source=self.source.as_ref().expect("live original alias");source.aliases.fetch_update(Ordering::AcqRel,Ordering::Acquire,|count|(count>0).then(||count.checked_add(1)).flatten()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias space exhausted"))?;Ok(Self{source:ManuallyDrop::new(Some(Arc::clone(source)))})}
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
