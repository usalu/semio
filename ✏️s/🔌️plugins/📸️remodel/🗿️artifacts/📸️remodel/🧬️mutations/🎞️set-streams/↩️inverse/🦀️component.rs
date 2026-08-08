//! ↩️ Inverse for `SetStreams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetStreams { streams: base.streams.clone() }]
}
//#endregion 🔖️Inverse
