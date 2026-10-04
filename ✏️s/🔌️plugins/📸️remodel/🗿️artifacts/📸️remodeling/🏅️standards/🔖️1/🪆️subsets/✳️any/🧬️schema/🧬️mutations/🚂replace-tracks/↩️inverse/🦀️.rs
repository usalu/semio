//! ↩️ Inverse for `ReplaceTracks` — the OLD `ReconstructionResults.tracks` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceTracks, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_tracks(base.results.tracks.clone())]

    })())
}
//#endregion 🔖️Inverse
