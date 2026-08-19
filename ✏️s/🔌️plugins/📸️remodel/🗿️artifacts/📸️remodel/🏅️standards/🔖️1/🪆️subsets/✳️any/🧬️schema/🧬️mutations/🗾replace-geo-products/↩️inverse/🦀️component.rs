//! ↩️ Inverse for `ReplaceGeoProducts` — the OLD `ReconstructionResults.geo` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceGeoProducts, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_geo_products(base.results.geo.clone())]
}
//#endregion 🔖️Inverse
