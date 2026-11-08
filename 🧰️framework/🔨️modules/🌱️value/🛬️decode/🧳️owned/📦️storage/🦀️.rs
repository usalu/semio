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
 pub fn reserve_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Option<NonNull<T>>>(),capacity_bytes:Layout::new::<T>().size(),depth:1,..Default::default()}}
 pub fn move_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Option<T>>()+std::mem::size_of::<T>()+std::mem::size_of::<bool>(),depth:1,..Default::default()}}
 pub fn release_demand(&self)->RetirementDemand{if self.pointer.is_none(){Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<Option<NonNull<T>>>(),release_bytes:Layout::new::<T>().size(),depth:1,..Default::default()}}}
 /// ⏱️ Quotes the actual backing release action.
 pub fn release_work_demand(&self)->u64{u64::from(self.pointer.is_some())}
 pub fn reserved(&self)->bool{self.pointer.is_some()}
 pub fn initialized(&self)->bool{self.initialized}
 pub fn address(&self)->Option<usize>{self.pointer.map(|p|p.as_ptr()as usize)}
 pub fn get(&self)->Option<&T>{if !self.initialized{return None}self.pointer.map(|pointer|unsafe{pointer.as_ref()})}
 pub fn get_mut(&mut self)->Option<&mut T>{if !self.initialized{return None}self.pointer.as_mut().map(|pointer|unsafe{pointer.as_mut()})}
 /// ⏱️ Quotes the original backing-allocation action before its effect.
 pub fn reserve_work_demand(&self)->u64{u64::from(!self.reserved())}
 /// 📦️ Retains the actual native backing and allocator marker in their original slots.
 pub fn reserve_empty(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  if self.reserved(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  let grant=effect.grant();let d=Self::reserve_demand();if !admitted(grant,d)||effect.maximum_work_units()<self.reserve_work_demand(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let layout=Layout::new::<T>();if layout.size()!=0&&effect.actual().is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("native backing requires its original empty allocator marker")),work:0})}let performed=match effect.performed(1){Ok(performed)=>performed,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:1})};if !performed{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original backing work exceeded its authority")),work:1})}let pointer=if layout.size()==0{Some(NonNull::dangling())}else{*effect.actual()=Some(0);let pointer=NonNull::new(unsafe{alloc(layout)}.cast::<T>());if pointer.is_some(){*effect.actual()=Some(layout.size())}pointer};
  let Some(pointer)=pointer else{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"native typed backing allocation refused")),work:1})};self.pointer=Some(pointer);crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()})),work:1})
 }
 /// 📐️ Quotes the original inline seed and its initialized-state write.
 pub fn seed_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<T>()+std::mem::size_of::<bool>(),depth:1,..Default::default()}}
 /// ⏱️ Quotes the actual pure seed publication work.
 pub fn seed_work_demand(&self)->u64{u64::from(self.reserved()&&!self.initialized)}
 /// 🌱️ Publishes the strict pure inline seed directly into its original resident backing.
 pub fn initialize_seed(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,seed:impl FnOnce()->Result<T,ValueError>)->crate::RetainedWorkPoll{
  if self.initialized{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("native backing already retains its seed")),work:0})}let Some(pointer)=self.pointer else{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("native seed requires its retained empty backing")),work:0})};let d=Self::seed_demand();if !admitted(effect.grant(),d)||effect.maximum_work_units()<self.seed_work_demand(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let performed=match effect.performed(1){Ok(performed)=>performed,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:1})};if !performed{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original backing work exceeded its authority")),work:1})}let value=match seed(){Ok(value)=>value,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:1})};unsafe{pointer.as_ptr().write(value)}self.initialized=true;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1})
 }
 pub fn put_from(&mut self,input:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if self.initialized{return RetainedWorkOutcome{result:Err(invalid("native typed backing already retains its initialized owner")),work:0}}let Some(pointer)=self.pointer else{return RetainedWorkOutcome{result:Err(invalid("native typed initialization requires its real retained backing")),work:0}};let d=Self::move_demand();if input.is_none()||!admitted(effect.grant(),d)||effect.maximum_work_units()==0{return pending()}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native typed initialization exceeded its work authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}unsafe{pointer.as_ptr().write(input.take().expect("original input selected"))};self.initialized=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 pub fn take_into(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.initialized{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if output.is_some(){return RetainedWorkOutcome{result:Err(invalid("native typed output slot already retains its owner")),work:0}}let Some(pointer)=self.pointer else{return RetainedWorkOutcome{result:Err(ValueError::invariant("pointer absent")),work:0}};let d=Self::move_demand();if !admitted(effect.grant(),d)||effect.maximum_work_units()==0{return pending()}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native typed publication exceeded its work authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}*output=Some(unsafe{pointer.as_ptr().read()});self.initialized=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 pub fn release(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if self.initialized{return RetainedWorkOutcome{result:Err(invalid("native typed backing retains its live initialized child")),work:0}}let Some(pointer)=self.pointer else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};let d=self.release_demand();if !admitted(effect.grant(),d)||effect.maximum_work_units()==0{return pending()}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original backing release exceeded its authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}let layout=Layout::new::<T>();if layout.size()!=0{unsafe{dealloc(pointer.as_ptr().cast(),layout)}}self.pointer=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}
 }
 /// 🪡️ Lends only admitted uninitialized storage; the typed field owner must track every live field before access.
 pub unsafe fn uninitialized_ptr(&mut self)->Option<*mut T>{if self.initialized{None}else{self.pointer.map(|p|p.as_ptr())}}
 /// 📐️ Quotes the original pointer, initialized flag and caller Box-slot writes.
 pub fn box_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Option<NonNull<T>>>()+std::mem::size_of::<bool>()+std::mem::size_of::<Option<Box<T>>>(),depth:1,..Default::default()}}
 /// ⏱️ Quotes the actual boxed publication action.
 pub fn box_work_demand(&self)->u64{u64::from(self.initialized)}
 /// 📤️ Transfers the same initialized backing directly into the caller's original output slot.
 pub fn take_box_into(&mut self,output:&mut Option<Box<T>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,ValueError>{
  if !self.initialized||output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("native boxed publication requires its original initialized backing and empty output")),work:0})}let d=Self::box_demand();if !admitted(effect.grant(),d)||effect.maximum_work_units()<self.box_work_demand(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original backing work exceeded its authority")),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:1})}let pointer=match self.pointer.take(){Some(pointer)=>pointer,None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("native boxed backing pointer is absent")),work:1})};self.initialized=false;*output=Some(unsafe{Box::from_raw(pointer.as_ptr())});crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1})
 }
 pub fn terminal_is_empty(&self)->bool{self.pointer.is_none()&&!self.initialized}
}
impl<T> Drop for NativeOwnedStorage<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native typed storage requires its actual paid release or box transfer");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📋️prefix/🦀️.rs"]mod prefix;pub use prefix::NativeOwnedPrefix;
#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod move_work_tests;
