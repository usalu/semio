//! 📦️ A concrete record retains typed backing and actual initialized-field custody through paid publication or cancellation.
use crate::{NativeOwnedStorage,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetainedAllocationEffect,ValueError,ValueRefusalKind};
use std::mem::{size_of,align_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native record field layout or initialization frontier is invalid")}
fn performed(effect:&mut RetainedAllocationEffect<'_>)->Result<u128,(ValueError,u128)>{let before=effect.performed_work();let verdict=effect.performed(1);let work=effect.performed_work()-before;match verdict{Ok(true)=>Ok(work),Ok(false)=>Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original record operation exceeded its work authority"),work)),Err(error)=>Err((error,work))}}
fn admitted(effect:&RetainedAllocationEffect<'_>,demand:RetirementDemand)->bool{effect.maximum_work_units()!=0&&crate::retained_clone_grant_admits(effect.grant(),demand)}
pub struct NativeRecordStorage<T>{backing:NativeOwnedStorage<T>,fields:usize,live:usize,taken:bool,ready:bool}
impl<T>NativeRecordStorage<T>{
 pub const fn empty(fields:usize)->Self{Self{backing:NativeOwnedStorage::empty(),fields,live:0,taken:false,ready:false}}
 pub fn reserve_demand()->RetirementDemand{NativeOwnedStorage::<T>::reserve_demand()}
 pub fn initialize_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}
 pub fn release_demand(&self)->RetirementDemand{let mut demand=self.backing.release_demand();if self.backing.reserved()&&self.ready{demand.copy_bytes+=size_of::<bool>()}demand}
 pub fn reserved(&self)->bool{self.backing.reserved()}
 pub fn initialized(&self)->bool{self.ready}
 pub fn taken(&self)->bool{self.taken}
 pub fn reserve_work_demand(&self)->u64{self.backing.reserve_work_demand()}
 pub fn reserve_empty(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{self.backing.reserve_empty(effect)}
 /// 🌱️ Publishes only the original empty record initialization frontier.
 pub fn initialize(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if self.ready{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !self.backing.reserved(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  let demand=Self::initialize_demand();if !admitted(effect,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let work=match performed(effect){Ok(work)=>work,Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}};
  self.ready=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work}
 }
 fn valid<F>(&self,offset:usize)->bool{!self.taken&&self.initialized()&&offset.checked_add(size_of::<F>()).is_some_and(|end|end<=size_of::<T>())&&offset%align_of::<F>()==0&&align_of::<T>()>=align_of::<F>()}
 /// 📐️ Quotes the original field, its resident option and the record frontier.
 pub fn field_demand<F>(extra:usize)->Result<RetirementDemand,ValueError>{let copy_bytes=size_of::<F>().checked_add(size_of::<Option<F>>()).and_then(|n|n.checked_add(size_of::<usize>())).and_then(|n|n.checked_add(extra)).ok_or_else(invalid)?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 🪡️ Moves one original field into admitted uninitialized record storage.
 /// # Safety
 /// The offset must name an uninitialized field of exactly F in T.
 pub unsafe fn place_field<F>(&mut self,offset:usize,value:&mut Option<F>,extra:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{
  if !self.valid::<F>(offset)||self.live>=self.fields||value.is_none(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  let demand=match Self::field_demand::<F>(extra){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !admitted(effect,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
  let pointer=match self.backing.uninitialized_ptr(){Some(pointer)=>unsafe{pointer.cast::<u8>().add(offset).cast::<F>()},None=>return RetainedWorkOutcome{result:Err(invalid()),work:0}};let work=match performed(effect){Ok(work)=>work,Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}};
  unsafe{pointer.write(value.take().expect("original field remains resident"))}self.live+=1;RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work}
 }
 /// 🧵️ Transfers one initialized original field into its caller's resident destination.
 /// # Safety
 /// The offset must name an initialized field of exactly F in T.
 pub unsafe fn remove_field<F>(&mut self,offset:usize,output:&mut Option<F>,extra:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.valid::<F>(offset)||self.live==0||output.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  let demand=match Self::field_demand::<F>(extra){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !admitted(effect,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let pointer=match self.backing.uninitialized_ptr(){Some(pointer)=>unsafe{pointer.cast::<u8>().add(offset).cast::<F>()},None=>return RetainedWorkOutcome{result:Err(invalid()),work:0}};let work=match performed(effect){Ok(work)=>work,Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}};
  *output=Some(unsafe{pointer.read()});self.live-=1;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work}
 }
 /// 📐️ Quotes the original value, destination and initialization headers before publication.
 pub fn take_initialized_demand(extra:usize)->Result<RetirementDemand,ValueError>{let copy_bytes=size_of::<T>().checked_add(size_of::<Option<T>>()).and_then(|n|n.checked_add(size_of::<bool>())).and_then(|n|n.checked_add(size_of::<usize>())).and_then(|n|n.checked_add(extra)).ok_or_else(invalid)?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 📤️ Publishes only a fully initialized original record into its resident destination.
 /// # Safety
 /// Every field of T must be initialized exactly once and ready for transfer.
 pub unsafe fn take_initialized_into(&mut self,output:&mut Option<T>,extra:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{
  if self.taken||!self.initialized()||self.live!=self.fields||output.is_some(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid()),work:0})}
  let demand=match Self::take_initialized_demand(extra){Ok(demand)=>demand,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if !admitted(effect,demand){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let pointer=match self.backing.uninitialized_ptr(){Some(pointer)=>pointer,None=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid()),work:0})};let work=match performed(effect){Ok(work)=>work,Err((error,work))=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})};
  *output=Some(unsafe{pointer.read()});self.taken=true;self.live=0;RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work})
 }
 /// 🧹️ Releases empty backing and its initialized-state header through the same original receipt.
 pub fn release(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if self.live!=0{return RetainedWorkOutcome{result:Err(invalid()),work:0}}let demand=self.release_demand();if self.backing.reserved()&&!admitted(effect,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let mut turn=self.backing.release(effect);if let Ok(RetainedCloneStep::Complete(progress))=&mut turn.result{if self.ready{self.ready=false;progress.copied_bytes+=size_of::<bool>()}}turn
 }
 pub fn terminal_is_empty(&self)->bool{self.backing.terminal_is_empty()&&self.live==0}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
