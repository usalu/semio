//! 🛤️ Remodel mutation — `SetTrajectory` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, trajectory: &Option<crate::artifacts::remodel::CameraTrajectory>) {
    next.results.trajectory = trajectory.clone();
}
//#endregion 🔖️Mutation
