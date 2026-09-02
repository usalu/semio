//! ↩️ Inverse for `ReplaceGeoProducts` — the OLD `ReconstructionResults.geo` from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceGeoProducts, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_geo_products(base.results.geo.clone())]
}
//#endregion 🔖️Inverse
