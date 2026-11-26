//! 🧪️ Test-only adapters run one effect-shaped ledger action under a real original action and assert its heap equals its receipt. See [`semio_framework_value_clone_source_testing::drive::drive`].
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_clone_source_testing::drive::drive;
pub const UNITS:u64=1<<20;
pub fn poll(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{drive(grant,UNITS,call)}
pub fn advance(grant:RetainedCloneGrant,mut call:impl FnMut(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalRetirementFailure>{
 let outcome=drive(grant,UNITS,|effect|{let result=call(effect).map(RetainedCloneStep::Progress).map_err(OriginalRetirementFailure::from);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})});
 RetainedWorkOutcome{result:outcome.result.map(|step|step.progress()),work:outcome.work}
}
pub fn close_owned<T:semio_framework_value::retirement::RetireOwned>(owner:&mut semio_framework_value::retained_clone::RetainedCloneClose,value:&mut Option<T>,body:usize,grant:RetainedCloneGrant)->RetainedCloneStep{
 poll(grant,|effect|{let started=owner.begin_granted(value,effect);match started.result{Ok(Some(step))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:started.work}),Ok(None)=>owner.step_granted(body,effect),Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:started.work})}}).result.unwrap()
}
