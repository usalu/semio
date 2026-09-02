//! 🔺️ Sparse diff builder for `ReplaceTracks` — a whole-value swap of `results.tracks`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceTracks, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.tracks == base.results.tracks {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Tracks already have this value.");
    }
    let mut results = base.results.clone();
    results.tracks = payload.tracks.clone();
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
