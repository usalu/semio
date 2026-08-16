//! 🔺 Sparse diff builder for `CreateCuratedItem` — a real append-only insert (never a whole-
//! snapshot capture). Fatal `duplicate-id` when the object is already curated.
use crate::artifacts::curate::diff::{CurateCuratedDelta, CurateDiff};
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateCuratedItem, base: &CurateSnapshot) -> protocol::MutationOutcome<CurateDiff> {
    if base.curated.iter().any(|item| item.object_id == payload.item.object_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("\"{}\" is already curated.", payload.item.object_id), [payload.item.object_id.clone()]);
    }
    protocol::MutationOutcome::new(CurateDiff { curated: Some(CurateCuratedDelta { added: vec![payload.item.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
