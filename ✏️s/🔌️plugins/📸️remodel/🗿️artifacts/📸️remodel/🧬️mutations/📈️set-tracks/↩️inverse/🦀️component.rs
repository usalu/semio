//! ↩️ Inverse for `SetTracks`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetTracks { tracks: base.results.tracks.clone() }]
}
//#endregion 🔖️Inverse
