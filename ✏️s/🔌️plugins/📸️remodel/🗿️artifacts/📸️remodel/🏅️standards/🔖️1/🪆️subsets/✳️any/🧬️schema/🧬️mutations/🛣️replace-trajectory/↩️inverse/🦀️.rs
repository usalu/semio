//! ↩️ Inverse for `ReplaceTrajectory` — the OLD `ReconstructionResults.trajectory` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceTrajectory, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::replace_trajectory(base.results.trajectory.clone())]
}
//#endregion 🔖️Inverse
