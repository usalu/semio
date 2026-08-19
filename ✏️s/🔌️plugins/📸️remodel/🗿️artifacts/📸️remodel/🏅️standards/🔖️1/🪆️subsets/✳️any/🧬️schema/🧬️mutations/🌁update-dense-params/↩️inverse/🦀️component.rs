//! ↩️ Inverse for `UpdateDenseParams` — the OLD `DenseParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::UpdateDenseParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::update_dense_params(base.params.dense.clone())]
}
//#endregion 🔖️Inverse
