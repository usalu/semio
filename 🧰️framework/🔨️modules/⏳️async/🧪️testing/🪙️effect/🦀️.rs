//! 🪙️ Runs synchronous effect-shaped cancellation closers under one real original action.
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
const WORK:u64=1<<20;
const STEP_WORK:u64=64;
pub struct Turn(EffectBank);
impl Turn{
 pub fn new()->Self{Self(EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<12},WORK)))}
 pub fn run<T,E>(&mut self,grant:RetainedCloneGrant,call:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{let outcome=self.0.run_outcome(grant,STEP_WORK,call);assert_ne!(self.0.take_admitted(),Some(false),"original Work admits every settled turn");outcome}
}
/// 🪙️ Closes one cursor turn under a real original action and returns the receipted heap the fixture verified against its progress.
pub fn close_step(grant:RetainedCloneGrant,cursor:&mut dyn RetirementCursor)->(RetirementStep,(usize,usize)){
 let progress=|reply:&Result<RetirementStep,OriginalRetirementFailure>|match reply{Ok(RetirementStep::Progress(progress,_)|RetirementStep::ChildProgress(_,progress,_))=>*progress,Err(failure)=>failure.retained_progress(),_=>RetainedCloneProgress::default()};
 let turn=semio_framework_value_clone_source_testing::drive::drive_reply::<RetirementStep,OriginalRetirementFailure>(grant,STEP_WORK,progress,|effect,reply,recover|if recover{cursor.recover_close_into(0,effect,reply)}else{cursor.close_into(0,effect,reply)});
 let step=match turn.result{Ok(step)=>step,Err(failure)=>RetirementStep::Failure(failure,turn.work)};
 let receipt=match &step{RetirementStep::Progress(progress,_)|RetirementStep::ChildProgress(_,progress,_)=>*progress,RetirementStep::Failure(failure,_)=>failure.retained_progress(),_=>RetainedCloneProgress::default()};
 (step,(receipt.retained_capacity_bytes,receipt.released_bytes))
}
