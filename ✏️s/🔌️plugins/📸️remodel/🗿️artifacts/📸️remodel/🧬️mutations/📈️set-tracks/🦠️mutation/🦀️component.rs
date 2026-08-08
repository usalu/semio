//! 📈️ Remodel mutation — `SetTracks` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, tracks: &Vec<crate::artifacts::remodel::MotionTrackSummary>) {
    next.results.tracks = tracks.clone();
}
//#endregion 🔖️Mutation
