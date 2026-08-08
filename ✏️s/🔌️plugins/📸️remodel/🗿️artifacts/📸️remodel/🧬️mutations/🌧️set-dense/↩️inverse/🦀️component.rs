//! ↩️ Inverse for `SetDense`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetDense { dense: base.results.dense.clone() }]
}
//#endregion 🔖️Inverse
