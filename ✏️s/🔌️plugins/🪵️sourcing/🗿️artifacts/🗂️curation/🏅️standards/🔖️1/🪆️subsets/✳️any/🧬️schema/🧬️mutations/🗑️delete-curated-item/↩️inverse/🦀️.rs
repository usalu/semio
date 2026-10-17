//! ↩️ Inverse for `DeleteCuratedItem` — reconstructs the removed `CuratedItem` from `base` (the
//! pre-state) as a `create-curated-item` at its original index; missing target ⇒ no-op.
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DeleteCuratedItem, base: &CurationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_curated_item::CreateCuratedItem>,
{
    Ok((|| {
    match base.curated.iter().position(|item| item.object_id == payload.object_id) {
        Some(at) => vec![crate::mutations::create_curated_item::create_curated_item_at(base.curated[at].clone(), at as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
