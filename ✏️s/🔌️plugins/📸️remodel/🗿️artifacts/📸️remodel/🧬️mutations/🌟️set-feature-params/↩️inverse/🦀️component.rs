//! ↩️ Inverse for `SetFeatureParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetFeatureParams { params: base.params.feature.clone() }]
}
//#endregion 🔖️Inverse
