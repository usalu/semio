//! 🌍️ Remodel mutation — `SetGeoProducts` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, geo: &Option<crate::artifacts::remodel::GeoProducts>) {
    next.results.geo = geo.clone();
}
//#endregion 🔖️Mutation
