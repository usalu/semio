//! ↩️ Inverse for `CreateCuratedItem` — always a `delete-curated-item` of the object id it curated
//! (the payload itself carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateCuratedItem, _base: &CurateSnapshot) -> Vec<SourcingMutation> {
    vec![crate::artifacts::curate::mutations::delete_curated_item::mutation::delete_curated_item(payload.item.object_id.clone())]
}
//#endregion 🔖️Inverse
