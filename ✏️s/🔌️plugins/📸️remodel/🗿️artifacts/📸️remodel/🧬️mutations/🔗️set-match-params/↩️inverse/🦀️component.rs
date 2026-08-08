//! ↩️ Inverse for `SetMatchParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMatchParams { params: base.params.matching.clone() }]
}
//#endregion 🔖️Inverse
