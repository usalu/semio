//! ↩️ Inverse for `SetStreams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetStreams { streams: base.streams.clone() }]
}
//#endregion 🔖️Inverse
