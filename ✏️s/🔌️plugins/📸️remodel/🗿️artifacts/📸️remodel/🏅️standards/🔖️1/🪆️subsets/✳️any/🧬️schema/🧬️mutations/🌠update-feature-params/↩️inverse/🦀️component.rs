//! ↩️ Inverse for `UpdateFeatureParams` — the OLD `FeatureParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::UpdateFeatureParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::update_feature_params(base.params.feature.clone())]
}
//#endregion 🔖️Inverse
