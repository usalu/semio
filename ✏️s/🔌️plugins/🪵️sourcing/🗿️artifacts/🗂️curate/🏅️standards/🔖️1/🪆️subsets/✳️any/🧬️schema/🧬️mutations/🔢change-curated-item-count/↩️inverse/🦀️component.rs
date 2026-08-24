//! ↩️ Inverse for `ChangeCuratedItemCount` — restores the OLD count read from `base` (the
//! pre-state), never inverted structurally from the diff; missing target ⇒ no-op.
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeCuratedItemCount, base: &CurateSnapshot) -> Vec<SourcingMutation> {
    match base.curated.iter().find(|item| item.object_id == payload.object_id) {
        Some(item) => vec![crate::artifacts::curate::mutations::change_curated_item_count::mutation::change_curated_item_count(payload.object_id.clone(), item.count)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
