//! 🔺 Sparse diff builder for `CreateCuratedItem` — a real insert at `index` or append (never a whole-
//! snapshot capture). Fatal `duplicate-id` when the object is already curated.
use crate::diff::{CurationCuratedDelta, CurationDiff};
use crate::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCuratedItem, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if base.curated.iter().any(|item| item.object_id == payload.item.object_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("\"{}\" is already curated.", payload.item.object_id), [payload.item.object_id.clone()]);
    }
    protocol::MutationOutcome::new(CurationDiff { curated: Some(CurationCuratedDelta {
            added: vec![payload.item.clone()],
            reordered: payload.index.filter(|index| (*index as usize) < base.curated.len()).map(|index| {
                let mut order: Vec<String> = base.curated.iter().map(|item| item.object_id.clone()).collect();
                order.insert(index as usize, payload.item.object_id.clone());
                order
            }),
            ..Default::default()
        }), ..Default::default() })
}
//#endregion 🔖️Diff
