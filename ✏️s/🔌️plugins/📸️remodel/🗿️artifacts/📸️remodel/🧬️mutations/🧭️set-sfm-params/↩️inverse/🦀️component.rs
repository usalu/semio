//! ↩️ Inverse for `SetSfmParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetSfmParams { params: base.params.sfm.clone() }]
}
//#endregion 🔖️Inverse
