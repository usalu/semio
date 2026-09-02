//! ↩️ Inverse for `UpdateGeoParams` — the OLD `GeoParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateGeoParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_geo_params(base.params.geo.clone())]
}
//#endregion 🔖️Inverse
