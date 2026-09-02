//! 🔺️ Sparse diff builder for `ReplaceTrajectory`. Clearing an already-absent trajectory ⇒ Error;
//! identical resubmission ⇒ Warning.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceTrajectory, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.trajectory.is_none() && base.results.trajectory.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no trajectory to clear.".to_string(), [base.id.clone()]);
    }
    if payload.trajectory == base.results.trajectory {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Trajectory is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.trajectory = payload.trajectory.clone();
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
