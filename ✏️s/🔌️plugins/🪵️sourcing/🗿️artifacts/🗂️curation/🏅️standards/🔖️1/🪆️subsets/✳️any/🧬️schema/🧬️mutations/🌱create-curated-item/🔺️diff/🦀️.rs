//! 🔺 Sparse diff builder for `CreateCuratedItem` — a real append-only insert (never a whole-
//! snapshot capture). Fatal `duplicate-id` when the object is already curated.
use crate::artifacts::curation::diff::{CurationCuratedDelta, CurationDiff};
use crate::artifacts::curation::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCuratedItem, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if base.curated.iter().any(|item| item.object_id == payload.item.object_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("\"{}\" is already curated.", payload.item.object_id), [payload.item.object_id.clone()]);
    }
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta { added: vec![payload.item.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
