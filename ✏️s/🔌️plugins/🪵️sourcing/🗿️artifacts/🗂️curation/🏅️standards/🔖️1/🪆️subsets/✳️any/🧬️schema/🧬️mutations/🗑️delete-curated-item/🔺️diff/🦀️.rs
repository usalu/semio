//! 🔺 Sparse diff builder for `DeleteCuratedItem` — a real removal (never a whole-snapshot
//! capture). Error `target-missing` when absent.
use crate::artifacts::curation::diff::{CurationCuratedDelta, CurationDiff};
use crate::artifacts::curation::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteCuratedItem, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if !base.curated.iter().any(|item| item.object_id == payload.object_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("\"{}\" is not curated.", payload.object_id), [payload.object_id.clone()]);
    }
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta { removed: vec![payload.object_id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
