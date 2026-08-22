//! ↩️ Inverse for `ReplaceTracks` — the OLD `ReconstructionResults.tracks` from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReplaceTracks, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::replace_tracks(base.results.tracks.clone())]
}
//#endregion 🔖️Inverse
