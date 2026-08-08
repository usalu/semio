//! ↩️ Inverse for `SetGeoProducts`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetGeoProducts { geo: base.results.geo.clone() }]
}
//#endregion 🔖️Inverse
