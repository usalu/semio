//! 📦️ Unique original Box custody separates field transfer from physical shell release.
use super::{RetireOwned,RetirementCursor,RetirementStep};
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,MaybeUninit,size_of};
/// 🎟️ Only the actual owned Box can admit this unique source; no borrowed reference can mint it.
pub struct BoxOriginal<T:Send+'static>{source:ManuallyDrop<Option<Box<MaybeUninit<T>>>>,initialized:bool}
impl<T:Send+'static> BoxOriginal<T>{
 /// 📥️ Transfers the unchanged allocation and pointer without reading or allocating its payload.
 pub fn from_original(original:Box<T>)->Self{let pointer=Box::into_raw(original).cast::<MaybeUninit<T>>();Self{source:ManuallyDrop::new(Some(unsafe{Box::from_raw(pointer)})),initialized:true}}
 pub fn allocation_bytes()->usize{size_of::<T>()}
 pub fn identity(&self)->usize{self.source.as_ref().expect("original Box shell remains owned").as_ptr()as usize}
 pub fn get(&self)->&T{assert!(self.initialized,"partially transferred original Box cannot expose a whole initialized value");unsafe{self.source.as_ref().expect("live original Box").assume_init_ref()}}
 pub fn is_initialized(&self)->bool{self.initialized}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 👁️ Borrows one still-initialized aligned field inside this original allocation.
 /// # Safety
 /// The projection must name an initialized field inside T and cannot revisit transferred fields.
 pub unsafe fn field<F>(&self,project:fn(*const T)->*const F)->&F{unsafe{&*project(self.source.as_ref().expect("live original Box").as_ptr())}}
 /// 📤️ Transfers one initialized nonoverlapping field while retaining the exact original shell.
 /// # Safety
 /// The projection must name that field inside T exactly once; later operations cannot read or drop it.
 pub unsafe fn take_field<F>(&mut self,project:fn(*mut T)->*mut F)->F{self.initialized=false;unsafe{project(self.source.as_mut().expect("live original Box").as_mut_ptr()).read()}}
 /// 📤️ Transfers a fully initialized bounded value after its complete copy and child birth were admitted.
 /// # Safety
 /// The caller must retain the shell and never query the transferred value again.
 pub unsafe fn take_whole(&mut self)->T{assert!(self.initialized);self.initialized=false;unsafe{self.source.as_ref().expect("live original Box").assume_init_read()}}
 /// ♻️ Releases only the original allocation after all owned payload has transferred.
 /// # Safety
 /// Every owned field must have transferred or reached its defining terminal-empty witness.
 pub unsafe fn release_empty(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{if self.source.is_none()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<size_of::<T>(){return Default::default()}self.initialized=false;drop(self.source.take());RetainedCloneProgress{copied_items:1,released_bytes:size_of::<T>(),..Default::default()}}
}
impl<T:Send+'static> Drop for BoxOriginal<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"unique original Box retains its shell until every initialized field transfers")}}
#[doc(hidden)]
pub struct BoxedOwner<T:RetireOwned>{original:Option<BoxOriginal<T>>,pending:ManuallyDrop<Option<T>>}
impl<T:RetireOwned> BoxedOwner<T>{pub(super) fn new(original:Box<T>)->Self{Self{original:Some(BoxOriginal::from_original(original)),pending:ManuallyDrop::new(None)}}}
impl<T:RetireOwned> RetirementCursor for BoxedOwner<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_demand(body)?;Ok(u64::from(!self.terminal_is_empty()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if let Some(value)=self.pending.as_ref(){return super::retirement_cursor_birth_demand(value)}let Some(original)=self.original.as_ref()else{return Ok(Default::default())};if !original.is_initialized(){return Ok(RetirementDemand{release_bytes:BoxOriginal::<T>::allocation_bytes(),depth:1,..Default::default()})}let value=original.get();if let Some(demand)=value.boxed_original_demands()?{return Ok(demand)}Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<T>()+size_of::<bool>(),depth:1,..Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  if self.pending.is_some(){return super::retirement_input_step(&mut self.pending,effect)}
  let Some(original)=self.original.as_mut()else{return RetirementStep::Complete(0)};
  if !original.is_initialized(){
   let p=unsafe{original.release_empty(grant)};self.original=None;
   return RetirementStep::Progress(p,1)
  }
  let specialized=match original.get().boxed_original_demands(){Ok(d)=>d.is_some(),Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if specialized{
   let turn=T::boxed_original_retirement(&mut self.original,effect);
   return match turn.result{
    Ok(Some((cursor,p)))=>RetirementStep::ChildProgress(cursor,p,turn.work),
    Ok(None)=>RetirementStep::Progress(Default::default(),turn.work),
    Err(error)=>RetirementStep::Failure(error.into(),turn.work)
   }
  }
  *self.pending=Some(unsafe{original.take_whole()});
  RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
