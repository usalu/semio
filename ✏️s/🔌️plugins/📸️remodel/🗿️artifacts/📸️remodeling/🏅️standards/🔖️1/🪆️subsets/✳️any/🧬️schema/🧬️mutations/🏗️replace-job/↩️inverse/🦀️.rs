//! ↩️ Inverse for `ReplaceJob` — the OLD `ReconstructionJob` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceJob, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_job(base.job.clone())]
}
//#endregion 🔖️Inverse
