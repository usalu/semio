//! ↩️ Inverse for `ReplaceGeoProducts` — the OLD `ReconstructionResults.geo` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_geo_products::ReplaceGeoProducts;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceGeoProducts>>(_payload: &super::ReplaceGeoProducts, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_geo_products(base.results.geo.clone())]

    })())
}
//#endregion 🔖️Inverse
