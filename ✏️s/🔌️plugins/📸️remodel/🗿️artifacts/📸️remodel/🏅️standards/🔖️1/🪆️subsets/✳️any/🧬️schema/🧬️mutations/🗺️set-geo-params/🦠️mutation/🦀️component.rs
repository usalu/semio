//! 🗺️ Remodel mutation — `SetGeoParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::GeoParams) {
    next.params.geo = params.clone();
}
//#endregion 🔖️Mutation
