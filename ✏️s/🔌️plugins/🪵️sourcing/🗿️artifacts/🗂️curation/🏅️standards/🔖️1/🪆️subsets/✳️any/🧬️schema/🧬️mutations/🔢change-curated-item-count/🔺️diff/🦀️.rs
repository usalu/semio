//! 🔺 Sparse diff builder for `ChangeCuratedItemCount` — a real single-field patch (never a whole-
//! snapshot capture). Error `target-missing` when absent, Warning `no-op` when the count is
//! unchanged.
use crate::artifacts::curation::diff::{CurationCuratedDelta, CurationCuratedPatchEntry, CurationDiff};
use crate::artifacts::curation::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeCuratedItemCount, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    let Some(existing) = base.curated.iter().find(|item| item.object_id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("\"{}\" is not curated.", payload.object_id), [payload.object_id.clone()]);
    };
    if existing.count == payload.new_count {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("\"{}\" count is already {}.", payload.object_id, payload.new_count));
    }
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta { patched: vec![CurationCuratedPatchEntry { object_id: payload.object_id.clone(), count: Some(payload.new_count) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
