//! ↩️ Inverse for `ReplaceQc` — the OLD `ReconstructionResults.qc` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReplaceQc, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_qc(base.results.qc.clone())]
}
//#endregion 🔖️Inverse
