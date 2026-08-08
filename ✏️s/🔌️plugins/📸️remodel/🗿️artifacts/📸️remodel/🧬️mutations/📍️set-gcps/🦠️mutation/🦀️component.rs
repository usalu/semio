//! 📍️ Remodel mutation — `SetGcps` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, gcps: &Vec<crate::artifacts::remodel::GroundControlPoint>) {
    next.gcps = gcps.clone();
}
//#endregion 🔖️Mutation
