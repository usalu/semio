//! ↩️ Inverse for `ReplaceTrajectory` — the OLD `ReconstructionResults.trajectory` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceTrajectory, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_trajectory(base.results.trajectory.clone())]
}
//#endregion 🔖️Inverse
