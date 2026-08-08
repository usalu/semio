//! 🏭️ Remodel mutation — `SetJob` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, job: &crate::artifacts::remodel::ReconstructionJob) {
    next.job = job.clone();
}
//#endregion 🔖️Mutation
