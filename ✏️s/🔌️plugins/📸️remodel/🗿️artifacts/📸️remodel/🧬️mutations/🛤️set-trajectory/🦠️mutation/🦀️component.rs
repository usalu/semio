//! 🛤️ Remodel mutation — `SetTrajectory` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, trajectory: &Option<crate::artifacts::remodel::CameraTrajectory>) {
    next.results.trajectory = trajectory.clone();
}
//#endregion 🔖️Mutation
