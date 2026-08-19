//! ↩️ Inverse for `ReplaceSparse` — the OLD `ReconstructionResults.sparse` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceSparse, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_sparse(base.results.sparse.clone())]
}
//#endregion 🔖️Inverse
