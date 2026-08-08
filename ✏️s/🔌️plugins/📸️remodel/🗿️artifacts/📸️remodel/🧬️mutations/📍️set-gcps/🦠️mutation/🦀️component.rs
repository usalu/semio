//! 📍️ Remodel mutation — `SetGcps` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, gcps: &Vec<crate::artifacts::remodel::GroundControlPoint>) {
    next.gcps = gcps.clone();
}
//#endregion 🔖️Mutation
