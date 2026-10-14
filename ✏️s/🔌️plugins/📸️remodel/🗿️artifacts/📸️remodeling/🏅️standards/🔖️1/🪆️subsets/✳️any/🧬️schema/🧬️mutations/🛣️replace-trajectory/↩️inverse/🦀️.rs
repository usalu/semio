//! ↩️ Inverse for `ReplaceTrajectory` — the OLD `ReconstructionResults.trajectory` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_trajectory::ReplaceTrajectory;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceTrajectory>>(_payload: &super::ReplaceTrajectory, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_trajectory(base.results.trajectory.clone())]

    })())
}
//#endregion 🔖️Inverse
