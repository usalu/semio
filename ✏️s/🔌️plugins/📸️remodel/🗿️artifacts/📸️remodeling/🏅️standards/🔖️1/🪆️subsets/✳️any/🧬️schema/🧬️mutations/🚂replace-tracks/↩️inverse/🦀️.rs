//! ↩️ Inverse for `ReplaceTracks` — the OLD `ReconstructionResults.tracks` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_tracks::ReplaceTracks;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceTracks>>(_payload: &super::ReplaceTracks, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_tracks(base.results.tracks.clone())]

    })())
}
//#endregion 🔖️Inverse
