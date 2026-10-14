//! ↩️ Inverse for `MoveReference` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::MoveReference;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<MoveReference>>(payload: &super::mutation::MoveReference, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::move_reference(item.id.clone(), item.origin)]

    })())
}
//#endregion 🔖️Inverse
