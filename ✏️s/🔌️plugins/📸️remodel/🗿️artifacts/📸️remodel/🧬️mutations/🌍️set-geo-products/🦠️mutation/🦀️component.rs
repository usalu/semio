//! 🌍️ Remodel mutation — `SetGeoProducts` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, geo: &Option<crate::artifacts::remodel::GeoProducts>) {
    next.results.geo = geo.clone();
}
//#endregion 🔖️Mutation
