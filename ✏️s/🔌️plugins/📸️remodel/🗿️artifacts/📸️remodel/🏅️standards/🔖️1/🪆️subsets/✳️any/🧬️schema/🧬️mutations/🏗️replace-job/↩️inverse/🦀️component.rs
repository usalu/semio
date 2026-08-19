//! ↩️ Inverse for `ReplaceJob` — the OLD `ReconstructionJob` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceJob, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_job(base.job.clone())]
}
//#endregion 🔖️Inverse
