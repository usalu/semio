//! 🎡️ Drives one poll-shaped retirement call through the real original Work authority; the physical heap receipt equals the recorded progress.
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep};
use semio_framework_value_clone_source_testing::drive;
pub(crate) fn turn(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(RetainedCloneStep,usize,usize){
 drive::warm();
 match drive::drive(grant,1<<16,call).result{Ok(step)=>{let progress=step.progress();(step,progress.retained_capacity_bytes,progress.released_bytes)}Err(error)=>panic!("close turn refused {error:?}")}
}
