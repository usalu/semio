//! ↩️ Inverse for `UpdateMatchParams` — the OLD `MatchParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_match_params::UpdateMatchParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateMatchParams>>(_payload: &super::UpdateMatchParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_match_params(base.params.matching.clone())]

    })())
}
//#endregion 🔖️Inverse
