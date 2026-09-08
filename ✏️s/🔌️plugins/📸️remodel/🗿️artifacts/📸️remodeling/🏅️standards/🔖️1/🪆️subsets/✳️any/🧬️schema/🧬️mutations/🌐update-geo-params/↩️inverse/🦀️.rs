//! ↩️ Inverse for `UpdateGeoParams` — the OLD `GeoParams` looked up from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateGeoParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_geo_params(base.params.geo.clone())]
}
//#endregion 🔖️Inverse
