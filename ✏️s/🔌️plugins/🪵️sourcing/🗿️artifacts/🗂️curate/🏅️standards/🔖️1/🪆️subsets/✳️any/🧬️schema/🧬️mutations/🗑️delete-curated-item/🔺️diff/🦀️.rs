//! 🔺 Sparse diff builder for `DeleteCuratedItem` — a real removal (never a whole-snapshot
//! capture). Error `target-missing` when absent.
use crate::artifacts::curate::diff::{CurateCuratedDelta, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteCuratedItem, base: &CurateSnapshot) -> protocol::MutationOutcome<CurateDiff> {
    if !base.curated.iter().any(|item| item.object_id == payload.object_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("\"{}\" is not curated.", payload.object_id), [payload.object_id.clone()]);
    }
    protocol::MutationOutcome::new(CurateDiff { curated: Some(CurateCuratedDelta { removed: vec![payload.object_id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
