//! ↩️ Inverse for `ReplaceTracks` — the OLD `ReconstructionResults.tracks` from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceTracks, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_tracks(base.results.tracks.clone())]
}
//#endregion 🔖️Inverse
