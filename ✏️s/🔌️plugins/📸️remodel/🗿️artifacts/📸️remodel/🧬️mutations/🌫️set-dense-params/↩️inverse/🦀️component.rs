//! ↩️ Inverse for `SetDenseParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetDenseParams { params: base.params.dense.clone() }]
}
//#endregion 🔖️Inverse
