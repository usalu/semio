//! 🏛️ The Source-resident clock custody: one actual owner, action and decision per original Source.
use super::*;
use semio_framework_value_control_contract::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_retirement::RetirementDemand;
use std::mem::{size_of,ManuallyDrop};
use semio_framework_value_control_contract::{RetainedOwnerGate,RetainedOwnerGateGuard};
type Turn=Option<OriginalSourceTurnIdentity>;
type Owner=Option<Box<dyn OriginalErasedHostClock>>;
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn other_dispatch()->ValueError{refused("original Source clock belongs to a different dispatch")}
fn absent()->ValueError{refused("original Source clock is absent")}
struct Inner{owner:ManuallyDrop<Owner>,action:ManuallyDrop<Option<OriginalClockAction>>,decision:ManuallyDrop<OriginalClockDecisionFrame>,reading:ManuallyDrop<Option<Result<OriginalClockObservation,ValueError>>>,refusal:ManuallyDrop<Option<ValueError>>,reading_transferred:bool,action_turn:Turn,decision_turn:Turn}
impl Inner{
 fn unsettled(&self)->bool{self.action.is_some()||!self.decision.terminal_is_empty()||self.reading.is_some()||self.refusal.is_some()||self.reading_transferred||self.action_turn.is_some()||self.decision_turn.is_some()}
 fn settled(&self)->Result<(),ValueError>{if self.unsettled(){Err(refused("original Source clock action requires recovery"))}else{Ok(())}}
}
impl Drop for Inner{fn drop(&mut self){assert!(std::thread::panicking()||(self.owner.is_none()&&!self.unsettled()),"original Source clock requires paid return")}}
/// 🕰️ Holds the exact Source clock until paid retirement returns it.
pub struct OriginalSourceClock(RetainedOwnerGate<Inner>);
impl OriginalSourceClock{
 /// 🌱️ Starts an empty custody without an admitted host.
 pub fn new()->Self{Self(RetainedOwnerGate::new(Inner{owner:ManuallyDrop::new(None),action:ManuallyDrop::new(None),decision:ManuallyDrop::new(OriginalClockDecisionFrame::new()),reading:ManuallyDrop::new(None),refusal:ManuallyDrop::new(None),reading_transferred:false,action_turn:None,decision_turn:None}))}
 fn lock(&self)->Result<RetainedOwnerGateGuard<'_,Inner>,ValueError>{self.0.try_lock().map_err(|_|refused("original Source clock is currently lent"))}
 /// 🧾️ Reports whether nothing remains to retire; a lent clock is not empty.
 pub fn terminal_is_empty(&self)->bool{self.lock().is_ok_and(|clock|clock.owner.is_none()&&!clock.unsettled())}
 /// 📏️ Quotes the physical demand of the next retirement step, if any.
 pub fn retirement_demand(&self)->Result<Option<RetirementDemand>,ValueError>{let clock=self.lock()?;clock.settled()?;let Some(owner)=clock.owner.as_ref()else{return Ok(None)};if owner.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Owner>(),release_bytes:owner.frame_release_bytes(),depth:1,..Default::default()}))}owner.next_demand(0).map(Some)}
 /// 🧮️ Quotes the Work demand of the next retirement step, if any.
 pub fn retirement_work(&self)->Result<Option<u64>,ValueError>{let clock=self.lock()?;clock.settled()?;let Some(owner)=clock.owner.as_ref()else{return Ok(None)};if owner.terminal_is_empty(){Ok(Some(1))}else{owner.next_work().map(Some)}}
 /// ♻️ Pays one retirement step of the owned clock.
 pub fn close(&self,effect:&mut RetainedAllocationEffect<'_>)->Option<RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>>{
  let fail=|error:ValueError,work|Some(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_primary(error)),work}));
  let mut clock=match self.lock(){Ok(clock)=>clock,Err(error)=>return fail(error,0)};
  if let Err(error)=clock.settled(){return fail(error,0)}
  let owner=clock.owner.as_mut()?;
  if !owner.terminal_is_empty(){return Some(owner.step(effect))}
  let released=owner.frame_release_bytes();
  let copied=size_of::<Owner>();
  let grant=effect.grant();
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copied||grant.maximum_release_bytes<released||effect.remaining_work_units().map_or(true,|work|work==0){return Some(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}))}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return fail(refused("original Source clock release exceeded its Work grant"),1),Err(error)=>return fail(error,0)}
  drop(clock.owner.take());
  Some(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,released_bytes:released,..Default::default()})),work:1}))
 }
 /// 🎟️ Admits the actual host with its original physical and Work effect.
 pub fn admit<P:OriginalHostClockProducer+semio_framework_value_retirement::retirement::RetireOwned+Send+'static>(&self,context:&OriginalSystemClockContext<'_>,turn:Turn,host:&mut Option<OriginalClockAuthority<OriginalHostClock<P>>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};
  let mut clock=match self.lock(){Ok(clock)=>clock,Err(error)=>return fail(error)};
  if effect.work_source_turn()!=turn{return fail(refused("original Source clock requires the same Work dispatch"))}
  let Some(authority)=host.as_ref()else{return fail(refused("original host clock owner is absent"))};
  if let Err(error)=authority.source_turn(context){return fail(error)}
  admit_original_host_clock(host,&mut clock.owner,effect)
 }
 /// 📨️ Drives or recovers the reading in its resident action.
 pub fn poll(&self,context:&mut OriginalSystemClockContext<'_>,effect:&mut RetainedAllocationEffect<'_>,turn:Turn)->Result<OriginalClockDecisionStatus,ValueError>{
  use OriginalClockDecisionStatus as Status;
  let mut clock=self.lock()?;
  let turn=turn.ok_or_else(||refused("original Source clock has no dispatch"))?;
  if clock.decision_turn.is_some(){return Err(refused("original Source clock decision is still active"))}
  if clock.action_turn.is_some_and(|original|original!=turn){return Err(other_dispatch())}
  if clock.refusal.is_some(){return Ok(Status::Refused)}
  if clock.reading_transferred{return Err(refused("original Source clock reading awaits finish"))}
  if clock.reading.is_some(){return Ok(Status::Ready)}
  if clock.owner.is_none(){return Err(absent())}
  clock.action_turn=Some(turn);
  let Inner{owner,action,reading,refusal,..}=&mut *clock;
  let authority=owner.as_mut().ok_or_else(absent)?;
  match authority.drive(context,effect,action){
   Err(error)=>{**refusal=Some(error);Ok(Status::Refused)},
   Ok(false)=>{if let Some(action)=action.as_mut(){action.transfer_refusal(refusal)?;}Ok(if refusal.is_some(){Status::Refused}else{Status::Pending})},
   Ok(true)=>{let action=action.as_mut().ok_or_else(||refused("original clock action is absent"))?;if action.transfer_reading(reading)?{Ok(Status::Ready)}else{Ok(Status::Pending)}}
  }
 }
 /// 📤️ Transfers the exact reading into a caller-owned slot.
 pub fn take_reading(&self,turn:Turn,output:&mut Option<Result<OriginalClockObservation,ValueError>>)->Result<bool,ValueError>{
  if output.is_some(){return Err(refused("original clock reading destination is occupied"))}
  let mut clock=self.lock()?;
  if clock.action_turn!=turn{return Err(other_dispatch())}
  let Some(reading)=clock.reading.take()else{return Ok(false)};
  *output=Some(reading);clock.reading_transferred=true;Ok(true)
 }
 /// 🏁️ Finishes the exact action after its reading transfers.
 pub fn finish_reading(&self,context:&OriginalSystemClockContext<'_>,turn:Turn)->Result<(),ValueError>{
  let mut clock=self.lock()?;
  if clock.action_turn!=turn{return Err(other_dispatch())}
  if !clock.reading_transferred||clock.refusal.is_some(){return Err(refused("original clock reading is not ready to finish"))}
  let Inner{owner,action,..}=&mut *clock;
  owner.as_mut().ok_or_else(absent)?.finish(context,action)?;
  clock.reading_transferred=false;clock.action_turn=None;Ok(())
 }
 /// 📤️ Transfers the actual clock or context refusal without replacing it.
 pub fn take_refusal(&self,turn:Turn,output:&mut Option<ValueError>)->Result<bool,ValueError>{
  if output.is_some(){return Err(refused("original clock refusal destination is occupied"))}
  let mut clock=self.lock()?;
  if clock.action_turn.or(clock.decision_turn)!=turn{return Err(other_dispatch())}
  let Some(error)=clock.refusal.take()else{return Ok(false)};
  *output=Some(error);Ok(true)
 }
 /// ↩️ Clears only a same-dispatch entry whose action never started.
 pub fn cancel_unstarted(&self,turn:Turn)->Result<(),ValueError>{
  let mut clock=self.lock()?;
  if clock.action_turn!=turn||clock.action.is_some()||clock.reading.is_some()||clock.refusal.is_some()||clock.reading_transferred{return Err(refused("original Source clock entry cannot be cancelled"))}
  clock.action_turn=None;Ok(())
 }
 /// 🎯️ Derives a deadline from the same clock's original reading.
 pub fn deadline(&self,context:&OriginalSystemClockContext<'_>,observation:OriginalClockObservation,duration:u64)->Result<OriginalClockDeadline,ValueError>{
  self.lock()?.owner.as_ref().ok_or_else(absent)?.deadline_from(context,observation,duration)
 }
 /// 🚦️ Drives the recorded deadline decision across Pending and unwind.
 pub fn poll_decision(&self,context:&mut OriginalSystemClockContext<'_>,effect:&mut RetainedAllocationEffect<'_>,turn:Turn,deadline:OriginalClockDeadline,fuel:u64)->Result<OriginalClockDecisionStatus,ValueError>{
  use OriginalClockDecisionStatus as Status;
  let mut clock=self.lock()?;
  let turn=turn.ok_or_else(||refused("original Source clock has no dispatch"))?;
  if clock.action_turn.is_some()||clock.decision_turn.is_some_and(|source|source!=turn){return Err(refused("original Source clock belongs to another action"))}
  if clock.refusal.is_some(){return Ok(Status::Refused)}
  if clock.owner.is_none(){return Err(absent())}
  clock.decision_turn=Some(turn);
  let Inner{owner,decision,refusal,..}=&mut *clock;
  let authority=owner.as_mut().ok_or_else(absent)?;
  match authority.poll_decision(context,effect,decision,deadline,fuel){Ok(Status::Refused)=>{decision.transfer_refusal(refusal)?;Ok(Status::Refused)},Ok(status)=>Ok(status),Err(error)=>{**refusal=Some(error);Ok(Status::Refused)}}
 }
 /// 📤️ Transfers the original decision directly to the caller-owned slot.
 pub fn take_decision(&self,turn:Turn,output:&mut Option<bool>)->Result<bool,ValueError>{
  let mut clock=self.lock()?;
  if clock.decision_turn!=turn{return Err(other_dispatch())}
  clock.decision.transfer_decision(output)
 }
 /// 🏁️ Finishes the exact decision after transfer.
 pub fn finish_decision(&self,context:&OriginalSystemClockContext<'_>,turn:Turn)->Result<(),ValueError>{
  let mut clock=self.lock()?;
  if clock.decision_turn!=turn||clock.refusal.is_some(){return Err(refused("original Source decision is not ready to finish"))}
  let Inner{owner,decision,..}=&mut *clock;
  owner.as_mut().ok_or_else(absent)?.finish_decision(context,decision)?;
  clock.decision_turn=None;Ok(())
 }
 /// ↩️ Clears only a decision whose producer never started.
 pub fn cancel_unstarted_decision(&self,turn:Turn)->Result<(),ValueError>{
  let mut clock=self.lock()?;
  if clock.decision_turn!=turn||clock.refusal.is_some(){return Err(refused("original Source decision cannot be cancelled"))}
  clock.decision.cancel_unstarted()?;clock.decision_turn=None;Ok(())
 }
}
