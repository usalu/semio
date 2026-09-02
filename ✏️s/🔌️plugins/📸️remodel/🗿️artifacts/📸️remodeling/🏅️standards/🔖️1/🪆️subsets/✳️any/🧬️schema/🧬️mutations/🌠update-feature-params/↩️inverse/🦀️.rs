//! ↩️ Inverse for `UpdateFeatureParams` — the OLD `FeatureParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateFeatureParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_feature_params(base.params.feature.clone())]
}
//#endregion 🔖️Inverse
