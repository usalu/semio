//! ↩️ Inverse for `SetDense`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetDense { dense: base.results.dense.clone() }]
}
//#endregion 🔖️Inverse
