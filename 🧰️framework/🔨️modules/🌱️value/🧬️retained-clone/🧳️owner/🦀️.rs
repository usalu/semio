//! 🧳️ Typed clone-cursor custody exposes genuine selected quotes and controlled partial-owner closure.
use super::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress};
use crate::{ValueError,RetirementDemand,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::ManuallyDrop;

pub struct RetainedCloneOwner<T:RetainedClone>{cursor:ManuallyDrop<T::Cursor>,closing:bool}
impl<T:RetainedClone> Default for RetainedCloneOwner<T>{fn default()->Self{Self{cursor:ManuallyDrop::new(T::retained_clone_cursor()),closing:false}}}
impl<T:RetainedClone> RetainedCloneOwner<T>{
 pub fn advance_demands(&self,source:RetainedCloneRef<'_,T>,copy:usize)->Result<RetirementDemand,ValueError>{self.cursor.advance_demands(source,copy)}
 pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.cursor.advance(source,grant)}
 pub fn take(&mut self)->Option<T>{self.cursor.take()}
 pub fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 pub fn selected_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:self.cursor.next_close_copy_byte_demand()?,capacity_bytes:self.cursor.next_close_capacity_byte_demand(body)?,release_bytes:self.cursor.next_close_release_byte_demand()?,depth:self.cursor.next_close_depth_demand()?})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.closing{self.cursor.begin_close();self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}self.cursor.close_step(grant)}
}
impl<T:RetainedClone> Drop for RetainedCloneOwner<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed clone cursor requires original partial-owner closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}}}
struct Close<T:RetainedClone>{owner:RetainedCloneOwner<T>}
impl<T:RetainedClone> RetirementCursor for Close<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.owner.closing&&self.owner.terminal_is_empty(){return RetirementStep::Complete}match self.owner.close_step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
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
