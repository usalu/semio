//! 🎞️ Remodel mutation — `SetStreams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, streams: &Vec<crate::artifacts::remodel::MediaStream>) {
    next.streams = streams.clone();
}
//#endregion 🔖️Mutation
