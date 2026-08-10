//! ↩️ Inverse for `SetFeatureParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetFeatureParams { params: base.params.feature.clone() }]
}
//#endregion 🔖️Inverse
