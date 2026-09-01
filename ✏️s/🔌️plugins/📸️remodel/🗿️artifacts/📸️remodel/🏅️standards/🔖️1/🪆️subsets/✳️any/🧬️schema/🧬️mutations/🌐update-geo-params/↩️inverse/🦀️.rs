//! ↩️ Inverse for `UpdateGeoParams` — the OLD `GeoParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateGeoParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_geo_params(base.params.geo.clone())]
}
//#endregion 🔖️Inverse
