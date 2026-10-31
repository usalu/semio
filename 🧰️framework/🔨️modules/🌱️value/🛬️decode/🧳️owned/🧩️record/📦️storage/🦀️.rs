//! 📦️ A concrete record retains typed backing and actual initialized-field custody through paid output or cancellation.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_native_storage::NativeOwnedStorage;
use std::mem::{MaybeUninit,size_of,align_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native record field layout or initialization frontier is invalid")}
/// 🧳️ Generated typed fields move through defining stores rather than an externally asserted empty-state flag.
pub struct NativeRecordStorage<T>{backing:NativeOwnedStorage<T>,fields:usize,live:usize,taken:bool,ready:bool}
impl<T>NativeRecordStorage<T>{
 pub const fn empty(fields:usize)->Self{Self{backing:NativeOwnedStorage::empty(),fields,live:0,taken:false,ready:false}}
 pub fn reserve_demand()->RetirementDemand{NativeOwnedStorage::<T>::reserve_demand()}
 pub fn initialize_demand()->RetirementDemand{NativeOwnedStorage::<T>::move_demand()}
 pub fn release_demand(&self)->RetirementDemand{self.backing.release_demand()}
 pub fn reserved(&self)->bool{self.backing.reserved()}
 pub fn initialized(&self)->bool{self.ready}
 pub fn taken(&self)->bool{self.taken}
 pub fn reserve(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.backing.reserve(g)}
 pub fn initialize(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if self.ready{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !self.backing.reserved(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !crate::retained_clone::retained_clone_grant_admits(g,Self::initialize_demand()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.ready=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 fn valid<F>(&self,offset:usize)->bool{!self.taken&&self.initialized()&&offset.checked_add(size_of::<F>()).is_some_and(|end|end<=size_of::<T>())&&offset%align_of::<F>()==0&&align_of::<T>()>=align_of::<F>()}
 /// 🪡️ Offset and F must name one actual uninitialized field of T; the genuine pending owner stays resident on denial.
 pub unsafe fn place_field<F>(&mut self,offset:usize,value:&mut Option<F>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let d=RetirementDemand{copy_bytes:size_of::<F>(),depth:1,..Default::default()};if !crate::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}if !self.valid::<F>(offset)||self.live>=self.fields||value.is_none(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}let field=value.take().unwrap();unsafe{let ptr=self.backing.uninitialized_ptr().unwrap().cast::<u8>().add(offset).cast::<F>();ptr.write(field);}self.live+=1;RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<F>(),..Default::default()})),work:1}}
 /// 🧹️ Offset and F must name one live field exactly once; its owner is retained before any later close admission.
 pub unsafe fn remove_field<F>(&mut self,offset:usize,output:&mut Option<F>,g:RetainedCloneGrant)->RetainedWorkOutcome{if !crate::retained_clone::retained_clone_grant_admits(g,RetirementDemand{depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if !self.valid::<F>(offset)||self.live==0||output.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}unsafe{let ptr=self.backing.uninitialized_ptr().unwrap().cast::<u8>().add(offset).cast::<F>();*output=Some(ptr.read());}self.live-=1;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 /// 📤️ Every exact declared field must have been placed before this single paid ownership handoff.
 pub unsafe fn take_initialized(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{if self.taken||!self.initialized()||!crate::retained_clone::retained_clone_grant_admits(g,RetirementDemand{depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(None),work:0}}if self.live!=self.fields{return RetainedWorkOutcome{result:Err(invalid()),work:0}}let value=unsafe{self.backing.uninitialized_ptr().unwrap().read()};self.taken=true;self.live=0;RetainedWorkOutcome{result:Ok(Some((value,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}}
 /// ♻️ The actual field counter must be empty before the exact typed allocation is released.
 pub fn release(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if self.live!=0{return RetainedWorkOutcome{result:Err(invalid()),work:0}}let turn=self.backing.release(g);if self.backing.terminal_is_empty(){self.ready=false}turn}
 pub fn terminal_is_empty(&self)->bool{self.backing.terminal_is_empty()&&self.live==0}
}
