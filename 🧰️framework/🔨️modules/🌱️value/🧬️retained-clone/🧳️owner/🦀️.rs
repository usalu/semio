//! 🧳️ Typed clone-cursor custody exposes genuine selected quotes and controlled partial-owner closure.
use super::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress};
use crate::{ValueError,RetirementDemand,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::ManuallyDrop;

pub struct RetainedCloneOwner<T:RetainedClone>{cursor:ManuallyDrop<T::Cursor>,closing:bool}
impl<T:RetainedClone> Default for RetainedCloneOwner<T>{fn default()->Self{Self{cursor:ManuallyDrop::new(T::retained_clone_cursor()),closing:false}}}
impl<T:RetainedClone> RetainedCloneOwner<T>{
 pub fn advance_demands(&self,source:RetainedCloneRef<'_,T>,copy:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.cursor.advance_demands(source,copy,maximum_work_units)}
 /// ⛽️ Quotes the selected original child work.
 pub fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,copy:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.cursor.advance_work_demand(source,copy,maximum_work_units)}
 pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,body: usize, maximum_work_units:u64, grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{{let turn=self.cursor.advance(source,body,maximum_work_units,grant);work+=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.cursor.take_demand()}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{self.cursor.take(grant)}
 pub fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 pub fn selected_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:self.cursor.next_close_copy_byte_demand()?,capacity_bytes:self.cursor.next_close_capacity_byte_demand(body)?,release_bytes:self.cursor.next_close_release_byte_demand()?,depth:self.cursor.next_close_depth_demand()?})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.closing{self.cursor.begin_close();self.closing=true;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}{let turn=self.cursor.close_step(grant);work+=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
}
impl<T:RetainedClone> Drop for RetainedCloneOwner<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed clone cursor requires original partial-owner closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}}}
struct Close<T:RetainedClone>{owner:RetainedCloneOwner<T>}
impl<T:RetainedClone> RetirementCursor for Close<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.selected_close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.owner.closing&&self.owner.terminal_is_empty(){return RetirementStep::Complete(0)}let turn=self.owner.close_step(grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{self.owner.closing&&self.owner.terminal_is_empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.selected_close_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.selected_close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetainedClone> RetireOwned for RetainedCloneOwner<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Close{owner:self})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Close<T>>())}fn controlled_retirement_supported()->bool{true}}
