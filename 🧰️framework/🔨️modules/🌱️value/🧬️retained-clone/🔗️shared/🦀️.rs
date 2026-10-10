//! 🔗️ Clones genuine original shared issuances through separately funded alias and projection turns.
use super::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::sealed::SealedShared}};
use std::mem::ManuallyDrop;

struct State<T:Send+Sync+'static>{output:Option<SealedShared<T>>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool}
pub struct SharedCloneCursor<T:Send+Sync+'static>{state:ManuallyDrop<State<T>>}
impl<T:Send+Sync+'static> Default for SharedCloneCursor<T>{fn default()->Self{Self{state:ManuallyDrop::new(State{output:None,source:None,close:Default::default(),closing:false,spent:false})}}}
impl<T:Send+Sync+'static> RetainedClone for SealedShared<T>{type Cursor=SharedCloneCursor<T>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<T:Send+Sync+'static> RetainedCloneCursor<SealedShared<T>> for SharedCloneCursor<T>{
 fn advance_demands(&self,source:RetainedCloneRef<'_,SealedShared<T>>,_body:usize)->Result<RetirementDemand,ValueError>{if self.state.closing||self.state.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone is closing or spent"));}Ok(RetirementDemand{copy_bytes:if self.state.source.is_none(){source.binding_copy_bytes()}else if self.state.output.is_none(){size_of::<SealedShared<T>>()}else{0},depth:1,..Default::default()})}
 fn advance(&mut self,source:RetainedCloneRef<'_,SealedShared<T>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let state=&mut *self.state;if state.closing||state.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone is closing or spent"));}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(progress)=source.bind(&mut state.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.output.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_copy_bytes<size_of::<SealedShared<T>>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let(output,progress)=source.get().try_duplicate(grant)?;state.output=Some(output);Ok(RetainedCloneStep::Complete(progress))
 }
 fn take(&mut self)->Option<SealedShared<T>>{if self.state.closing||self.state.spent{return None;}let output=self.state.output.take();if output.is_some(){self.state.spent=true;}output}
 fn begin_close(&mut self)->bool{if self.state.closing{return false;}self.state.closing=true;true}
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_copy_with_binding(&self.state.source)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.state.close.next_owner_capacity_with_binding::<SealedShared<T>>(self.state.output.is_some(),body,&self.state.source)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_release_with_binding(&self.state.source)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.state.close.next_owner_depth_with_binding(self.state.output.is_some(),&self.state.source)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let state=&mut *self.state;if !state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone requires closing intent"));}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !state.close.is_empty(){return state.close.step_granted(grant);}
  if let Some(step)=state.close.begin_granted(&mut state.output,grant)?{return Ok(step);}
  super::close_retained_binding(&mut state.source,grant)
 }
}
impl<T:Send+Sync+'static> Drop for SharedCloneCursor<T>{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original shared clone retains unpublished output or source binding")}}}

struct SharedCloneRetirement<T:Send+Sync+'static>{cursor:ManuallyDrop<SharedCloneCursor<T>>}
impl<T:Send+Sync+'static> RetirementCursor for SharedCloneRetirement<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.cursor.close_step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.cursor.next_close_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.cursor.next_close_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_depth_demand()}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for SharedCloneRetirement<T>{fn drop(&mut self){if self.cursor.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}else if !std::thread::panicking(){panic!("original shared clone retirement frame remains retained")}}}
impl<T:Send+Sync+'static> RetireOwned for SharedCloneCursor<T>{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(SharedCloneRetirement{cursor:ManuallyDrop::new(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedCloneRetirement<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
