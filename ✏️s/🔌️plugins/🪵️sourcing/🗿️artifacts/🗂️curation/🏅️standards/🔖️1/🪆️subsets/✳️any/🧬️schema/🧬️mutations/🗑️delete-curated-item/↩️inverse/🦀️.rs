//! ↩️ Inverse for `DeleteCuratedItem` — reconstructs the removed `CuratedItem` from `base` (the
//! pre-state) as a `create-curated-item`; missing target ⇒ no-op.
use crate::artifacts::curation::mutations::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteCuratedItem, base: &CurationSnapshot) -> Vec<SourcingMutation> {
    match base.curated.iter().find(|item| item.object_id == payload.object_id) {
        Some(item) => vec![crate::artifacts::curation::mutations::create_curated_item::create_curated_item(item.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
