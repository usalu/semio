//! 📦️ Typed native backing is born, initialized and released under distinct actual work turns.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::{alloc::{alloc,dealloc,Layout},ptr::NonNull};
pub struct NativeOwnedStorage<T>{pointer:Option<NonNull<T>>,initialized:bool}
unsafe impl<T:Send> Send for NativeOwnedStorage<T>{}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn pending()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
fn admitted(grant:RetainedCloneGrant,d:RetirementDemand)->bool{crate::retained_clone_grant_admits(grant,d)}
impl<T> NativeOwnedStorage<T>{
 pub const fn empty()->Self{Self{pointer:None,initialized:false}}
 pub fn reserve_demand()->RetirementDemand{RetirementDemand{capacity_bytes:Layout::new::<T>().size(),depth:1,..Default::default()}}
 pub fn move_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub fn release_demand(&self)->RetirementDemand{if self.pointer.is_none(){Default::default()}else{RetirementDemand{release_bytes:Layout::new::<T>().size(),depth:1,..Default::default()}}}
 pub fn reserved(&self)->bool{self.pointer.is_some()}
 pub fn initialized(&self)->bool{self.initialized}
 pub fn address(&self)->Option<usize>{self.pointer.map(|p|p.as_ptr()as usize)}
 pub fn get(&self)->Option<&T>{if !self.initialized{return None}Some(unsafe{self.pointer.unwrap().as_ref()})}
 pub fn get_mut(&mut self)->Option<&mut T>{if !self.initialized{return None}Some(unsafe{self.pointer.as_mut().unwrap().as_mut()})}
 pub fn reserve(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.reserved(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=Self::reserve_demand();if !admitted(grant,d){return pending()}let layout=Layout::new::<T>();let pointer=if layout.size()==0{Some(NonNull::dangling())}else{NonNull::new(unsafe{alloc(layout)}.cast::<T>())};let Some(pointer)=pointer else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"native typed backing allocation refused")),work:1}};self.pointer=Some(pointer);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})),work:1}
 }
 pub fn put_from(&mut self,input:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.initialized{return RetainedWorkOutcome{result:Err(invalid("native typed backing already retains its initialized owner")),work:0}}let Some(pointer)=self.pointer else{return RetainedWorkOutcome{result:Err(invalid("native typed initialization requires its real retained backing")),work:0}};if input.is_none(){return pending()}if !admitted(grant,Self::move_demand()){return pending()}unsafe{pointer.as_ptr().write(match input.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("input absent").into()),work:0}})};self.initialized=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.initialized{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if output.is_some(){return RetainedWorkOutcome{result:Err(invalid("native typed output slot already retains its owner")),work:0}}if !admitted(grant,Self::move_demand()){return pending()}*output=Some(unsafe{match self.pointer{Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("pointer absent").into()),work:0}}.as_ptr().read()});self.initialized=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn release(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.initialized{return RetainedWorkOutcome{result:Err(invalid("native typed backing retains its live initialized child")),work:0}}let Some(pointer)=self.pointer else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};let d=self.release_demand();if !admitted(grant,d){return pending()}let layout=Layout::new::<T>();if layout.size()!=0{unsafe{dealloc(pointer.as_ptr().cast(),layout)}}self.pointer=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}
 }
 /// 🪡️ Lends only admitted uninitialized storage; the typed field owner must track every live field before access.
 pub unsafe fn uninitialized_ptr(&mut self)->Option<*mut T>{if self.initialized{None}else{self.pointer.map(|p|p.as_ptr())}}
 pub fn take_box(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<T>,RetainedCloneProgress)>>{
  if !self.initialized{return RetainedWorkOutcome{result:Ok(None),work:0}}if !admitted(grant,RetirementDemand{depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(None),work:0}}let pointer=match self.pointer.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("pointer absent").into()),work:0}};self.initialized=false;let output=unsafe{Box::from_raw(pointer.as_ptr())};RetainedWorkOutcome{result:Ok(Some((output,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{self.pointer.is_none()&&!self.initialized}
}
impl<T> Drop for NativeOwnedStorage<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native typed storage requires its actual paid release or box transfer");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📋️prefix/🦀️.rs"]mod prefix;pub use prefix::NativeOwnedPrefix;
#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod move_work_tests;
