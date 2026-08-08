//! ↩️ Inverse for `SetTrajectory`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetTrajectory { trajectory: base.results.trajectory.clone() }]
}
//#endregion 🔖️Inverse
