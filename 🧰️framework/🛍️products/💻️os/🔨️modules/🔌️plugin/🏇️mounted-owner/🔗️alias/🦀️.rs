//! 🔗️ Original mutable owner aliases retain final payload and box-shell custody under caller authority.
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop,sync::{Arc,atomic::{AtomicU8,Ordering}},cell::UnsafeCell,ops::{Deref,DerefMut}};

/// 🚪️ An inline original mutable-owner gate has no hidden operating-system allocation.
pub struct MountedOwnerCellV1<T:?Sized+Send>{state:AtomicU8,owner:UnsafeCell<Box<T>>}
/// 🚦️ Busy or poisoned gates preserve the same concrete owner without allocating an error.
#[derive(Debug,PartialEq,Eq)]
pub enum MountedOwnerCellErrorV1{Busy,Poisoned}
/// 🔒️ A borrowed original exclusive lease releases only its inline gate state.
pub struct MountedOwnerCellGuardV1<'a,T:?Sized+Send>{cell:&'a MountedOwnerCellV1<T>,marker:std::marker::PhantomData<&'a mut T>}
impl<T:?Sized+Send> MountedOwnerCellV1<T>{
 /// 📥️ Retains the exact supplied concrete box in an inline exclusive gate.
 pub fn new(owner:Box<T>)->Self{Self{state:AtomicU8::new(0),owner:UnsafeCell::new(owner)}}
 /// 🎟️ Borrows one original mutable lease without blocking or creating native mutex backing.
 pub fn try_lock(&self)->Result<MountedOwnerCellGuardV1<'_,T>,MountedOwnerCellErrorV1>{match self.state.compare_exchange(0,1,Ordering::Acquire,Ordering::Relaxed){Ok(_)=>Ok(MountedOwnerCellGuardV1{cell:self,marker:std::marker::PhantomData}),Err(2)=>Err(MountedOwnerCellErrorV1::Poisoned),Err(_)=>Err(MountedOwnerCellErrorV1::Busy)}}
 /// 📤️ Final exclusive ownership moves the exact original box without releasing a payload or shell.
 pub fn into_inner(self)->Box<T>{self.owner.into_inner()}
}
unsafe impl<T:?Sized+Send> Send for MountedOwnerCellV1<T>{}
unsafe impl<T:?Sized+Send> Sync for MountedOwnerCellV1<T>{}
impl<T:?Sized+Send> Deref for MountedOwnerCellGuardV1<'_,T>{type Target=Box<T>;fn deref(&self)->&Box<T>{unsafe{&*self.cell.owner.get()}}}
impl<T:?Sized+Send> DerefMut for MountedOwnerCellGuardV1<'_,T>{fn deref_mut(&mut self)->&mut Box<T>{unsafe{&mut*self.cell.owner.get()}}}
impl<T:?Sized+Send> Drop for MountedOwnerCellGuardV1<'_,T>{fn drop(&mut self){self.cell.state.store(if std::thread::panicking(){2}else{0},Ordering::Release)}}

/// 🧳️ An inline original alias frontier never closes a payload still owned by another instance.
pub struct MountedOwnerAliasRetirementV1<T:?Sized+Send> {source:ManuallyDrop<Option<Arc<MountedOwnerCellV1<T>>>>,unique:ManuallyDrop<Option<Box<T>>>}
impl<T:?Sized+Send> MountedOwnerAliasRetirementV1<T> {
 /// 📥️ Transfers exactly one original alias without cloning or allocating a cursor frame.
 pub fn new(source:Arc<MountedOwnerCellV1<T>>)->Self {Self{source:ManuallyDrop::new(Some(source)),unique:ManuallyDrop::new(None)}}
 /// 📏️ Inline source transfer requires depth but no physical heap allocation or copy.
 pub const fn constructor_demands()->RetirementDemand {RetirementDemand{copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1}}
 /// 🔎️ Borrows the exact final concrete owner after atomic alias extraction.
 pub fn unique(&self)->Option<&T> {self.unique.as_deref()}
 /// 🔧️ Borrows the exact final concrete owner for its original funded lifecycle.
 pub fn unique_mut(&mut self)->Option<&mut T> {self.unique.as_deref_mut()}
 /// 📏️ Quotes only the next original shared backing or final concrete box shell.
 pub fn demands(&self)->RetirementDemand {
  let release_bytes=if self.source.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<MountedOwnerCellV1<T>>()}else{self.unique.as_deref().map_or(0,std::mem::size_of_val)};
  RetirementDemand{release_bytes,depth:usize::from(!self.terminal_is_empty()),..Default::default()}
 }
 /// 🧾️ Advances one actual backing or terminal shell; nonfinal aliases never close shared payloads.
 pub fn step(&mut self,grant:RetainedCloneGrant,payload_is_empty:impl FnOnce(&T)->bool)->Result<RetainedCloneStep,ValueError> {
  let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
  let demand=self.demands();if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
  if let Some(source)=self.source.as_ref(){
   if Arc::weak_count(source)!=0{return Ok(RetainedCloneStep::Progress(empty));}
   let unique=Arc::into_inner(self.source.take().expect("original alias remains retained"));let released_bytes=if let Some(owner)=unique{*self.unique=Some(owner.into_inner());demand.release_bytes}else{0};
   let progress=RetainedCloneProgress{copied_items:1,released_bytes,..empty};return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)});
  }
  if !payload_is_empty(self.unique.as_deref().expect("original final payload remains retained")){return Ok(RetainedCloneStep::Progress(empty));}
  drop(self.unique.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}))
 }
 /// 🏁️ Completion requires both the original alias and its final concrete box to be physically absent.
 pub fn terminal_is_empty(&self)->bool {self.source.is_none()&&self.unique.is_none()}
}
impl<T:?Sized+Send> Drop for MountedOwnerAliasRetirementV1<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original mounted alias abandoned final owner custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.unique);}}}}
