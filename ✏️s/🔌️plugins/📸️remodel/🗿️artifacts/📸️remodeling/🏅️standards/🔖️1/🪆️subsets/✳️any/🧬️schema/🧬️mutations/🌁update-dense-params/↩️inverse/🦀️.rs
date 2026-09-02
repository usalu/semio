//! ↩️ Inverse for `UpdateDenseParams` — the OLD `DenseParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateDenseParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_dense_params(base.params.dense.clone())]
}
//#endregion 🔖️Inverse
