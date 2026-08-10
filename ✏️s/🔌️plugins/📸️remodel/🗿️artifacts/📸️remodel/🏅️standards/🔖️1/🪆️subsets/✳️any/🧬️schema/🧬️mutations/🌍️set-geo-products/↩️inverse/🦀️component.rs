//! ↩️ Inverse for `SetGeoProducts`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetGeoProducts { geo: base.results.geo.clone() }]
}
//#endregion 🔖️Inverse
