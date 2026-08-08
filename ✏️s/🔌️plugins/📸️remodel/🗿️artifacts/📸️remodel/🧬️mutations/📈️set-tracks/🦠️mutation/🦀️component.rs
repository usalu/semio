//! 📈️ Remodel mutation — `SetTracks` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, tracks: &Vec<crate::artifacts::remodel::MotionTrackSummary>) {
    next.results.tracks = tracks.clone();
}
//#endregion 🔖️Mutation
