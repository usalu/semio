//! 🔺️ Sparse diff builder for `ReplaceJob` — a whole-value swap of `job`, which is always present
//! on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceJob, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.job == base.job {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Reconstruction job already has this value.");
    }
    protocol::MutationOutcome::new(RemodelDiff { job: Some(payload.job.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
