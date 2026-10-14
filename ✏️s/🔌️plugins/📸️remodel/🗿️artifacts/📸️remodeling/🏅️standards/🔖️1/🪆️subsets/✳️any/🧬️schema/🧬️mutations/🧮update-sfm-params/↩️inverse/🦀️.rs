//! ↩️ Inverse for `UpdateSfmParams` — the OLD `SfmParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_sfm_params::UpdateSfmParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateSfmParams>>(_payload: &super::UpdateSfmParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_sfm_params(base.params.sfm.clone())]

    })())
}
//#endregion 🔖️Inverse
