//! ↩️ Inverse for `CreateCuratedItem` — always a `delete-curated-item` of the object id it curated
//! (the payload itself carries the id, so no BASE lookup is needed to know what to undo).
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::CreateCuratedItem, _base: &CurationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_curated_item::DeleteCuratedItem>,
{
    Ok((|| {
    vec![crate::mutations::delete_curated_item::delete_curated_item(payload.item.object_id.clone())]

    })())
}
//#endregion 🔖️Inverse
