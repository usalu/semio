//! 🗺️ Remodel mutation — `SetGeoParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::GeoParams) {
    next.params.geo = params.clone();
}
//#endregion 🔖️Mutation
