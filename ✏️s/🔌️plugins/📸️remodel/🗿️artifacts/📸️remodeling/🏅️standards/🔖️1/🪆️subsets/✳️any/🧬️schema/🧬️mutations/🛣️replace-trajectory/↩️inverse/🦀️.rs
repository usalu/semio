//! ↩️ Inverse for `ReplaceTrajectory` — the OLD `ReconstructionResults.trajectory` from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceTrajectory, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_trajectory(base.results.trajectory.clone())]
}
//#endregion 🔖️Inverse
