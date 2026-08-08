//! 🏭️ Remodel mutation — `SetJob` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, job: &crate::artifacts::remodel::ReconstructionJob) {
    next.job = job.clone();
}
//#endregion 🔖️Mutation
