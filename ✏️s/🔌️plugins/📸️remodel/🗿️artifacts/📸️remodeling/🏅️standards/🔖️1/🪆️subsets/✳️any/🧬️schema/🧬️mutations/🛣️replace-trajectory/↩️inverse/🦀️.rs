//! ↩️ Inverse for `ReplaceTrajectory` — the OLD `ReconstructionResults.trajectory` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceTrajectory, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_trajectory(base.results.trajectory.clone())]

    })())
}
//#endregion 🔖️Inverse
