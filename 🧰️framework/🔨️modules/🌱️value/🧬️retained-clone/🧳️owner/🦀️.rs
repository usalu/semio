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
 pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,body: usize, maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{{let turn=self.cursor.advance(source,body,maximum_work_units,&mut effect.nested_within(grant,maximum_work_units));work+=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.cursor.take_demand()}
 pub fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{self.cursor.take(output,effect,reply)}
 pub fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 pub fn selected_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}if !self.cursor.close_started(){return self.cursor.begin_close_demand()}Ok(RetirementDemand{copy_bytes:self.cursor.next_close_copy_byte_demand()?,capacity_bytes:self.cursor.next_close_capacity_byte_demand(body)?,release_bytes:self.cursor.next_close_release_byte_demand()?,depth:self.cursor.next_close_depth_demand()?})}
 /// ⛽️ Parent intent and child start are separate original actions, each admitting one-work loans.
 pub fn selected_close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(1)}if !self.cursor.close_started(){return self.cursor.begin_close_work_demand()}self.cursor.close_work_demand(body)}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();
  let mut work=0;let result=(||->Result<RetainedCloneStep,crate::RetirementPollExit>{
   if self.closing&&self.cursor.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let required=self.selected_close_work_demand(grant.maximum_copy_bytes)?;if effect.maximum_work_units()<required{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.selected_close_demand(grant.maximum_copy_bytes)?;if !super::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.closing{self.closing=true;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))}
   if !self.cursor.close_started(){let turn=self.cursor.begin_close_paid(effect);work=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
   let turn=crate::RetirementPollExit::settled(self.cursor.close_step(&mut effect.nested(grant)))?;work=turn.work;turn.result.map_err(Into::into)
  })();crate::RetirementPollExit::finish(result,work)
 }

}
impl<T:RetainedClone> Drop for RetainedCloneOwner<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed clone cursor requires original partial-owner closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}}}
#[doc(hidden)]
pub struct RetainedCloneOwnerRetirement<T:RetainedClone>{owner:RetainedCloneOwner<T>}
impl<T:RetainedClone> RetirementCursor for RetainedCloneOwnerRetirement<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.selected_close_demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.owner.selected_close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if self.owner.closing&&self.owner.terminal_is_empty(){return RetirementStep::Complete(0)}crate::retirement::erased_cursor_turn(self.owner.close_step(&mut effect.nested(grant)))}
 fn terminal_is_empty(&self)->bool{self.owner.closing&&self.owner.terminal_is_empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.owner.selected_close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.selected_close_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.selected_close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetainedClone> RetireOwned for RetainedCloneOwner<T>{type RetirementCursor=RetainedCloneOwnerRetirement<T>;fn retirement_seed(self)->Self::RetirementCursor{RetainedCloneOwnerRetirement{owner:self}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}fn controlled_retirement_supported()->bool{true}}
