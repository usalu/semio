//! ↩️ Inverse for `SetTrajectory`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetTrajectory { trajectory: base.results.trajectory.clone() }]
}
//#endregion 🔖️Inverse
