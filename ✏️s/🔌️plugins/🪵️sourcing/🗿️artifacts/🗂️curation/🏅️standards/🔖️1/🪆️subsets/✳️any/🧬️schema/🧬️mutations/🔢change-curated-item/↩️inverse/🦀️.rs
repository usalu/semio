//! ↩️ Inverse for `ChangeCuratedItemCount` — restores the OLD count read from `base` (the
//! pre-state), never inverted structurally from the diff; missing target ⇒ no-op.
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeCuratedItemCount, base: &CurationSnapshot) -> Vec<SourcingMutation> {
    match base.curated.iter().find(|item| item.object_id == payload.object_id) {
        Some(item) => vec![crate::mutations::change_curated_item_count::change_curated_item_count(payload.object_id.clone(), item.count)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
