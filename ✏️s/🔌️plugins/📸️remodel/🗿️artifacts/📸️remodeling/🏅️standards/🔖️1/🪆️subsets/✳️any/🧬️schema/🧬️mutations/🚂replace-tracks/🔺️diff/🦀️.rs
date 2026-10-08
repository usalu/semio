//! 🔺️ Sparse diff builder for `ReplaceTracks` — a whole-value swap of `results.tracks`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::diff::{RemodelingDiff, RemodelingResultsDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceTracks, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.tracks == base.results.tracks {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Tracks already have this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { tracks: Some(payload.tracks.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
