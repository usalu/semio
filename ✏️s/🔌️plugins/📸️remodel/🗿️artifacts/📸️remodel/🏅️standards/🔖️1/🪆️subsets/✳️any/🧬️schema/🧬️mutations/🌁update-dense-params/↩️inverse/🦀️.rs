//! ↩️ Inverse for `UpdateDenseParams` — the OLD `DenseParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateDenseParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_dense_params(base.params.dense.clone())]
}
//#endregion 🔖️Inverse
