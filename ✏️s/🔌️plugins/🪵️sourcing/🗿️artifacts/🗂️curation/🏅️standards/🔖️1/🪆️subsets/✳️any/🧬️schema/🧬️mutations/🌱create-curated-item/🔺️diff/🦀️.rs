//! 🔺 Sparse diff builder for `CreateCuratedItem` — a real insert at `index` or append (never a whole-
//! snapshot capture). Fatal `duplicate-id` when the object is already curated.
use crate::diff::{CurationCuratedDelta, CurationDiff};
use crate::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCuratedItem, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if base.curated.iter().any(|item| item.object_id == payload.item.object_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("\"{}\" is already curated.", payload.item.object_id), [payload.item.object_id.clone()]);
    }
    let at = payload.index.map_or(base.curated.len(), |index| (index as usize).min(base.curated.len()));
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta::insertion(at, payload.item.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
