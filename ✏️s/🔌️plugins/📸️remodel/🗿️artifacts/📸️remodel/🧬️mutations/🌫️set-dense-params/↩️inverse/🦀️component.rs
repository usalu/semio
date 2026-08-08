//! ↩️ Inverse for `SetDenseParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetDenseParams { params: base.params.dense.clone() }]
}
//#endregion 🔖️Inverse
