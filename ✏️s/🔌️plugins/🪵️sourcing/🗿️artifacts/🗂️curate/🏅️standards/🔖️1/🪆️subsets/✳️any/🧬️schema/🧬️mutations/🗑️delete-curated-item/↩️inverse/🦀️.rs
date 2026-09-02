//! ↩️ Inverse for `DeleteCuratedItem` — reconstructs the removed `CuratedItem` from `base` (the
//! pre-state) as a `create-curated-item`; missing target ⇒ no-op.
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteCuratedItem, base: &CurateSnapshot) -> Vec<SourcingMutation> {
    match base.curated.iter().find(|item| item.object_id == payload.object_id) {
        Some(item) => vec![crate::artifacts::curate::mutations::create_curated_item::create_curated_item(item.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
