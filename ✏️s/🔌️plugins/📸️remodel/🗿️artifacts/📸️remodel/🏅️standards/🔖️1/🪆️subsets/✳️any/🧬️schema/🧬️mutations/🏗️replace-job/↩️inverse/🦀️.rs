//! ↩️ Inverse for `ReplaceJob` — the OLD `ReconstructionJob` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceJob, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::replace_job(base.job.clone())]
}
//#endregion 🔖️Inverse
