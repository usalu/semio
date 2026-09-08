//! ↩️ Inverse for `ReplaceDense` — the OLD `ReconstructionResults.dense` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceDense, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_dense(base.results.dense.clone())]
}
//#endregion 🔖️Inverse
