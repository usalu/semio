//! ↩️ Inverse for `CreateCuratedItem` — always a `delete-curated-item` of the object id it curated
//! (the payload itself carries the id, so no BASE lookup is needed to know what to undo).
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateCuratedItem, _base: &CurationSnapshot) -> Vec<SourcingMutation> {
    vec![crate::mutations::delete_curated_item::delete_curated_item(payload.item.object_id.clone())]
}
//#endregion 🔖️Inverse
