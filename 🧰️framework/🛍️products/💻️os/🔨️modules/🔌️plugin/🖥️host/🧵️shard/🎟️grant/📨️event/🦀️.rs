//! 📨️ One selected Event stays owned until its guest finishes and its physical source closes.
use semio_framework_actor::{OriginalActorInput,RetainedTurnInput,RetainedTurnReceipt,ActorGuestPhase};
use semio_framework::kernel::{Event,TurnResult};
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::ManuallyDrop;

pub(super) struct OriginalShardEvent{pub(super) input:OriginalActorInput<Event>,pending:ManuallyDrop<Option<TurnResult>>,retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,frame_bytes:usize,trap:ManuallyDrop<Option<crate::TurnFault>>,failed:Option<crate::original_actor_context::OriginalActorFixedCall>}
impl std::fmt::Debug for OriginalShardEvent{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalShardEvent").field("input",&self.input).field("pending",&self.pending.is_some()).field("retirement",&self.retirement.is_some()).finish()}}
impl OriginalShardEvent{
 pub(super) fn new(event:Event,issued:RetainedTurnInput)->Result<Self,(ValueError,Event)>{OriginalActorInput::new(event,issued).map(|input|Self{input,pending:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),frame_bytes:0,trap:ManuallyDrop::new(None),failed:None})}
 pub(super) fn receive_next_issued(&mut self,input:RetainedTurnInput)->Result<(),ValueError>{self.input.receive_next_issued(input)}
 pub(super) fn is_retiring(&self)->bool{self.pending.is_some()}
 pub(super) fn receive_dispatched_result(&mut self,result:TurnResult)->Result<RetainedTurnReceipt,ValueError>{assert!(self.pending.is_none(),"original Event has one returned result");*self.pending=Some(result);let receipt=self.pending.as_ref().unwrap().retained_receipt;self.input.receive_guest(receipt,ActorGuestPhase::Dispatched)?;self.input.return_actor_turn()}
 pub(super) fn take_received_more_work(&mut self,receipt:RetainedTurnReceipt)->Option<TurnResult>{if !self.pending.as_ref().is_some_and(|result|matches!(result.status,semio_framework::kernel::TurnStatus::MoreWork)){return None}let mut result=self.pending.take()?;result.retained_receipt=receipt;Some(result)}
 pub(super) fn receive_failed_call(&mut self,call:crate::original_actor_context::OriginalActorFixedCall)->Result<RetainedTurnReceipt,ValueError>{assert!(self.failed.is_none(),"original Event has one returned failure");self.failed=Some(call);self.input.receive_guest_spent(call.input,call.spent,ActorGuestPhase::Dispatched)?;self.input.return_actor_turn()}
 pub(super) fn retain_trap(&mut self,trap:crate::TurnFault){assert!(self.trap.is_none(),"original Event has one unreturned trap");*self.trap=Some(trap);}
 pub(super) fn terminal_is_empty(&self)->bool{self.input.original().is_none()&&self.retirement.is_none()&&self.pending.is_none()&&self.trap.is_none()&&self.failed.is_none()}
 pub(super) fn source_close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.pending.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Event has no terminal guest result"))}
  if let Some(cursor)=self.retirement.as_mut(){return cursor.close_step(grant).map(|step|step.progress())}
  if self.input.original().is_some(){let bytes=semio_framework_value::retirement::owned_retirement_birth_bytes::<Event>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(Default::default())}let event=self.input.take_dispatched_input()?.expect("quoted original Event source");return match semio_framework_value::retirement::admit_owned_retirement(event,grant){Ok((cursor,progress))=>{self.frame_bytes=progress.retained_capacity_bytes;*self.retirement=Some(cursor);Ok(progress)},Err((error,event))=>{self.input.restore_dispatched_input(event).unwrap_or_else(|_|panic!("refused original Event retirement returns its exact source"));Err(error)}}}
  Ok(Default::default())
 }
 pub(super) fn release_empty_cursor(&mut self,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{let cursor=self.retirement.as_ref()?;if !cursor.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<self.frame_bytes{return None}let bytes=self.frame_bytes;drop(self.retirement.take());self.frame_bytes=0;Some(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})}
 pub(super) fn take_closed_result(&mut self,receipt:RetainedTurnReceipt)->Option<TurnResult>{if self.input.original().is_some()||self.retirement.is_some(){return None}let mut result=self.pending.take()?;result.retained_receipt=receipt;Some(result)}
}
impl Drop for OriginalShardEvent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Event reached Drop before source and pending result returned")}}
