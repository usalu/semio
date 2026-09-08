//! ↩️ Inverse for `ReplaceSparse` — the OLD `ReconstructionResults.sparse` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceSparse, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_sparse(base.results.sparse.clone())]
}
//#endregion 🔖️Inverse
