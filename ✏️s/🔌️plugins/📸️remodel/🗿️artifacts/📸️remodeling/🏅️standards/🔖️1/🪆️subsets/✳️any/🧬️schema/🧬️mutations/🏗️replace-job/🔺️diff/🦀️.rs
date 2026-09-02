//! 🔺️ Sparse diff builder for `ReplaceJob` — a whole-value swap of `job`, which is always present
//! on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceJob, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.job == base.job {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Reconstruction job already has this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { job: Some(payload.job.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
