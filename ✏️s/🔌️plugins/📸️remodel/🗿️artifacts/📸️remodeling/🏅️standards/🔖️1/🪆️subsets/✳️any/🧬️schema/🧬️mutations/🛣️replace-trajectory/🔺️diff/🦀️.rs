//! 🔺️ Sparse diff builder for `ReplaceTrajectory`. Clearing an already-absent trajectory ⇒ Error;
//! identical resubmission ⇒ Warning.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssigned;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingResultsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::replace_trajectory::component::ReplaceTrajectory;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::replace_trajectory::component::ReplaceTrajectory, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.trajectory.is_none() && base.results.trajectory.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no trajectory to clear.".to_string(), [base.id.clone()]);
    }
    if payload.trajectory == base.results.trajectory {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Trajectory is already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { trajectory: Some(RemodelingAssigned::new(payload.trajectory.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
