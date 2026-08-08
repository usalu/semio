//! ↩️ Inverse for `SetJob`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetJob { job: base.job.clone() }]
}
//#endregion 🔖️Inverse
