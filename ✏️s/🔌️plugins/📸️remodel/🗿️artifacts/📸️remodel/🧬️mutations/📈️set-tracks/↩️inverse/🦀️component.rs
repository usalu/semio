//! ↩️ Inverse for `SetTracks`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetTracks { tracks: base.results.tracks.clone() }]
}
//#endregion 🔖️Inverse
