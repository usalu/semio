//! ↩️ Inverse for `UpdateGeoParams` — the OLD `GeoParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_geo_params::component::UpdateGeoParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateGeoParams>>(_payload: &crate::standards::v1::subsets::any::schema::mutations::update_geo_params::component::UpdateGeoParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_geo_params(base.params.geo.clone())]

    })())
}
//#endregion 🔖️Inverse
