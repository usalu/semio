//! ↩️ Inverse for `UpdateFeatureParams` — the OLD `FeatureParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_feature_params::component::UpdateFeatureParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateFeatureParams>>(_payload: &crate::standards::v1::subsets::any::schema::mutations::update_feature_params::component::UpdateFeatureParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_feature_params(base.params.feature.clone())]

    })())
}
//#endregion 🔖️Inverse
