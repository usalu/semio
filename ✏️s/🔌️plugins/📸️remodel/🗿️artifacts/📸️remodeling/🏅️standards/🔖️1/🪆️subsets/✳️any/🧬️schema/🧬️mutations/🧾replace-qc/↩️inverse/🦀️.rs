//! ↩️ Inverse for `ReplaceQc` — the OLD `ReconstructionResults.qc` from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceQc, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_qc(base.results.qc.clone())]
}
//#endregion 🔖️Inverse
