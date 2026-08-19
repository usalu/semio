//! ↩️ Inverse for `ReplaceDense` — the OLD `ReconstructionResults.dense` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceDense, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_dense(base.results.dense.clone())]
}
//#endregion 🔖️Inverse
