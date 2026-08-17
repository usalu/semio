//! 🔺️ Sparse diff builder for `ReplaceTrajectory`. Clearing an already-absent trajectory ⇒ Error;
//! identical resubmission ⇒ Warning.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceTrajectory, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.trajectory.is_none() && base.results.trajectory.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no trajectory to clear.".to_string(), [base.id.clone()]);
    }
    if payload.trajectory == base.results.trajectory {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Trajectory is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.trajectory = payload.trajectory.clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
