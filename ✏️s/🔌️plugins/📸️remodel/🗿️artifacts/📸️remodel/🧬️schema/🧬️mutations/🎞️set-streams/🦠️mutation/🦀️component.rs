//! 🎞️ Remodel mutation — `SetStreams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, streams: &Vec<crate::artifacts::remodel::MediaStream>) {
    next.streams = streams.clone();
}
//#endregion 🔖️Mutation
