//! 🧹️ Shared counters admit work before private geometry ownership changes.
use semio_framework_value::retained_clone::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,ValueError};

/// 🪙️ The host's funding seam: every turn of a long operation runs inside one original effect the host opens and charges to its own turn.
pub trait EffectTurns {
    /// 🎟️ Runs one operation inside one actual original effect granted at most `grant`.
    fn turn<R>(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>;
}

struct Ceiling{maximum:u64}
impl OriginalWorkRecipient for Ceiling{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
struct Invoked<F,R>{operation:Option<F>,output:Option<R>}
impl<F:FnOnce(&mut RetainedAllocationEffect<'_>)->R,R> OriginalEffectProducer for Invoked<F,R>{
 type Output=RetainedCloneStep;type Failure=ValueError;type Context<'a,'b>=() where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&())->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,_:&mut (),effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where 'b:'a{self.output=Some((self.operation.take().ok_or_else(||ValueError::invariant("blocking turn has no operation"))?)(effect));Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
 fn recover<'r,'a,'b>(&mut self,_:&mut (),_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where 'b:'a{Err(ValueError::invariant("blocking turn has no original callback to recover"))}
}

/// 🚪️ The blocking host edge: opens each turn as its own original Work action carrying a genuine producer invocation, capped at `maximum_work` units.
#[derive(Clone,Copy,Debug)]
pub struct BlockingTurns{maximum_work:u64}
impl BlockingTurns{
 pub const fn new(maximum_work:u64)->Self{Self{maximum_work}}
}
impl EffectTurns for BlockingTurns {
 fn turn<R>(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>{
  let maximum=self.maximum_work;let mut authority=OriginalWorkAuthority::new(Ceiling{maximum});let mut actual=None;
  let output=authority.with_loan(&mut (),|loan|{
   let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;
   if work.prepare_for_action(maximum,&mut action)?!=Some(true){return Err(ValueError::invariant("blocking turn work quote was not admitted"))}
   let mut receipt=None;work.issue_performed_for_action(action,&mut receipt)?;
   let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,&mut actual,receipt.as_mut().ok_or_else(||ValueError::invariant("blocking turn receipt absent"))?);
    let mut producer=OriginalEffectAuthority::new(Invoked{operation:Some(operation),output:None});let mut invoked=None;
    if !producer.drive(&mut (),&mut effect,&mut invoked)?{return Err(ValueError::invariant("blocking turn stayed pending"))}
    let mut answer=None;invoked.as_mut().ok_or_else(||ValueError::invariant("blocking turn action absent"))?.transfer_reply(&mut answer)?;producer.finish(&mut invoked)?;
    answer.ok_or_else(||ValueError::invariant("blocking turn gave no reply"))??;
    producer.into_producer().ok().and_then(|mut done|done.output.take()).ok_or_else(||ValueError::invariant("blocking turn did not run"))?};
   let performed=OriginalWorkSink::observed_work(receipt.as_ref().ok_or_else(||ValueError::invariant("blocking turn receipt absent"))?);
   work.claim_performed_for_action(action,&mut receipt)?;
   if performed==0{work.cancel_for_action(&mut action)?}else{let mut partial=None;if !work.settle_partial_for_action_into(action,&mut partial)?{return Err(ValueError::invariant("blocking turn partial work was refused"))}let partial=partial.take().ok_or_else(||ValueError::invariant("blocking turn partial absent"))?;if partial.total()!=performed{return Err(ValueError::invariant("blocking turn partial differs from performed"))}if !work.commit_for_action(performed,action,&mut proof)?{return Err(ValueError::invariant("blocking turn commit was refused"))}work.finish_for_action(&mut proof,&mut action)?}
   Ok(output)
  })??;
  if authority.has_live_actions(){return Err(ValueError::invariant("blocking turn left a live work action"))}
  Ok(output)
 }
}

#[derive(Clone,Copy,Debug)]
pub struct WorkRetirementProgress{pub phase:&'static str,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Default)]
pub struct WorkRetirementCounter{work:u64,done:bool}
impl WorkRetirementCounter{
 pub fn advance(&mut self,grant:usize,mut step:impl FnMut()->bool)->Result<WorkRetirementProgress,&'static str>{
  if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid retirement work grant");}
  for _ in 0..grant{if self.done{break;}self.done=step();self.work+=1;}Ok(WorkRetirementProgress{phase:if self.done{"complete"}else{"closing"},work:self.work,done:self.done})
 }
}

/// 🎮️ Retains typed ownership inline and closes only under the caller's explicit physical grant.
#[macro_export]
macro_rules! physical_work_retirement {
 ($retired:ident,$job:ty) => {
  pub struct $retired {#[doc(hidden)] pub owner:semio_framework_value::retirement::controlled::ControlledRetirement<$job>}
  impl $retired {
   pub fn new(job:$job)->Self {Self{owner:semio_framework_value::retirement::controlled::ControlledRetirement::retain(job)}}
   pub fn terminal_is_empty(&self)->bool {self.owner.terminal_is_empty()}
   pub fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.owner.step(body,effect)}
   pub fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.owner.next_demand(body)}
   pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand()}
   pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_capacity_byte_demand(copy)}
   pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_demand()}
   pub fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_depth_demand()}
  }
  semio_framework_value::artifact_retire_struct!($retired{owner});
 };
}
