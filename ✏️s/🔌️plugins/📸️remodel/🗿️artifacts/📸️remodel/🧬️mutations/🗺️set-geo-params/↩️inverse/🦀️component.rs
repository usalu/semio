//! ↩️ Inverse for `SetGeoParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetGeoParams { params: base.params.geo.clone() }]
}
//#endregion 🔖️Inverse
