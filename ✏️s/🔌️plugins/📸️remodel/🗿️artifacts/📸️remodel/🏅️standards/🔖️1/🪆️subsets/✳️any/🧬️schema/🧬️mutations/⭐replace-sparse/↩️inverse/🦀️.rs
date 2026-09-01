//! ↩️ Inverse for `ReplaceSparse` — the OLD `ReconstructionResults.sparse` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceSparse, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::replace_sparse(base.results.sparse.clone())]
}
//#endregion 🔖️Inverse
